use rayon::prelude::*;

mod app;
mod converter;
mod data_type;
mod error;
mod protos;
mod reader;
mod utils;

pub use converter::*;
pub use data_type::*;
use error::MetError;
// pub use hdf5_error::Hdf5Error;
pub use app::*;
pub use protos::nom;
pub use reader::*;
pub use utils::interplate;
pub use utils::kjlocationer;
pub use utils::mercator_trans;
pub use utils::transforms;

pub const MISSING: f32 = 9999.0; //无效值

use common_data::SingleGrid;

pub trait ToGrids {
    fn to_grids(&self) -> Option<Vec<SingleGrid>>;
}

pub trait RadarData {
    ///ppi产品
    fn ppi_to_grid_lonlat(
        &self,
        ele: f32,      //仰角
        element: &str, //物理量
    ) -> Option<SingleGrid>
    where
        Self: Sync,
    {
        // let xstart = self._extents.0;
        // let xend = self._extents.1;
        // let ystart = self._extents.2;
        // let yend = self._extents.3;
        let (xstart, xend, ystart, yend) = self.extents();

        let cols = 1024;
        let rows = 1024;
        // let cols = 1840;
        // let rows = 1840;
        let cols = 256;
        let rows = 256;

        let bin_num = (self.extents().0 / self.bin_length()).abs() as usize;

        dbg!(self.extents(),bin_num);

        let cols = bin_num;
        let rows = bin_num;

        let (lon0, lat0) = self.center_lon_lat().unwrap_or((0.0, 0.0));

        let ((lon1, lat1, lon2, lat2), (steplon, steplat)) =
            transforms::create_grid_extent(xstart, ystart, xend, yend, lon0, lat0, rows, cols);

        let mut lons: Vec<f32> = Vec::with_capacity(cols);
        let mut lats: Vec<f32> = Vec::with_capacity(rows);
        for c in 0..=cols {
            let lon = lon1 + steplon * c as f32;
            lons.push(lon);
        }
        for r in 0..=rows {
            let lat = lat1 + steplat * r as f32;
            lats.push(lat);
        }
        let total_num = (cols + 1) * (rows + 1);
        let mut grid_value: Vec<f32> = vec![crate::MISSING; total_num];
        // let elv_values = &self.data[element_idx][ele_idx];

        // grid_value.iter_mut().enumerate().for_each(|(i, d)| {
        grid_value.par_iter_mut().enumerate().for_each(|(i, d)| {
            let yi = i / (cols + 1);
            let xi = i % (rows + 1);
            let lat = lats[yi];
            let lon = lons[xi];
            let (xv, yv) = transforms::geographic_to_cartesian_aeqd(lon, lat, lon0, lat0);

            let height = self.ground_height();
            let (az, rang, _z) = transforms::cartesian_to_antenna_cwr(xv, yv, ele, height as f32);
            // println!(
            //     "lon {} lat {}   xv {} yv {}  az {} rang {}  lon0 {} lat0 {}",
            //     lon,
            //     lat,
            //     xv,
            //     yv,
            //     az.to_degrees(),
            //     rang,
            //     lon0,
            //     lat0
            // );
            //注意角度的单位
            let az = az.to_degrees();
            let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
                self.get_nearest_4v(element, ele, az, rang);

            //双线性插值
            let v = interplate::interp_ppi(
                az, rang, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32, v10 as f32,
                v11 as f32,
            );
            // if v.is_nan() {
            //     println!("is_nan");
            // }
            // if v >1000.0{
            //     println!(
            //         "az {} rang {} az0 {} az1 {} rs0 {} rs1 {} v {}  v00 {} v01 {} v10 {} v11 {}",
            //         az, rang, az0, az1, rs0, rs1, v, v00, v01, v10, v11
            //     );
            // }
            *d = v;
        });

        let sgrid = SingleGrid {
            ni: (cols + 1) as i64,
            nj: (rows + 1) as i64,
            lat_gap: steplat as f64,
            lng_gap: steplon as f64,
            start_lat: lat1 as f64,
            start_lng: lon1 as f64,
            end_lat: lat2 as f64,
            end_lng: lon2 as f64,
            level: Some(ele),
            element: String::from(element),
            values: grid_value,
            data_date: self.start_date(), //年月日
            data_time: self.start_time(), //时次   时分秒
            forecast_time: 0,             //时效
            center: String::from("radar"),
            product: self.product(),
            station: Some(self.site_name()),
        };
        Some(sgrid)
    }

    ///剖面产品
    fn get_vcs_data(
        &self,
        element: &str,
        start_point: &(f32, f32),
        end_point: &(f32, f32),
    ) -> Result<Vec<Vec<f32>>, MetError> {
        let (lon0, lat0) = self.center_lon_lat().unwrap_or((0.0, 0.0));

        let (start_x, start_y) =
            transforms::geographic_to_cartesian_aeqd(start_point.0, start_point.1, lon0, lat0);
        let (end_x, end_y) =
            transforms::geographic_to_cartesian_aeqd(end_point.0, end_point.1, lon0, lat0);

        let dist_h = 2.0 * 1000.0; //高度取 20 公里
        let num_h = ((dist_h / self.bin_length()).round() / 1.0) as usize;
        let mut z = vec![0.0; num_h];
        z.iter_mut().enumerate().for_each(|(i, d)| {
            *d = i as f32 * self.bin_length();
        });

        let dist_xy = ((end_x - start_x).powf(2.0) + (end_y - start_y).powf(2.0)).sqrt();
        let step = self.bin_length() as f32 * 1.0; // dist_xy / num_h as f32;
        let angle_xy = if end_x - start_x == 0.0 {
            90.0_f32.to_radians()
        } else {
            ((end_y - start_y) / (end_x - start_x)).atan()
        };
        let del_x = step * angle_xy.sin();
        let del_y = step * angle_xy.cos();
        let num_xy = ((dist_xy / step as f32).round() / 1.0) as usize;

        let mut data = Vec::with_capacity(num_h);
        for (zidx, zz) in z.iter().enumerate() {
            // println!("zz {}  num_xy {}  num_h {}", zz, num_xy, num_h);
            let mut r_value = vec![crate::MISSING; num_xy];
            for idx in 0..num_xy {
                let xx = start_x + idx as f32 * del_x;
                let yy = start_y + idx as f32 * del_y;
                let (az, rs, el) = transforms::cartesian_xyz_to_antenna(xx, yy, *zz, 0.0);
                // transforms::cartesian_xyz_to_antenna(xx, yy, *zz, self.ground_height as f32);

                let el = el.to_degrees();
                // println!(
                //     "zz {} xx {} yy {} el {} az {} rs {}",
                //     zz, xx, yy, el, az, rs
                // );
                if let Some(els) = self.get_nearest_el(el) {
                    let e0 = &els[0];
                    let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
                        self.get_nearest_4v(element, *e0, az, rs);
                    let v0 = interplate::interp_ppi(
                        az, rs, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32,
                        v10 as f32, v11 as f32,
                    );
                    let e1 = &els[1];
                    let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
                        self.get_nearest_4v(element, *e1, az, rs);
                    let v1 = interplate::interp_ppi(
                        az, rs, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32,
                        v10 as f32, v11 as f32,
                    );
                    let v = interplate::interp_azimuth(el, *e0, *e1, v0, v1);
                    // println!("xx {} yy {} zz {} e0 {} e1 {}  el {} az {} rs {} v0 {} v1 {} v {}",xx,yy,zz,e0,e1,el,az,rs,v0,v1,v);
                    r_value[idx] = v;
                }
            }
            // if !(r_value.iter().all(|&d| d == crate::MISSING) == true) {
            // println!("zz {}",zz);
            data.push(r_value);
            // }
        }
        Ok(data)
    }

    fn get_vol_data(&self, element: &str) -> Result<Vec<Vec<Vec<f32>>>, MetError> {
        let start_x = -100.0 * self.bin_length() as f32;
        let end_x = 100.0 * self.bin_length() as f32;
        let start_y = -100.0 * self.bin_length() as f32;
        let end_y = 100.0 * self.bin_length() as f32;

        let dist_h = 2.0 * 1000.0; //高度取 20 公里
        let num_h = ((dist_h / self.bin_length()).round() / 1.0) as usize;
        let mut z = vec![0.0; num_h];
        z.iter_mut().enumerate().for_each(|(i, d)| {
            *d = i as f32 * self.bin_length() as f32;
        });

        let step = self.bin_length() as f32 * 1.0; // dist_xy / num_h as f32;
        let num_x = ((end_x - start_x).abs() / step) as usize;
        let num_y = ((end_y - start_y).abs() / step) as usize;

        // println!(
        //     "extents {:?} num_x {} num_y {} num_h {}",
        //     self._extents, num_x, num_y, num_h
        // );
        // let mut x = vec![crate::MISSING;num_x];
        // let mut y = vec![crate::MISSING;num_y];

        let mut data = Vec::with_capacity(num_h);
        for (zidx, zz) in z.iter().enumerate() {
            // println!("zz {}  num_xy {}  num_h {}", zz, num_xy, num_h);
            let mut y_value = Vec::with_capacity(num_y);
            for y_idx in 0..num_y {
                let yy = start_y + y_idx as f32 * step;
                let mut x_value = vec![crate::MISSING; num_x];
                for x_idx in 0..num_x {
                    let xx = start_x + x_idx as f32 * step;
                    let (az, rs, el) = transforms::cartesian_xyz_to_antenna(xx, yy, *zz, 0.0);
                    // transforms::cartesian_xyz_to_antenna(xx, yy, *zz, self.ground_height as f32);
                    let el = el.to_degrees();
                    // println!(
                    //     "zz {} xx {} yy {} el {} az {} rs {}",
                    //     zz, xx, yy, el, az, rs
                    // );
                    if let Some(els) = self.get_nearest_el(el) {
                        let e0 = &els[0];
                        let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
                            self.get_nearest_4v(element, *e0, az, rs);
                        let v0 = interplate::interp_ppi(
                            az, rs, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32,
                            v10 as f32, v11 as f32,
                        );
                        let e1 = &els[1];
                        let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
                            self.get_nearest_4v(element, *e1, az, rs);
                        let v1 = interplate::interp_ppi(
                            az, rs, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32,
                            v10 as f32, v11 as f32,
                        );
                        let v = interplate::interp_azimuth(el, *e0, *e1, v0, v1);
                        // println!("xx {} yy {} zz {} e0 {} e1 {}  el {} az {} rs {} v0 {} v1 {} v {}",xx,yy,zz,e0,e1,el,az,rs,v0,v1,v);
                        x_value[x_idx] = v;
                    }
                }
                y_value.push(x_value);
            }
            data.push(y_value);
        }
        Ok(data)
    }

    fn site_name(&self) -> String;
    fn start_date(&self) -> String;
    fn start_time(&self) -> String;
    fn ground_height(&self) -> f32;
    //库长
    fn bin_length(&self) -> f32;
    fn product(&self) -> String;
    /// (xstart, xend, ystart, yend)
    fn extents(&self) -> (f32, f32, f32, f32);
    fn center_lon_lat(&self) -> Option<(f32, f32)>;
    fn get_nearest_4v(
        &self,
        element: &str,
        elv: f32,
        az: f32,
        rs: f32,
    ) -> (f32, f32, f32, f32, f32, f32, f32, f32);
    fn get_nearest_el(&self, el: f32) -> Option<Vec<f32>>;
}
