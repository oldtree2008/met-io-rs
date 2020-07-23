#![allow(non_snake_case)]
use crate::SingleGrid;
use crate::ToGrids;
use anyhow::*;
use binread::prelude::*;
use std::fs::File;
use std::io::{Cursor, Read};

#[derive(Debug, BinRead)]
#[br(little)]
pub struct GpfReader {
    pub wFileID: i16,    //2，固定为GF
    pub wVersion: i16,   //4，版本号
    pub wSatID: i16,     //6，卫星标识，FY2C为35
    pub wYear: i16,      //8，开始扫描时间：年
    pub wMonth: i16,     //10，开始扫描时间：月
    pub wDay: i16,       //12，开始扫描时间：日
    pub wHour: i16,      //14，开始扫描时间：时
    pub wMinute: i16,    //16，开始扫描时间：分
    pub wChNums: i16,    //18，通道数目
    pub wPjType: i16, //20, 投影类型，不投影=0, 等经纬度=1, 麦卡托=2, 兰伯特=3,                                                //极射赤面=4, 艾尔伯斯投影=5
    pub wWidth: i16,  //22，宽度
    pub wHeight: i16, //24，高度
    pub fCLonRes: f32, //28，投影展开面上中心点像素对应的地球点的分辨率
    pub fCLatRes: f32, //32，投影展开面上中心点像素对应的地球点的分辨率
    pub fStdLat1: f32, //36，标准纬度1
    pub fStdLat2: f32, //40，标准纬度2
    pub fEarthR: f32, //44，地球半径
    pub fMinLat: f32, //48，投影范围最小纬度
    pub fMaxLat: f32, //52，投影范围最大纬度
    pub fMinLon: f32, //56，投影范围最小经度
    pub fMaxLon: f32, //60，投影范围最大经度
    pub fLtLat: f32,  //64，左上角纬度
    pub fLtLon: f32,  //68，左上角经度
    pub fRtLat: f32,  //72，右下角纬度
    pub fRtLon: f32,  //76，右下角经度
    pub fLbLat: f32,  //80，左下角纬度
    pub fLbLon: f32,  //84，左下角经度
    pub fRbLat: f32,  //88，右上角纬度
    pub fRbLon: f32,  //92，右上角经度
    pub fStdLon: f32, //96，标准经度(中心经度)
    pub fCenterLon: f32, //100，中心经度
    pub fCenterLat: f32, //104，中心纬度
    #[br(count = 128)]
    pub ucChIndex: Vec<u8>, /*= new byte[128]*/
    //104+128=232，通道索引：红外1为1，红外2为2，红外3为3，红外4为4，可见光为5
    pub fPLonRes: f32, //236，投影展开面的经向分辨率
    pub fPLatRes: f32, //240，投影展开面的纬向分辨率
    #[br(count = 1808)]
    pub cReserved: Vec<u8>, // 2048，保留空间， 以备后用
    #[br(count = 1024)]
    pub stand1: Vec<f32>, //定标数据
    #[br(count = 1024)]
    pub stand2: Vec<f32>,
    #[br(count = 1024)]
    pub stand3: Vec<f32>,
    #[br(count = 1024)]
    pub stand4: Vec<f32>,
    #[br(count = 1024)]
    pub stand5: Vec<f32>,
    #[br(count = 1024)]
    pub stand6: Vec<f32>,
    #[br(count = 1024)]
    pub stand7: Vec<f32>,
    #[br(count = 1024)]
    pub stand8: Vec<f32>,

    #[br(calc= wWidth as i32 * wHeight as i32)]
    pub data_size: i32,
    #[br(count= data_size)]
    pub channel1: Vec<i16>,
    #[br(count=data_size)]
    pub channel2: Vec<i16>,
    #[br(count=data_size)]
    pub channel3: Vec<i16>,
    #[br(count=data_size)]
    pub channel4: Vec<i16>,
    #[br(count=data_size)]
    pub channel5: Vec<i8>,
}

impl GpfReader {
    pub fn read(fname: &str) -> Result<GpfReader> {
        let mut f = File::open(fname).unwrap();
        let mut data = Vec::new();
        f.read_to_end(&mut data);
        let mut cursor = Cursor::new(&data);
        let gpfreader: GpfReader = BinRead::read(&mut cursor).unwrap();

        //   dbg!(&gpfreader);
        dbg!(
            &gpfreader.channel1.len(),
            &gpfreader.wWidth,
            &gpfreader.wHeight
        );
        Ok(gpfreader)
    }

    pub fn prod_name(&self) -> Option<String> {
        if self.wSatID == 35 {
            Some(String::from("FY2C"))
        } else if self.wSatID == 36 {
            Some(String::from("FY2D"))
        } else if self.wSatID == 37 {
            Some(String::from("FY2E"))
        } else if self.wSatID == 38 {
            Some(String::from("FY2F"))
        } else {
            None
        }
    }
    pub fn proj(&self) -> String {
        match self.wPjType {
            0 => String::from("不投影"),
            1 => String::from("等经纬度"),
            2 => String::from("墨卡托投影坐标系"),
            3 => String::from("兰伯特投影坐标系"),
            _ => String::from("UNKNOWN"),
        }
    }

    pub fn to_grid_img(&self) -> (usize, usize, Vec<f32>) {
        //   let data = &self
        //       .channel5
        //       .iter()
        //       .map(|d| (*d * 4) as f32)
        //       .collect::<Vec<_>>();
        //   (self.wWidth as usize, self.wHeight as usize, data.clone())

        let data = &self
            .channel4
            .iter()
            .map(|d| (*d / 4) as f32)
            .collect::<Vec<_>>();
        (self.wWidth as usize, self.wHeight as usize, data.clone())
    }
}

impl ToGrids for GpfReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let mut datas = Vec::new();
        let ni = 2000;
        let nj = 2000;

        let start_lng = &self.fLbLon;
        let end_lng = &self.fRbLon;
        let start_lat = &self.fLtLat;
        let end_lat = &self.fLbLat;

        let lng_gap = 0.05;
        let lat_gap = -0.05;

        let prod_name = self.prod_name().unwrap_or(String::from("UNKNOWN"));
        let data_date = format!("{}{:02}{:02}", self.wYear, self.wMonth, self.wDay);
        let data_time = format!("{:02}{:02}00", self.wHour, self.wMinute);
        let data_des = format!("{}{}{}", data_date, data_time, prod_name);

        let data = &self
            .channel1
            .iter()
            .map(|d| (*d / 4) as f32)
            .collect::<Vec<_>>();

        datas.push(SingleGrid::<_, f32> {
            ni: ni as i64,
            nj: nj as i64,
            start_lng: *start_lng as f64,
            end_lng: *end_lng as f64,
            start_lat: *start_lat as f64,
            end_lat: *end_lat as f64,
            lng_gap: lng_gap as f64,
            lat_gap: lat_gap as f64,
            product: prod_name.clone(),
            element: String::from("IR1"),
            data_date: data_date.clone(),
            data_time: data_time.clone(),
            data_des: data_des.clone(),
            level: None,
            values: data.clone(),
            forecast_time: 0,
            center: String::from(""),
        });
        let data = &self
            .channel2
            .iter()
            .map(|d| (*d / 4) as f32)
            .collect::<Vec<_>>();

        datas.push(SingleGrid::<_, f32> {
            ni: ni as i64,
            nj: nj as i64,
            start_lng: *start_lng as f64,
            end_lng: *end_lng as f64,
            start_lat: *start_lat as f64,
            end_lat: *end_lat as f64,
            lng_gap: lng_gap as f64,
            lat_gap: lat_gap as f64,
            product: prod_name.clone(),
            element: String::from("IR2"),
            data_date: data_date.clone(),
            data_time: data_time.clone(),
            data_des: data_des.clone(),
            level: None,
            values: data.clone(),
            forecast_time: 0,
            center: String::from(""),
        });
        let data = &self
            .channel3
            .iter()
            .map(|d| (*d / 4) as f32)
            .collect::<Vec<_>>();

        datas.push(SingleGrid::<_, f32> {
            ni: ni as i64,
            nj: nj as i64,
            start_lng: *start_lng as f64,
            end_lng: *end_lng as f64,
            start_lat: *start_lat as f64,
            end_lat: *end_lat as f64,
            lng_gap: lng_gap as f64,
            lat_gap: lat_gap as f64,
            product: prod_name.clone(),
            element: String::from("IR3"),
            data_date: data_date.clone(),
            data_time: data_time.clone(),
            data_des: data_des.clone(),
            level: None,
            values: data.clone(),
            forecast_time: 0,
            center: String::from(""),
        });
        let data = &self
            .channel4
            .iter()
            .map(|d| (*d / 4) as f32)
            .collect::<Vec<_>>();

        datas.push(SingleGrid::<_, f32> {
            ni: ni as i64,
            nj: nj as i64,
            start_lng: *start_lng as f64,
            end_lng: *end_lng as f64,
            start_lat: *start_lat as f64,
            end_lat: *end_lat as f64,
            lng_gap: lng_gap as f64,
            lat_gap: lat_gap as f64,
            product: prod_name.clone(),
            element: String::from("IR4"),
            data_date: data_date.clone(),
            data_time: data_time.clone(),
            data_des: data_des.clone(),
            level: None,
            values: data.clone(),
            forecast_time: 0,
            center: String::from(""),
        });
        let data = &self
            .channel5
            .iter()
            .map(|d| (*d * 4) as f32)
            .collect::<Vec<_>>();

        datas.push(SingleGrid::<_, f32> {
            ni: ni as i64,
            nj: nj as i64,
            start_lng: *start_lng as f64,
            end_lng: *end_lng as f64,
            start_lat: *start_lat as f64,
            end_lat: *end_lat as f64,
            lng_gap: lng_gap as f64,
            lat_gap: lat_gap as f64,
            product: prod_name.clone(),
            element: String::from("VIS"),
            data_date: data_date.clone(),
            data_time: data_time.clone(),
            data_des: data_des.clone(),
            level: None,
            values: data.clone(),
            forecast_time: 0,
            center: String::from(""),
        });
        Some(datas)
    }
}
