#![allow(non_snake_case)]
// use crate::data_type::SingleGrid;
use crate::interplate;
use crate::transforms;
use crate::MetError;
use crate::{RadarData, ToGrids};
use common_data::SingleGrid;
// use rayon::prelude::*;
use std::collections::HashMap;
use std::default::Default;

#[derive(Debug)]
pub struct RadialData {
    pub _extents: (f32, f32, f32, f32),
    pub lon: f32,
    pub lat: f32,
    pub height: f32,
    pub props: HashMap<String, String>,
    pub start_date: String,
    pub start_time: String,
    pub elements: Vec<String>, //物理量名称  Z,uZ,V,W等等
    // pub end_time: String,
    pub eles: Vec<f32>,                //所有的仰角
    pub azs: Vec<Vec<f32>>,            //每个仰角对应的方位角
    pub rs: Vec<Vec<Vec<f64>>>,        //仰角->方位角->斜距   米为单位
    pub data: Vec<Vec<Vec<Vec<f32>>>>, //物理量->仰角->方位角->值
}

impl RadialData {
    //取得物理量对应的数据的索引值
    pub fn get_element_idx(&self, ename: &str) -> Option<usize> {
        for (i, e) in self.elements.iter().enumerate() {
            if e == ename {
                return Some(i);
            }
        }
        None
    }
    pub fn get_ele_idx(&self, ele: f32) -> Option<usize> {
        for (i, v) in self.eles.iter().enumerate() {
            if *v == ele {
                return Some(i);
            }
        }
        None
    }

    pub fn set_extents(&mut self, xstart: f32, xend: f32, ystart: f32, yend: f32) {
        self._extents = (xstart, xend, ystart, yend);
    }

    pub fn ppi_to_grid(
        &self,
        ele: f32,       //仰角
        _element: &str, //物理量
        // xstart: f32,
        // xend: f32,
        // ystart: f32,
        // yend: f32,
        res: f32,
        // h: f32,
    ) -> Option<(usize, usize, Vec<f32>)> {
        let xstart = self._extents.0;
        let xend = self._extents.1;
        let ystart = self._extents.2;
        let yend = self._extents.3;

        let element_idx = self.get_element_idx(_element);
        if element_idx.is_none() {
            return None;
        }
        let element_idx = element_idx.unwrap();

        let idx = self.get_ele_idx(ele);
        if idx.is_none() {
            println!("{} not found ", ele);
            return None;
        }
        let ele_idx = idx.unwrap();

        let cols = ((xend - xstart) / res) as usize + 1;
        let rows = ((yend - ystart) / res) as usize + 1;
        let mut x: Vec<f32> = Vec::with_capacity(cols * 4);
        let mut y: Vec<f32> = Vec::with_capacity(rows * 4);
        for c in 0..=cols {
            let xv = xstart + res * c as f32;
            x.push(xv);
        }
        for r in 0..=rows {
            let yv = ystart + res * r as f32;
            y.push(yv);
        }
        let total_num = (cols + 1) * (rows + 1);
        let mut grid_value: Vec<f32> = vec![crate::MISSING; total_num];
        let elv_values = &self.data[element_idx][ele_idx];
        grid_value.iter_mut().enumerate().for_each(|(i, d)| {
            let yi = i / (cols + 1);
            let xi = i % (rows + 1);
            let yv = y[yi];
            let xv = x[xi];

            let (az, rang, _z) = transforms::cartesian_to_antenna_cwr(xv, -yv, ele, self.height);
            let elv_azs = &self.azs[ele_idx];
            let az = az.to_degrees();
            let (az_idx, az_idx1) = find_az_index(elv_azs, az);
            let elv_rs = &self.rs[ele_idx][az_idx];
            if let Some(range_idx) = find_range_index(elv_rs, rang as f64) {
                let az0 = elv_azs[az_idx];
                let az1 = elv_azs[az_idx1];
                let rang0 = elv_rs[range_idx];
                let rang1 = elv_rs[range_idx + 1];
                let v00 = elv_values[az_idx][range_idx];
                let v01 = elv_values[az_idx][range_idx + 1];
                let v10 = elv_values[az_idx1][range_idx];
                let v11 = elv_values[az_idx1][range_idx + 1];
                let v = interplate::interp_ppi(
                    az,
                    rang,
                    az0,
                    az1,
                    rang0 as f32,
                    rang1 as f32,
                    v00 as f32,
                    v01 as f32,
                    v10 as f32,
                    v11 as f32,
                );
                *d = v;
            }
        });
        Some((cols + 1, rows + 1, grid_value))
    }

}

fn find_az_index(azs: &Vec<f32>, az: f32) -> (usize, usize) {
    let az_len = azs.len();
    // println!("az_len {}",az_len);
    let first = azs[0];
    let last = azs[az_len - 1];
    let step = (last - first) / (az_len as f32 - 1.0);
    if az >= first {
        let idx1 = (az - first) / step;
        let idx1 = idx1 as usize;
        if idx1 >= az_len - 1 {
            return (az_len - 1, 0);
        } else {
            return (idx1, idx1 + 1);
        }
    } else {
        (0, 1)
    }
}

fn find_range_index(azs: &Vec<f64>, az: f64) -> Option<usize> {
    let az_len = azs.len();
    // println!("az_len {}",az_len);
    let first = azs[0];
    let last = azs[az_len - 1];
    let step = (last - first) / (az_len as f64 - 1.0);
    if az >= first && az <= last {
        let idx1 = (az - first) / step;
        let idx2 = idx1.floor();
        let idx3 = idx2 as usize;
        if idx3 >= az_len - 1 {
            println!("az {}  az_len {}", az, az_len);
            return Some(az_len - 2);
        } else {
            // println!("az {}  az_len {}", idx3, az_len);
            return Some(idx3);
        }
    }
    None
}

impl Default for RadialData {
    fn default() -> Self {
        Self {
            _extents: (-200000.0, 200000.0, -200000.0, 200000.0), //半径为200公里
            lon: 0.0f32,
            lat: 0f32,
            height: 0f32,
            props: HashMap::<String, String>::new(),
            start_date: String::from(""),
            start_time: String::from(""),
            elements: vec![],
            // end_time: String::from(""),
            eles: vec![0f32],                   //所有的仰角
            azs: vec![vec![0f32]],              //每个仰角对应的方位角
            rs: vec![vec![vec![0f64]]],         //仰角->方位角->斜距   米为单位
            data: vec![vec![vec![vec![0f32]]]], //物理量->仰角->方位角->值
        }
    }
}

impl RadarData for RadialData {
    fn site_name(&self) -> String {
        if let Some(station) = self.props.get("station") {
            station.to_string()
        } else {
            String::from("unknown")
        }
    }

    fn product(&self) -> String {
        if self.props.contains_key("province") && self.props.contains_key("area") {
            format!("{}/{}", &self.props["province"], &self.props["area"])
        } else if self.props.contains_key("product") {
            self.props["product"].clone()
        } else {
            String::new()
        }
    }
    fn start_date(&self) -> String {
        self.start_date.clone()
    }

    fn start_time(&self) -> String {
        self.start_time.clone()
    }

    fn ground_height(&self) -> f32 {
        self.height
    }

    fn bin_length(&self) -> f32 {
        todo!()
    }

    fn extents(&self) -> (f32, f32, f32, f32) {
        self._extents
    }

    fn center_lon_lat(&self) -> Option<(f32, f32)> {
        Some((self.lon, self.lat))
    }

    fn get_nearest_4v(
        &self,
        element: &str,
        elv: f32,
        az: f32,
        rang: f32,
    ) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
        let element_idx = self.get_element_idx(element);
        if element_idx.is_none() {
            return (
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
            );
        }
        let element_idx = element_idx.unwrap();
        let idx = self.get_ele_idx(elv);
        if idx.is_none() {
            println!("{} not found ", elv);
            return (
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
                crate::MISSING,
            );
        }
        let ele_idx = idx.unwrap();

        let elv_values = &self.data[element_idx][ele_idx];
        let elv_azs = &self.azs[ele_idx];
        // let az = az.to_degrees();
        //找出临近方位角的索引
        let (az_idx, az_idx1) = find_az_index(elv_azs, az);
        let elv_rs = &self.rs[ele_idx][az_idx];
        //找出临近库的索引
        if let Some(range_idx) = find_range_index(elv_rs, rang as f64) {
            let az0 = elv_azs[az_idx];
            let mut az1 = elv_azs[az_idx1];
            //hack
            if az0 == az1 {
                az1 = 0.0001;
            }
            let rang0 = elv_rs[range_idx];
            let rang1 = elv_rs[range_idx + 1];
            let v00 = elv_values[az_idx][range_idx];
            let v01 = elv_values[az_idx][range_idx + 1];
            let v10 = elv_values[az_idx1][range_idx];
            let v11 = elv_values[az_idx1][range_idx + 1];
            return (v00, v01, v10, v11, az0, az1, rang0 as f32, rang1 as f32);
        }

        return (
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
            crate::MISSING,
        );
    }

    fn get_nearest_el(&self, el: f32) -> Option<Vec<f32>> {
        if el.is_nan() {
            return None;
        }
        let els = &self.eles;
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

impl ToGrids for RadialData {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let mut grids = Vec::new();
        for elv in self.eles.iter() {
            for element in self.elements.iter() {
                if let Some(grid) = self.ppi_to_grid_lonlat(*elv, element) {
                    grids.push(grid);
                }
            }
        }
        Some(grids)
    }
}
