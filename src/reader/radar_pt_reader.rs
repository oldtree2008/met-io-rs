use crate::error::MetError;
use crate::{SingleGrid, ToGrids};
use binread::prelude::*;
use std::fs::File;
use std::io::{Cursor, Read};
use std::result::Result;

/**
 * 拼图数据基本信息。[999]字节。
 */
#[derive(Debug, BinRead)]
struct PTInfo {
    radar_count: u8, //[1]组网探测和拼图雷达个数
    #[br(count = 150)]
    station_ids: Vec<u8>, //[5×30]组网探测和拼图雷达站号，依站号从小到大排列，每个站号占5个字节
    #[br(count = 600)]
    radar_types: Vec<u8>, //[20×30]组网探测和拼图雷达型号，依站号从小到大排列，每个型号占20个字节
    #[br(count = 30)]
    rada_xs: Vec<u32>, //[30]组网探测雷达位置经度（以0.001度为记数单位），
    //依站号从小到大排列
    #[br(count = 30)]
    rada_ys: Vec<u32>, //[30]组网探测雷达位置纬度（以0.001度为记数单位），
    //依站号从小到大排列
    year: i16, //[7]组网探测记录结束时间
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    sec: u8,
    time_p: u8, //[1]组网探测时间来源，以各测站相应值的最小值
    //0=计算机时钟，但一天内未进行对时
    //1=计算机时钟，一天内已进行对时
    calibration: u8, //[1]组网探测测站雷达标校状态，以各测站相应值的最小值记,
                     //0=无标校,1=一月内人工标校,2=一星期内人工标校, 3=自动标校
}
/**
 * 拼图数据产品参数信息。[36]字节。
 */
#[derive(Debug, BinRead)]
struct PTProduction {
    production: u16, //[2]拼图产品名称:1 =回波强度等高PPI
    heigh: i16,      //[2]等高PPI高度层公里数，计数单位0.1公里

    prj_type: u8, //[1]投影类型:1:等经纬度投影
    x_lu: i32,    //[4]左上角位置经度（以0.001度为记数单位），依站号从小到大排列
    y_lu: i32,    //[4]左上角位置纬度（以0.001度为记数单位），依站号从小到大排列
    res_x: u16,   //[2]水平网格距（以0.001度为记数单位）
    res_y: u16,   //[2]垂直网格距（以0.001度为记数单位）
    count_x: u32, //[4]水平格点数
    count_y: u32, //[4]垂直格点数
    ratio: i16,   //[2]数值放大倍数
    //[6]拼图数据产品生成时间
    year: i16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    hyperrefract: u8, //[1]超折射识别和剔除（0为否，1为是）
    obstruct: u8,     //[1]波束阻挡订正（0为否，1为是）
    attenuation: u8,  //[1]回波衰减订正（0为否，1为是）
}

pub struct RadarPTReader {
    pt_info: PTInfo,
    pt_product: PTProduction,
    datas: Vec<f32>,
}

impl RadarPTReader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        let mut f = File::open(fname)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        let mut cursor = Cursor::new(&buf);
        let pt_info: PTInfo = BinRead::read(&mut cursor)?;
        let pt_product: PTProduction = BinRead::read(&mut cursor)?;
        let count = (pt_product.count_x * pt_product.count_y) as usize;
        let is_grid = buf.len() == 1281 + count;

        dbg!(&pt_product);
        dbg!(is_grid);
        cursor.set_position(1281);

        let mut datas = vec![crate::MISSING; count];
        if is_grid {
            datas.iter_mut().enumerate().for_each(|(_, d)| {
                let b: u8 = BinRead::read(&mut cursor).unwrap();
                if b == 0 {
                    *d = 0 as f32;
                } else {
                    *d = (b - 64) as f32 * 0.5;
                }
            });
        } else {
            let width = pt_product.count_x as usize;
            let h: i16 = BinRead::read(&mut cursor)?;
            let count1: i32 = BinRead::read(&mut cursor)?;

            for _ in 0..count1 {
                let x: i32 = BinRead::read(&mut cursor)?;
                let y: i32 = BinRead::read(&mut cursor)?;
                let b: u8 = BinRead::read(&mut cursor).unwrap();
                let value = (b - 64) as f32 * 0.5;
                let n = x as usize + y as usize * width;
                if n < count {
                    datas[n] = value;
                }
            }
        }

        Ok(Self {
            pt_info,
            pt_product,
            datas,
        })
    }
}

impl ToGrids for RadarPTReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let prod = &self.pt_product;

        let ni = prod.count_x as i64;
        let nj = prod.count_y as i64;
        let lat_gap = prod.res_y as f64 * -0.001;
        let lng_gap = prod.res_x as f64 * 0.001;
        let start_lat = prod.y_lu as f64 * 0.001;
        let start_lng = prod.x_lu as f64 * 0.001;
        let end_lat = start_lat + (nj - 1) as f64 * lat_gap;
        let end_lng = start_lng + (ni - 1) as f64 * lng_gap;
        let heigh = prod.heigh as f32;
        let values = self.datas.clone();
        let product = if prod.production == 1 {
            String::from("回波强度等高PPI")
        } else {
            String::from("UNKNOW")
        };
        let data_date = format!("{:04}{:02}{:02}", prod.year, prod.month, prod.day);
        let data_time = format!("{:02}{:02}00", prod.hour, prod.minute);
        let project = if prod.prj_type == 1 {
            String::from("等经纬度投影")
        } else {
            String::new()
        };

        let data_des = format!("{}{}{}{}", data_date, data_time, project, product);
        let sgrid = SingleGrid::<_, f32> {
            ni, //列数，lon的个数
            nj,
            lat_gap,
            lng_gap,
            start_lat,
            start_lng,
            end_lat,
            end_lng,
            level: Some(heigh),
            element: String::from("Z"),
            values,
            data_date,
            data_time,        //时次
            forecast_time: 0, //时效
            center: String::new(),
            product,
            data_des,
        };
        Some(vec![sgrid])
    }
}
