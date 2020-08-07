use crate::ToGrids;
use crate::error::MetError;
use crate::kjlocationer::KJLocationer;
use crate::SingleGrid;
use binread::prelude::*;
use std::fs::File;
use std::io::{Cursor, Read, SeekFrom};
use std::result::Result;

#[derive(Debug, BinRead)]
#[br(little)]
pub struct KJSatReader {
    pub org: i16,        //固定值1
    pub info_type: i16,  //资料类型
    pub attr: i16,       //投影方式
    pub startline: i16,  //起始行号
    pub endline: i16,    //终止行号
    pub startcol: i16,   //起始列号
    pub endcol: i16,     //终止列号
    pub centerloni: i16, //中央经度
    pub centerlati: i16, //中央纬度
    pub sample: i16,     //采样率
    pub height: i16,     //高度
    pub width: i16,      //宽度
    pub east: i16,
    pub west: i16,
    pub south: i16,
    pub north: i16,
    pub y: i16,
    pub x: i16,
    pub version: f32,
    #[br(count = 1250)]
    pub grid: Vec<i16>,
    #[br(count = 9)]
    pub status1: Vec<i16>,
    #[br(count = 6)]
    pub time: Vec<u8>, //资料日期
    #[br(count = 51)]
    pub status2: Vec<i16>, //卫星状态2
    #[br(count = 32)]
    pub mapping: Vec<i16>, //常数块
    #[br(count = 1600)]
    pub attitude: Vec<i16>, //轨道姿态数据块
    pub res: f64, //分辨率
    #[br(count = 103)]
    pub blank: Vec<i16>, //空格
    #[br(count = 256)]
    pub table: Vec<f32>, //定标数据

    #[br(calc=height as usize * width as usize)]
    pub data_size: usize,
    #[br(seek_before=SeekFrom::Start(7168),count=data_size)]
    pub values: Vec<u8>,
}

impl KJSatReader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        let mut f = File::open(fname)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        let mut cursor = Cursor::new(&buf);
        let reader: KJSatReader = BinRead::read(&mut cursor)?;
        dbg!(reader.width, reader.height, reader.values.len());
        Ok(reader)
    }

    pub fn data_date_time(&self) -> (String, String) {
        let mut datetime = Vec::new();
        for d in self.time.iter() {
            let s = format!("{:X}", d);
            let s: u16 = s.parse().unwrap();
            let s = format!("{:02}", s);
            datetime.push(s);
        }
        let date = datetime[0..4].join("");
        let time = datetime[4..6].join("");
        let time = format!("{}00", time); //添加秒
        (date, time)
    }

    pub fn data_prod_ele(&self) -> (String, String) {
        let mut prod: String;
        let mut ele: String;
        let info = &self.info_type;
        if *info < 10 {
            prod = String::from("GMS4");
            if *info == 0 {
                ele = String::from("IR1");
            } else if *info == 1 {
                ele = String::from("VIS");
            } else {
                ele = String::from("UNKNOWN");
            }
            return (prod, ele);
        }
        let infostr = format!("{}", info);
        let infostr_len = infostr.len();
        let (sat, el) = infostr.split_at(infostr_len - 1);
        let sat: i16 = sat.parse().unwrap();
        let el: i16 = el.parse().unwrap();
        // dbg!(sat, el);
        match sat {
            5 => prod = String::from("GMS5"),
            6 => prod = String::from("FY2B"),
            7 => prod = String::from("FY2C"),
            8 => prod = String::from("MTSAT1"),
            9 => prod = String::from("FY2D"),
            10 => prod = String::from("FY2E"),
            11 => prod = String::from("FY2F"),
            12 => prod = String::from("FY2G"),
            13 => prod = String::from("FY2H"),
            _ => prod = String::from("UNKNOWN"),
        };

        match el {
            1 => ele = String::from("IR1"),
            2 => ele = String::from("VIS"),
            3 => ele = String::from("IR2"),
            4 => ele = String::from("WV"),
            5 => ele = String::from("IR4"),
            6 => ele = String::from("VIS1KM"),
            _ => ele = String::from("UNKNOWN"),
        }

        (prod, ele)
    }

    pub fn proj(&self) -> String {
        match self.attr {
            0 => String::from("卫星坐标系"),
            2 => String::from("墨卡托投影坐标系"),
            4 => String::from("兰伯特投影坐标系"),
            6 => String::from("地理坐标系"),
            _ => String::from("UNKNOWN"),
        }
    }

    pub fn to_grid_img(&self) -> (usize, usize, Vec<f32>) {
        let data = self.values.iter().map(|&v| v as f32).collect::<Vec<_>>();
        (self.width as usize, self.height as usize, data)
    }
}

impl ToGrids for KJSatReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let width = self.width;
        let height = self.height;
        let loc = KJLocationer::new(1).unwrap();
        let start_lng = self.west as f32;
        let end_lng = self.east as f32;
        let start_lat = self.north as f32;
        let end_lat = self.south as f32;
        let step = 0.04;
        let ni = (end_lng - start_lng) / step;
        let ni = ni as i16 + 1;
        let nj = -(end_lat - start_lat) / step;
        let nj = nj as i16 + 1;

        dbg!(ni, nj);
        let data_size = ni as usize * nj as usize;

        dbg!(ni, nj, data_size);
        let mut values = vec![crate::MISSING; data_size];
        values.iter_mut().enumerate().for_each(|(idx, v)| {
            let r = idx / ni as usize;
            let c = idx % ni as usize;
            let lon = start_lng + c as f32 * step;
            let lat = start_lat - r as f32 * step;
            let (x, y) = loc.lbt_lat_lon_to_xy_coord_proc(lat, lon);
            if x >= 0.0 && y >= 0.0 && x < width as f64 && y < height as f64 {
                let ix = x as usize;
                let iy = y as usize;
                // let index = (height as usize - 1 - iy) * width as usize + ix;
                let index = iy * width as usize + ix;
                if index < data_size {
                    *v = self.values[index] as f32;
                }
            }
        });

        let date_time = self.data_date_time();
        let prod_el = self.data_prod_ele();
        let sgrid = SingleGrid::<_, f32> {
            ni: ni as i64,
            nj: nj as i64,
            start_lat: start_lat as f64,
            end_lat: end_lat as f64,
            start_lng: start_lng as f64,
            end_lng: end_lng as f64,
            lat_gap: -step as f64,
            lng_gap: step as f64,
            level: None,
            forecast_time: 0,
            data_date: date_time.0,
            data_time: date_time.1,
            product: prod_el.0,
            element: prod_el.1,
            center: String::from("center"),
            data_des: String::from("kjsat"),
            values,
        };
        Some(vec![sgrid])
    }
}
