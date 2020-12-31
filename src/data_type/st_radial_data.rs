use crate::transforms;
use crate::ToGrids;
use crate::{interplate, nom::file_descriptor_proto};
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
    pub fn el2idx(&self, el: f32) -> Vec<i32> {
        self.idx_el
            .iter()
            .filter(|d| d.1 == el)
            .map(|d| d.0)
            .collect()
    }

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

    pub fn get_nearest_4v(
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
                idx_az.push((ia.0,ia.1, ia.2));
            }
            idx_az
                .as_mut_slice()
                .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            // println!("az {},  idx_az {:?}", az, idx_az);

            if idx_az.len() >= 2 {
                let az_idx0 = idx_az[0].0;
                let az_idx1 = idx_az[1].0;
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
    pub fn ppi_to_grid_lonlat(
        &self,
        ele: f32,      //仰角
        element: &str, //物理量
    ) -> Option<SingleGrid> {
        let xstart = self._extents.0;
        let xend = self._extents.1;
        let ystart = self._extents.2;
        let yend = self._extents.3;
        // let element_idx = self.get_element_idx(element);
        // if element_idx.is_none() {
        //     return None;
        // }
        // let element_idx = element_idx.unwrap();
        // let idx = self.get_ele_idx(ele);
        // if idx.is_none() {
        //     println!("{} not found ", ele);
        //     return None;
        // }
        // let ele_idx = idx.unwrap();
        let cols = 1024;
        let rows = 1024;
        let cols = 512;
        let rows = 512;
        // let cols = 256;
        // let rows = 256;
        let lon0 = self.longtitude;
        let lat0 = self.latitude;

        let ((lon1, lat1, lon2, lat2), (steplon, steplat)) =
            create_grid_extent(xstart, ystart, xend, yend, lon0, lat0, rows, cols);

        let mut lons: Vec<f32> = Vec::with_capacity(cols * 4);
        let mut lats: Vec<f32> = Vec::with_capacity(rows * 4);
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

        grid_value.iter_mut().enumerate().for_each(|(i, d)| {
            let yi = i / (cols + 1);
            let xi = i % (rows + 1);
            let lat = lats[yi];
            let lon = lons[xi];
            let (xv, yv) = transforms::geographic_to_cartesian_aeqd(lon, lat, lon0, lat0);
            let height = self.ground_height;
            let (az, rang, _z) = transforms::cartesian_to_antenna_cwr(xv, yv, ele, height as f32);
            // println!("lon {} lat {}   az {} rang {}", lon, lat, az, rang);
            let (v00, v01, v10, v11, az0, az1, rs0, rs1) =
                self.get_nearest_4v(element, ele, az, rang);

            //双线性插值
            let v = interplate::interp_ppi(
                az, rang, az0, az1, rs0 as f32, rs1 as f32, v00 as f32, v01 as f32, v10 as f32,
                v11 as f32,
            );
            if v.is_nan() {
                println!("is_nan");
            }

            // println!(
            //     "az {} rang {} az0 {} az1 {} rs0 {} rs1 {} v {}  v00 {} v01 {} v10 {} v11 {}",
            //     az, rang, az0, az1, rs0, rs1, v, v00, v01, v10, v11
            // );
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
            data_date: self.start_date.clone(), //年月日
            data_time: self.start_time.clone(), //时次   时分秒
            forecast_time: 0,                   //时效
            center: String::from("radar"),
            product: String::from("standard"),
            station: Some(self.site_name.clone()),
        };
        Some(sgrid)
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

fn create_grid_extent(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    lon0: f32,
    lat0: f32,
    row: usize,
    col: usize,
) -> ((f32, f32, f32, f32), (f32, f32)) {
    let (lon1, lat1) = transforms::cartesian_to_geographic_aeqd(x1, y1, lon0, lat0);
    let (lon2, lat2) = transforms::cartesian_to_geographic_aeqd(x2, y2, lon0, lat0);
    let steplon = (lon2 - lon1) / (col - 1) as f32;
    let steplat = (lat2 - lat1) / (row - 1) as f32;
    let lon2 = lon1 + (col - 1) as f32 * steplon;
    let lat2 = lat1 + (row - 1) as f32 * steplat;
    ((lon1, lat1, lon2, lat2), (steplon, steplat))
}
