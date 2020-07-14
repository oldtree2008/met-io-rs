use crate::transforms;
use crate::data_type::SingleGrid;
use rayon::prelude::*;

#[derive(Debug)]
pub struct RadialData {
    pub eles: Vec<f32>,           //所有的仰角
    pub azs: Vec<Vec<f32>>,       //每个仰角对应的方位角
    pub rs: Vec<Vec<Vec<f64>>>,   //仰角->方位角->斜距   米为单位
    pub data: Vec<Vec<Vec<f32>>>, //仰角->方位角->值
}

impl RadialData {
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
        ele: f32,
        xstart: f32,
        xend: f32,
        ystart: f32,
        yend: f32,
        res: f32,
        h: f32,
    ) -> Option<(usize, usize, Vec<f32>)> {
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
        let mut grid_value: Vec<f32> = vec![999.0; total_num];
        let elv_values = &self.data[ele_idx];
        grid_value.par_iter_mut().enumerate().for_each(|(i, d)| {
            let yi = i / (cols + 1);
            let xi = i % (rows + 1);
            let yv = y[yi];
            let xv = x[xi];

            let (az, rang, _z) = transforms::cartesian_to_antenna_cwr(xv, -yv, ele, h);
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
                let v = interp_ppi(
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
        ele: f32,
        xstart: f32,
        xend: f32,
        ystart: f32,
        yend: f32,       
        h: f32,
    ) -> Option<SingleGrid> {
        let idx = self.get_ele_idx(ele);
        if idx.is_none() {
            println!("{} not found ", ele);
            return None;
        }
        let ele_idx = idx.unwrap();
        let cols = 1000;
        let rows = 1000;
        let ((lon1,lat1,lon2,lat2),(steplon,steplat)) = create_grid_extent(xstart, ystart, xend, yend, 114.0, 40.0, 1000, 1000);
        
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
        let mut grid_value: Vec<f32> = vec![999.0; total_num];
        let elv_values = &self.data[ele_idx];
        grid_value.par_iter_mut().enumerate().for_each(|(i, d)| {
            let yi = i / (cols + 1);
            let xi = i % (rows + 1);
            let lat = lats[yi];
            let lon = lons[xi];
            let (xv,yv) = transforms::geographic_to_cartesian_aeqd(lon,lat,114.0,40.0);
            let (az, rang, _z) = transforms::cartesian_to_antenna_cwr(xv, yv, ele, h);
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
                let v = interp_ppi(
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
        let sgrid = SingleGrid {
            ni:(cols+1) as i64,
            nj:(rows+1) as i64,
            lat_gap:steplat as f64,
            lng_gap:steplon as f64,
            start_lat:lat1 as f64,
            start_lng:lon1 as f64,
            end_lat:lat2 as f64,
            end_lng:lon2 as f64,
            level:0,
            element:String::from("R"),
            values:grid_value,
        };
        Some(sgrid)
    }
}
fn create_grid_extent(x1:f32,y1:f32,x2:f32,y2:f32,lon0:f32,lat0:f32,row:usize,col:usize)->((f32,f32,f32,f32),(f32,f32)) {
    let (lon1,lat1) = transforms::cartesian_to_geographic_aeqd(x1,y1,lon0,lat0);
    let (lon2,lat2) = transforms::cartesian_to_geographic_aeqd(x2,y2,lon0,lat0) ;
    let steplon = (lon2-lon1)/(col-1) as f32;
    let steplat = (lat2-lat1)/(row -1) as f32;
    let lon2 = lon1 + (col-1) as f32 * steplon;
    let lat2 = lat1 + (row-1) as f32 * steplat;
    ((lon1,lat1,lon2,lat2),(steplon,steplat))
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
            return (259, 0);
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

pub fn interp_ppi(
    az: f32,
    r: f32,
    az_0: f32,
    az_1: f32,
    r_0: f32,
    r_1: f32,
    mat_00: f32,
    mat_01: f32,
    mat_10: f32,
    mat_11: f32,
) -> f32 {
    // 利用雷达扫描的周围四个点插值中间的点(az, r)
    // interp radar ppi scan data
    // az : target azimuth, units:degree
    // r : target range, units:meters
    // az_0 : grid start azimuth, units:degree
    // az_1 : grid end azimuth, units:degree
    // r_0 : grid start range , units : meters
    // r_1 : grid end range, units: meters
    // mat_00: data for [az_0, r_0]
    // mat_01: data for [az_0, r_1]
    // mat_10: data for [az_1, r_0]
    // mat_11: data for [az_1, r_1]
    // fillvalue: fillvalue for mat
    // return target value interped, units: like mat
    let interped;
    let fillvalue = 999.0;
    if ((mat_00 != fillvalue) && (mat_01 != fillvalue))
        && ((mat_10 != fillvalue) && (mat_11 != fillvalue))
    {
        interped = (mat_00 * (az_1 - az) * (r_1 - r)
            + mat_10 * (az - az_0) * (r_1 - r)
            + mat_01 * (az_1 - az) * (r - r_0)
            + mat_11 * (az - az_0) * (r - r_0))
            / (r_1 - r_0)
            / (az_1 - az_0);
    } else if (mat_00 != fillvalue) && (mat_01 != fillvalue) {
        interped = (mat_00 * (r_1 - r) + mat_01 * (r - r_0)) / (r_1 - r_0);
    } else if (mat_10 != fillvalue) && (mat_11 != fillvalue) {
        interped = (mat_10 * (r_1 - r) + mat_11 * (r - r_0)) / (r_1 - r_0);
    } else if (mat_00 != fillvalue) && (mat_10 != fillvalue) {
        interped = (mat_00 * (az_1 - az) + mat_10 * (az - az_0)) / (az_1 - az_0);
    } else if (mat_01 != fillvalue) && (mat_11 != fillvalue) {
        interped = (mat_01 * (az_1 - az) + mat_11 * (az - az_0)) / (az_1 - az_0);
    } else {
        interped = fillvalue;
    }
    interped
}
