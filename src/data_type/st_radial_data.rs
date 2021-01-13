use crate::transforms;
use crate::MetError;
use crate::{interplate, nom::file_descriptor_proto};
use crate::{RadarData, ToGrids};
use common_data::SingleGrid;
///标准雷达数据结构
use std::collections::HashMap;

#[derive(Debug)]
pub struct STRadialData {
    pub _extents: (f32, f32, f32, f32),
    pub start_date: String,
    pub start_time: String,
    pub site_code: String,
    pub site_name: String,
    pub latitude: f32,
    pub longtitude: f32,
    pub antena_height: i32,
    pub ground_height: i32,
    pub log_res: i32,
    pub dop_res: i32,
    pub idx_el: Vec<(i32, f32)>,
    pub data: HashMap<i32, Vec<(f32, f32, HashMap<String, Vec<f32>>)>>,
}

impl STRadialData {
    //根据el值找对应的索引值。因为有同一仰角可能有多次扫描。
    pub fn el2idx(&self, el: f32) -> Vec<i32> {
        self.idx_el
            .iter()
            .filter(|d| d.1 == el)
            .map(|d| d.0)
            .collect()
    }

    // fn center_lon_lat(&self)->Option<(f32,f32)> {
    //     Some((self.longtitude,self.latitude))
    // }
    /// datas 为某一扫描层数据
    fn find_az_index(
        datas: &Vec<(f32, f32, HashMap<String, Vec<f32>>)>,
        element: &str,
        az: f32,
    ) -> Vec<(usize, f32, f32)> {
        let mut az_dis = Vec::new();
        for (idx, (_, az_, dt_data)) in datas.iter().enumerate() {
            if dt_data.contains_key(element) {
                let dis = (az - az_).abs();
                //az索引，距离，az值
                az_dis.push((idx, dis, *az_));
            }
        }
        let mut az0: Vec<(usize, f32, f32)> = az_dis
            .iter()
            .filter(|d| d.1 <= 1.0)
            .map(|d| (d.0, d.1, d.2))
            .collect();
        az0.as_mut_slice()
            .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        az0
    }

    // pub fn get_nearest_4v(
    //     &self,
    //     element: &str,
    //     elv: f32,
    //     az: f32,
    //     rs: f32,
    // ) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
    //     let sweeps = self.el2idx(elv);

    //     for s in sweeps.iter() {
    //         let datas = &self.data[s];
    //         let mut idx_az = STRadialData::find_az_index(&datas, element, az);

    //         let mut idx_az1 = Vec::new();
    //         if idx_az.len() < 2 && az < 1.0 {
    //             idx_az1 = STRadialData::find_az_index(&datas, element, az + 359.0);
    //             // println!("{:?}",&idx_az1);
    //         }
    //         if idx_az.len() < 2 && az > 359.0 {
    //             idx_az1 = STRadialData::find_az_index(&datas, element, az - 359.0);
    //             // println!("{:?}",&idx_az1);
    //         }
    //         for ia in idx_az1.iter() {
    //             idx_az.push((ia.0, ia.1, ia.2));
    //         }
    //         idx_az
    //             .as_mut_slice()
    //             .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    //         // println!("az {},  idx_az {:?}", az, idx_az);

    //         if idx_az.len() >= 2 {
    //             //选择距离最近的连个方位角的数据。
    //             let az_idx0 = idx_az[0].0;
    //             let az_idx1 = idx_az[1].0;
    //             //计算库数最近的两个数据
    //             let rs_idx0 = rs / self.log_res as f32;
    //             let rs_idx0 = rs_idx0.floor() as i32;
    //             let rs_idx1 = rs_idx0 + 1;

    //             let az0 = idx_az[0].2;
    //             let az1 = idx_az[1].2;
    //             let rs0 = rs_idx0 as f32 * self.log_res as f32;
    //             let rs1 = rs_idx1 as f32 * self.log_res as f32;
    //             // if datas[az_idx0 as usize].2[element][rs_idx0 as usize] != crate::MISSING {
    //             //     println!(
    //             //         "az0 {} az1 {}  rs_idx0 {} rs1 {}   {}",
    //             //         az0,
    //             //         az1,
    //             //         rs_idx0,
    //             //         rs_idx1,
    //             //         datas[az_idx0 as usize].2[element][rs_idx0 as usize]
    //             //     );
    //             // }
    //             let v00 = datas[az_idx0 as usize].2[element]
    //                 .get(rs_idx0 as usize)
    //                 .unwrap_or(&crate::MISSING);
    //             let v01 = datas[az_idx0 as usize].2[element]
    //                 .get(rs_idx1 as usize)
    //                 .unwrap_or(&crate::MISSING);
    //             let v10 = datas[az_idx1 as usize].2[element]
    //                 .get(rs_idx0 as usize)
    //                 .unwrap_or(&crate::MISSING);
    //             let v11 = datas[az_idx1 as usize].2[element]
    //                 .get(rs_idx1 as usize)
    //                 .unwrap_or(&crate::MISSING);

    //             // if element == "dBT"
    //             //     && v00 != &crate::MISSING
    //             //     && v01 != &crate::MISSING
    //             //     && v10 != &crate::MISSING
    //             //     && v11 != &crate::MISSING
    //             // {
    //             //     println!("v00 {} v01 {}  v10 {} v11 {}", v00, v01, v10, v11);
    //             // }

    //             return (*v00, *v01, *v10, *v11, az0, az1, rs0, rs1);
    //         } else if idx_az.len() == 1 {
    //             // println!("found one az");
    //             let az_idx0 = idx_az[0].0;
    //             let rs_idx0 = rs / self.log_res as f32;
    //             let rs_idx0 = rs_idx0.floor() as i32;
    //             let rs_idx1 = rs_idx0 + 1;

    //             let az0 = idx_az[0].2;
    //             let az1 = crate::MISSING;
    //             let rs0 = rs_idx0 as f32 * self.log_res as f32;
    //             let rs1 = rs_idx1 as f32 * self.log_res as f32;
    //             // println!("az0 {} az1 {}  rs0 {} rs1 {}   {}", az0, az1, rs0, rs1, datas[az0 as usize].2[element].len());
    //             let v00 = datas[az_idx0 as usize].2[element]
    //                 .get(rs_idx0 as usize)
    //                 .unwrap_or(&crate::MISSING);
    //             let v01 = datas[az_idx0 as usize].2[element]
    //                 .get(rs_idx1 as usize)
    //                 .unwrap_or(&crate::MISSING);
    //             let v10 = &crate::MISSING;
    //             let v11 = &crate::MISSING;
    //             return (*v00, *v01, *v10, *v11, az0, az1, rs0, rs1);
    //         } else {
    //             // println!("not found az");
    //         }
    //     }
    //     (
    //         crate::MISSING,
    //         crate::MISSING,
    //         crate::MISSING,
    //         crate::MISSING,
    //         crate::MISSING,
    //         crate::MISSING,
    //         crate::MISSING,
    //         crate::MISSING,
    //     )
    // }
    pub fn get_elvs(&self) -> Vec<f32> {
        let mut elvs: Vec<f32> = self.idx_el.iter().map(|r| r.1).collect();
        elvs.as_mut_slice()
            .sort_by(|a, b| a.partial_cmp(&b).unwrap());
        elvs.dedup();
        elvs
    }

    pub fn get_elevate_element(&self) -> (Vec<f32>, Vec<Vec<String>>) {
        let mut els = Vec::new();
        let mut el_elements = Vec::new();
        for (idx, elv) in self.idx_el.iter() {
            let datas = &self.data[idx];
            let mut elements = Vec::new();
            let first_az = &datas[0];
            for k in first_az.2.keys() {
                elements.push(k.to_string());
            }
            els.push(*elv);
            el_elements.push(elements);
        }
        (els, el_elements)
    }
    // pub fn ppi_to_grid_lonlat(
    //     &self,
    //     ele: f32,      //仰角
    //     element: &str, //物理量
    // ) -> Option<SingleGrid> {
    //     let xstart = self._extents.0;
    //     let xend = self._extents.1;
    //     let ystart = self._extents.2;
    //     let yend = self._extents.3;
    //     // let element_idx = self.get_element_idx(element);
    //     // if element_idx.is_none() {
    //     //     return None;
    //     // }
    //     // let element_idx = element_idx.unwrap();
    //     // let idx = self.get_ele_idx(ele);
    //     // if idx.is_none() {
    //     //     println!("{} not found ", ele);
    //     //     return None;
    //     // }
    //     // let ele_idx = idx.unwrap();
    //     let cols = 1024;
    //     let rows = 1024;
    //     let cols = 1840;
    //     let rows = 1840;
    //     // let cols = 256;
    //     // let rows = 256;
    //     let lon0 = self.longtitude;
    //     let lat0 = self.latitude;

    //     let ((lon1, lat1, lon2, lat2), (steplon, steplat)) =
    //     transforms::create_grid_extent(xstart, ystart, xend, yend, lon0, lat0, rows, cols);

    //     let mut lons: Vec<f32> = Vec::with_capacity(cols);
    //     let mut lats: Vec<f32> = Vec::with_capacity(rows);
    //     for c in 0..=cols {
    //         let lon = lon1 + steplon * c as f32;
    //         lons.push(lon);
    //     }
    //     for r in 0..=rows {
    //         let lat = lat1 + steplat * r as f32;
    //         lats.push(lat);
    //     }
    //     let total_num = (cols + 1) * (rows + 1);
    //     let mut grid_value: Vec<f32> = vec![crate::MISSING; total_num];
    //     // let elv_values = &self.data[element_idx][ele_idx];

    //     grid_value.iter_mut().enumerate().for_each(|(i, d)| {
    //         let yi = i / (cols + 1);
    //         let xi = i % (rows + 1);
    //         let lat = lats[yi];
    //         let lon = lons[xi];
    //         let (xv, yv) = transforms::geographic_to_cartesian_aeqd(lon, lat, lon0, lat0);

    //         let height = self.ground_height;
    //         let (az, rang, _z) = transforms::cartesian_to_antenna_cwr(xv, yv, ele, height as f32);
    //         // println!(
    //         //     "lon {} lat {}   xv {} yv {}  az {} rang {}  lon0 {} lat0 {}",
    //         //     lon,
    //         //     lat,
    //         //     xv,
    //         //     yv,
    //         //     az.to_degrees(),
    //         //     rang,
    //         //     lon0,
    //         //     lat0
    //         // );
    //         //注意角度的单位
    //         let az = az.to_degrees();
    //         let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
    //             self.get_nearest_4v(element, ele, az, rang);

    //         //双线性插值
    //         let v = interplate::interp_ppi(
    //             az, rang, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32, v10 as f32,
    //             v11 as f32,
    //         );
    //         // if v.is_nan() {
    //         //     println!("is_nan");
    //         // }
    //         // if v != crate::MISSING {
    //         //     println!(
    //         //         "az {} rang {} az0 {} az1 {} rs0 {} rs1 {} v {}  v00 {} v01 {} v10 {} v11 {}",
    //         //         az, rang, az0, az1, rs0, rs1, v, v00, v01, v10, v11
    //         //     );
    //         // }
    //         *d = v;
    //     });

    //     let sgrid = SingleGrid {
    //         ni: (cols + 1) as i64,
    //         nj: (rows + 1) as i64,
    //         lat_gap: steplat as f64,
    //         lng_gap: steplon as f64,
    //         start_lat: lat1 as f64,
    //         start_lng: lon1 as f64,
    //         end_lat: lat2 as f64,
    //         end_lng: lon2 as f64,
    //         level: Some(ele),
    //         element: String::from(element),
    //         values: grid_value,
    //         data_date: self.start_date.clone(), //年月日
    //         data_time: self.start_time.clone(), //时次   时分秒
    //         forecast_time: 0,                   //时效
    //         center: String::from("radar"),
    //         product: String::from("standard"),
    //         station: Some(self.site_name.clone()),
    //     };
    //     Some(sgrid)
    // }
    // start_point 起始点经纬度  (lon,lat)
    // end_point 终止点经纬度   (lon,lat)
    // pub fn get_vcs_data(
    //     &self,
    //     element: &str,
    //     start_point: &(f32, f32),
    //     end_point: &(f32, f32),
    // ) -> Result<Vec<Vec<f32>>, MetError> {
    //     let (start_x, start_y) = transforms::geographic_to_cartesian_aeqd(
    //         start_point.0,
    //         start_point.1,
    //         self.longtitude,
    //         self.latitude,
    //     );
    //     let (end_x, end_y) = transforms::geographic_to_cartesian_aeqd(
    //         end_point.0,
    //         end_point.1,
    //         self.longtitude,
    //         self.latitude,
    //     );

    //     let dist_h = 2.0 * 1000.0; //高度取 20 公里
    //     let num_h = ((dist_h / self.log_res as f32).round() / 1.0) as usize;
    //     let mut z = vec![0.0; num_h];
    //     z.iter_mut().enumerate().for_each(|(i, d)| {
    //         *d = i as f32 * self.log_res as f32;
    //     });

    //     let dist_xy = ((end_x - start_x).powf(2.0) + (end_y - start_y).powf(2.0)).sqrt();
    //     let step = self.log_res as f32 * 1.0; // dist_xy / num_h as f32;
    //     let angle_xy = if end_x - start_x == 0.0 {
    //         90.0_f32.to_radians()
    //     } else {
    //         ((end_y - start_y) / (end_x - start_x)).atan()
    //     };
    //     let del_x = step * angle_xy.sin();
    //     let del_y = step * angle_xy.cos();
    //     let num_xy = ((dist_xy / step as f32).round() / 1.0) as usize;

    //     let mut data = Vec::with_capacity(num_h);
    //     for (zidx, zz) in z.iter().enumerate() {
    //         // println!("zz {}  num_xy {}  num_h {}", zz, num_xy, num_h);
    //         let mut r_value = vec![crate::MISSING; num_xy];
    //         for idx in 0..num_xy {
    //             let xx = start_x + idx as f32 * del_x;
    //             let yy = start_y + idx as f32 * del_y;
    //             let (az, rs, el) = transforms::cartesian_xyz_to_antenna(xx, yy, *zz, 0.0);
    //             // transforms::cartesian_xyz_to_antenna(xx, yy, *zz, self.ground_height as f32);

    //             let el = el.to_degrees();
    //             // println!(
    //             //     "zz {} xx {} yy {} el {} az {} rs {}",
    //             //     zz, xx, yy, el, az, rs
    //             // );
    //             if let Some(els) = self.get_nearest_el(el) {
    //                 let e0 = &els[0];
    //                 let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
    //                     self.get_nearest_4v(element, *e0, az, rs);
    //                 let v0 = interplate::interp_ppi(
    //                     az, rs, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32,
    //                     v10 as f32, v11 as f32,
    //                 );
    //                 let e1 = &els[1];
    //                 let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
    //                     self.get_nearest_4v(element, *e1, az, rs);
    //                 let v1 = interplate::interp_ppi(
    //                     az, rs, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32,
    //                     v10 as f32, v11 as f32,
    //                 );
    //                 let v = interplate::interp_azimuth(el, *e0, *e1, v0, v1);
    //                 // println!("xx {} yy {} zz {} e0 {} e1 {}  el {} az {} rs {} v0 {} v1 {} v {}",xx,yy,zz,e0,e1,el,az,rs,v0,v1,v);
    //                 r_value[idx] = v;
    //             }
    //         }
    //         // if !(r_value.iter().all(|&d| d == crate::MISSING) == true) {
    //         // println!("zz {}",zz);
    //         data.push(r_value);
    //         // }
    //     }
    //     Ok(data)
    // }

    // pub fn get_vol_data(&self, element: &str) -> Result<Vec<Vec<Vec<f32>>>, MetError> {
    //     let start_x = -100.0 * self.log_res as f32;
    //     let end_x = 100.0 * self.log_res as f32;
    //     let start_y = -100.0 * self.log_res as f32;
    //     let end_y = 100.0 * self.log_res as f32;

    //     let dist_h = 2.0 * 1000.0; //高度取 20 公里
    //     let num_h = ((dist_h / self.log_res as f32).round() / 1.0) as usize;
    //     let mut z = vec![0.0; num_h];
    //     z.iter_mut().enumerate().for_each(|(i, d)| {
    //         *d = i as f32 * self.log_res as f32;
    //     });

    //     let step = self.log_res as f32 * 1.0; // dist_xy / num_h as f32;
    //     let num_x = ((end_x - start_x).abs() / step) as usize;
    //     let num_y = ((end_y - start_y).abs() / step) as usize;

    //     println!(
    //         "extents {:?} num_x {} num_y {} num_h {}",
    //         self._extents, num_x, num_y, num_h
    //     );
    //     // let mut x = vec![crate::MISSING;num_x];
    //     // let mut y = vec![crate::MISSING;num_y];

    //     let mut data = Vec::with_capacity(num_h);
    //     for (zidx, zz) in z.iter().enumerate() {
    //         // println!("zz {}  num_xy {}  num_h {}", zz, num_xy, num_h);
    //         let mut y_value = Vec::with_capacity(num_y);
    //         for y_idx in 0..num_y {
    //             let yy = start_y + y_idx as f32 * step;
    //             let mut x_value = vec![crate::MISSING; num_x];
    //             for x_idx in 0..num_x {
    //                 let xx = start_x + x_idx as f32 * step;
    //                 let (az, rs, el) = transforms::cartesian_xyz_to_antenna(xx, yy, *zz, 0.0);
    //                 // transforms::cartesian_xyz_to_antenna(xx, yy, *zz, self.ground_height as f32);
    //                 let el = el.to_degrees();
    //                 // println!(
    //                 //     "zz {} xx {} yy {} el {} az {} rs {}",
    //                 //     zz, xx, yy, el, az, rs
    //                 // );
    //                 if let Some(els) = self.get_nearest_el(el) {
    //                     let e0 = &els[0];
    //                     let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
    //                         self.get_nearest_4v(element, *e0, az, rs);
    //                     let v0 = interplate::interp_ppi(
    //                         az, rs, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32,
    //                         v10 as f32, v11 as f32,
    //                     );
    //                     let e1 = &els[1];
    //                     let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
    //                         self.get_nearest_4v(element, *e1, az, rs);
    //                     let v1 = interplate::interp_ppi(
    //                         az, rs, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32,
    //                         v10 as f32, v11 as f32,
    //                     );
    //                     let v = interplate::interp_azimuth(el, *e0, *e1, v0, v1);
    //                     // println!("xx {} yy {} zz {} e0 {} e1 {}  el {} az {} rs {} v0 {} v1 {} v {}",xx,yy,zz,e0,e1,el,az,rs,v0,v1,v);
    //                     x_value[x_idx] = v;
    //                 }
    //             }
    //             y_value.push(x_value);
    //         }
    //         data.push(y_value);
    //     }
    //     Ok(data)
    // }

    // fn get_nearest_el(&self, el: f32) -> Option<Vec<f32>> {
    //     if el.is_nan() {
    //         return None;
    //     }
    //     let els = self.get_elvs();
    //     let min = els[0];
    //     let max = els[els.len() - 1];
    //     if el < min || el > max {
    //         return None;
    //     } else {
    //         let mut el_dis: Vec<(usize, f32, f32)> = Vec::new();
    //         for (i, e) in els.iter().enumerate() {
    //             el_dis.push((i, (*e - el).abs(), *e));
    //         }
    //         el_dis
    //             .as_mut_slice()
    //             .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    //         return Some(vec![el_dis[0].2, el_dis[1].2]);
    //     }
    // }
}

impl RadarData for STRadialData {
    fn site_name(&self) -> String {
        self.site_name.clone()
    }
    fn start_date(&self) -> String {
        self.start_date.clone()
    }
    fn start_time(&self) -> String {
        self.start_time.clone()
    }

    fn ground_height(&self) -> f32 {
        self.ground_height as f32
    }
    fn bin_length(&self) -> f32 {
        self.log_res as f32
    }

    fn extents(&self) -> (f32, f32, f32, f32) {
        self._extents
    }
    fn center_lon_lat(&self) -> Option<(f32, f32)> {
        Some((self.longtitude, self.latitude))
    }
    fn get_nearest_4v(
        &self,
        element: &str,
        elv: f32,
        az: f32,
        rs: f32,
    ) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
        let sweeps = self.el2idx(elv);

        for s in sweeps.iter() {
            let datas = &self.data[s];
            let mut idx_az = STRadialData::find_az_index(&datas, element, az);

            let mut idx_az1 = Vec::new();
            if idx_az.len() < 2 && az < 1.0 {
                idx_az1 = STRadialData::find_az_index(&datas, element, az + 359.0);
                // println!("{:?}",&idx_az1);
            }
            if idx_az.len() < 2 && az > 359.0 {
                idx_az1 = STRadialData::find_az_index(&datas, element, az - 359.0);
                // println!("{:?}",&idx_az1);
            }
            for ia in idx_az1.iter() {
                idx_az.push((ia.0, ia.1, ia.2));
            }
            idx_az
                .as_mut_slice()
                .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            // println!("az {},  idx_az {:?}", az, idx_az);

            if idx_az.len() >= 2 {
                //选择距离最近的连个方位角的数据。
                let az_idx0 = idx_az[0].0;
                let az_idx1 = idx_az[1].0;
                //计算库数最近的两个数据
                let rs_idx0 = rs / self.log_res as f32;
                let rs_idx0 = rs_idx0.floor() as i32;
                let rs_idx1 = rs_idx0 + 1;

                let az0 = idx_az[0].2;
                let az1 = idx_az[1].2;
                let rs0 = rs_idx0 as f32 * self.log_res as f32;
                let rs1 = rs_idx1 as f32 * self.log_res as f32;
                // if datas[az_idx0 as usize].2[element][rs_idx0 as usize] != crate::MISSING {
                //     println!(
                //         "az0 {} az1 {}  rs_idx0 {} rs1 {}   {}",
                //         az0,
                //         az1,
                //         rs_idx0,
                //         rs_idx1,
                //         datas[az_idx0 as usize].2[element][rs_idx0 as usize]
                //     );
                // }
                let v00 = datas[az_idx0 as usize].2[element]
                    .get(rs_idx0 as usize)
                    .unwrap_or(&crate::MISSING);
                let v01 = datas[az_idx0 as usize].2[element]
                    .get(rs_idx1 as usize)
                    .unwrap_or(&crate::MISSING);
                let v10 = datas[az_idx1 as usize].2[element]
                    .get(rs_idx0 as usize)
                    .unwrap_or(&crate::MISSING);
                let v11 = datas[az_idx1 as usize].2[element]
                    .get(rs_idx1 as usize)
                    .unwrap_or(&crate::MISSING);

                // if element == "dBT"
                //     && v00 != &crate::MISSING
                //     && v01 != &crate::MISSING
                //     && v10 != &crate::MISSING
                //     && v11 != &crate::MISSING
                // {
                //     println!("v00 {} v01 {}  v10 {} v11 {}", v00, v01, v10, v11);
                // }

                return (*v00, *v01, *v10, *v11, az0, az1, rs0, rs1);
            } else if idx_az.len() == 1 {
                // println!("found one az");
                let az_idx0 = idx_az[0].0;
                let rs_idx0 = rs / self.log_res as f32;
                let rs_idx0 = rs_idx0.floor() as i32;
                let rs_idx1 = rs_idx0 + 1;

                let az0 = idx_az[0].2;
                let az1 = crate::MISSING;
                let rs0 = rs_idx0 as f32 * self.log_res as f32;
                let rs1 = rs_idx1 as f32 * self.log_res as f32;
                // println!("az0 {} az1 {}  rs0 {} rs1 {}   {}", az0, az1, rs0, rs1, datas[az0 as usize].2[element].len());
                let v00 = datas[az_idx0 as usize].2[element]
                    .get(rs_idx0 as usize)
                    .unwrap_or(&crate::MISSING);
                let v01 = datas[az_idx0 as usize].2[element]
                    .get(rs_idx1 as usize)
                    .unwrap_or(&crate::MISSING);
                let v10 = &crate::MISSING;
                let v11 = &crate::MISSING;
                return (*v00, *v01, *v10, *v11, az0, az1, rs0, rs1);
            } else {
                // println!("not found az");
            }
        }
        (
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
        )
    }
    fn get_nearest_el(&self, el: f32) -> Option<Vec<f32>> {
        if el.is_nan() {
            return None;
        }
        let els = self.get_elvs();
        let min = els[0];
        let max = els[els.len() - 1];
        if el < min || el > max {
            return None;
        } else {
            let mut el_dis: Vec<(usize, f32, f32)> = Vec::new();
            for (i, e) in els.iter().enumerate() {
                el_dis.push((i, (*e - el).abs(), *e));
            }
            el_dis
                .as_mut_slice()
                .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            return Some(vec![el_dis[0].2, el_dis[1].2]);
        }
    }
}

impl ToGrids for STRadialData {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let mut grids = Vec::new();
        let (elvs, elements) = self.get_elevate_element();
        for (idx, el) in elvs.iter().enumerate() {
            let el_elements = &elements[idx];
            for eleme in el_elements.iter() {
                if let Some(grid) = self.ppi_to_grid_lonlat(*el, eleme) {
                    println!("el {} element {}", el, eleme);
                    grids.push(grid);
                }
            }
        }
        Some(grids)
    }
}

// fn create_grid_extent(
//     x1: f32,
//     y1: f32,
//     x2: f32,
//     y2: f32,
//     lon0: f32,
//     lat0: f32,
//     row: usize,
//     col: usize,
// ) -> ((f32, f32, f32, f32), (f32, f32)) {
//     let (lon1, lat1) = transforms::cartesian_to_geographic_aeqd(x1, y1, lon0, lat0);
//     let (lon2, lat2) = transforms::cartesian_to_geographic_aeqd(x2, y2, lon0, lat0);
//     let steplon = (lon2 - lon1) / (col - 1) as f32;
//     let steplat = (lat2 - lat1) / (row - 1) as f32;
//     let lon2 = lon1 + (col - 1) as f32 * steplon;
//     let lat2 = lat1 + (row - 1) as f32 * steplat;
//     ((lon1, lat1, lon2, lat2), (steplon, steplat))
// }
