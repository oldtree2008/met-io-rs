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
}

impl RadarData for STRadialData {
    fn site_name(&self) -> String {
        self.site_name.clone()
    }
    fn product(&self) -> String {
        String::from("standard")
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
