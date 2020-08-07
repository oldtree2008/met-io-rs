#![allow(non_snake_case)]
use crate::data_type::SingleGrid;
use crate::interplate;
use crate::transforms;
use rayon::prelude::*;
use std::collections::HashMap;
use std::default::Default;

#[derive(Debug)]
pub struct RadialData {
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
    pub fn ppi_to_grid(
        &self,
        ele: f32,       //仰角
        _element: &str, //物理量
        xstart: f32,
        xend: f32,
        ystart: f32,
        yend: f32,
        res: f32,
        // h: f32,
    ) -> Option<(usize, usize, Vec<f32>)> {
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
        grid_value.par_iter_mut().enumerate().for_each(|(i, d)| {
            let yi = i / (cols + 1);
            let xi = i % (rows + 1);
            let yv = y[yi];
            let xv = x[xi];

            let (az, rang, _z) = transforms::cartesian_to_antenna_cwr(xv, -yv, ele, self.height);
            let elv_azs = &self.azs[ele_idx];
            let az = az.to_degrees();
            let (az_idx, az_idx1) = find_index(elv_azs, az);
            let elv_rs = &self.rs[ele_idx][az_idx];
            if let Some(range_idx) = find_index1(elv_rs, rang as f64) {
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

    pub fn ppi_to_grid_lonlat(
        &self,
        ele: f32,      //仰角
        element: &str, //物理量
        xstart: f32,
        xend: f32,
        ystart: f32,
        yend: f32,
        // h: f32,
    ) -> Option<SingleGrid> {
        let element_idx = self.get_element_idx(element);
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
        let cols = 1000;
        let rows = 1000;

        let lon0 = self.lon;
        let lat0 = self.lat;

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
        let elv_values = &self.data[element_idx][ele_idx];

        grid_value.par_iter_mut().enumerate().for_each(|(i, d)| {
            let yi = i / (cols + 1);
            let xi = i % (rows + 1);
            let lat = lats[yi];
            let lon = lons[xi];
            let (xv, yv) = transforms::geographic_to_cartesian_aeqd(lon, lat, lon0, lat0);
            let (az, rang, _z) = transforms::cartesian_to_antenna_cwr(xv, yv, ele, self.height);
            let elv_azs = &self.azs[ele_idx];
            let az = az.to_degrees();
            let (az_idx, az_idx1) = find_index(elv_azs, az);
            let elv_rs = &self.rs[ele_idx][az_idx];
            if let Some(range_idx) = find_index1(elv_rs, rang as f64) {
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
        let product = if self.props.contains_key("province") && self.props.contains_key("area") {
            format!("{}/{}", &self.props["province"], &self.props["area"])
        } else {
            String::new()
        };
        let data_des = format!("{}{}{}", &self.start_date, &self.start_time, product);
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
            product,
            data_des,
        };
        Some(sgrid)
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

fn find_index(azs: &Vec<f32>, az: f32) -> (usize, usize) {
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

fn find_index1(azs: &Vec<f64>, az: f64) -> Option<usize> {
    let az_len = azs.len();
    // println!("az_len {}",az_len);
    let first = azs[0];
    let last = azs[az_len - 1];
    let step = (last - first) / (az_len as f64 - 1.0);
    if az >= first && az <= last {
        let idx1 = (az - first) / step;
        let idx2 = idx1.floor();
        let idx3 = idx2 as usize;
        if idx3 > az_len {
            println!("az {}  az_len {}", az, az_len);
            return Some(az_len - 1);
        } else {
            return Some(idx3);
        }
    }
    None
}

impl Default for RadialData {
    fn default() -> Self {
        Self {
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
