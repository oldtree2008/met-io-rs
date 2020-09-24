#![allow(non_snake_case)]
use crate::kjlocationer::KJLocationer;
use crate::MetError;
use crate::{SingleGrid, ToGrids};
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::result::Result;

use binread::prelude::*;
use binread::NullString;
use std::io::SeekFrom;

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Product {
    pub fName: NullString,
    #[br(seek_before = SeekFrom::Start(12))] //1-12   Sat96文件名
    pub byteSequence: u16,
    #[br(is_little=(byteSequence==0))] //13_14  整形数的字节顺序
    pub firstClassHeadLength: i16, //15-16  第一级文件头长度
    #[br(is_little=(byteSequence==0))]
    pub secondClassHeadLength: i16, //17-18  第二级文件头长度
    #[br(is_little=(byteSequence==0))]
    pub padDataLength: i16, //19-20  填充段数据长度
    #[br(is_little=(byteSequence==0))]
    pub recordLength: i16, //21-22  记录长度
    #[br(is_little=(byteSequence==0))]
    pub headRecordNumber: i16, //23-24   文件头占用记录数
    #[br(is_little=(byteSequence==0))]
    pub dataRecordNumber: i16, //25-26   产品数据占用记录数
    #[br(is_little=(byteSequence==0))]
    pub productCategory: i16, //27-28   产品类别
    #[br(is_little=(byteSequence==0))]
    pub compressMethod: i16, //29-30   压缩方式
    pub formatString: NullString, //31-38   格式说明字符串
    #[br(is_little=(byteSequence==0),seek_before = SeekFrom::Start(38))]
    pub qualityFlag: i16, //39-40   产品数据质量标记

    //for product1
    #[br(is_little=(byteSequence==0),if(productCategory==1))]
    pub header1: Option<Header1>, //第一类产品的头
    #[br(is_little=(byteSequence==0),if(productCategory==1),calc=header1.as_ref().unwrap().widthOfImage  as u64)]
    data1_width: Option<u64>,
    #[br(is_little=(byteSequence==0),if(productCategory==1),calc=header1.as_ref().unwrap().heightOfImage  as u64)]
    data1_height: Option<u64>,
    #[br(is_little=(byteSequence==0),if(productCategory==1),calc=data1_height.unwrap() * data1_width.unwrap())]
    data1_len: Option<u64>,
    #[br(is_little=(byteSequence==0),if(productCategory==1),calc=(headRecordNumber*recordLength) as u64)]
    data1_start: Option<u64>,
    #[br(is_little=(byteSequence==0),if(productCategory==1),seek_before=SeekFrom::Start(data1_start.unwrap()),count=data1_len.unwrap())]
    pub data1: Option<Vec<u8>>,

    //HeadExt
    #[br(calc=(firstClassHeadLength +secondClassHeadLength + padDataLength) as u64)]
    index: u64,
    #[br(seek_before=SeekFrom::Start(index),pad_size_to=64)]
    fileName: NullString,
    /// 1-64    64 bytes    Sat2004文件名
    #[br(pad_size_to = 8)]
    formateVersion: NullString,
    /// 65-72   8 bytes     格式版本号
    #[br(pad_size_to = 8)]
    manufacturer: NullString,
    /// 73-80   8 bytes     生产商
    #[br(pad_size_to = 16)]
    satelliteName: NullString,
    /// 81-96   16 bytes    卫星名
    #[br(pad_size_to = 8)]
    instrumentName: NullString,
    /// 97-104  8 bytes     仪器名
    #[br(pad_size_to = 8)]
    channelCount: NullString,
    /// 105-112 8 bytes     总通道数
    #[br(pad_size_to = 8)]
    programVersion: NullString, // 113-120 8 bytes     处理程序的版本号
    #[br(pad_size_to = 8)]
    copyright: NullString, // 121-128 8 bytes     版权
    #[br(pad_size_to = 8)]
    sizeFilled: NullString, // 129-136 8bytes      扩展段的填充段长度
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Header1 {
    pub satelliteName: NullString, //41-48  卫星名
    #[br(seek_before = SeekFrom::Start(48))]
    pub year: u16, //49-50   时间（年）
    pub month: u16,                //51-52   时间（月
    pub day: u16,                  //53-54   时间（日）
    pub hour: u16,                 //55-56   时间（时）
    pub minute: u16,               //57-58   时间（分）
    pub channel: i16,              //59-60   通道号
    pub flagOfProjection: i16,     //61-62   投影方式
    pub widthOfImage: i16,         //63-64   图形宽度
    pub heightOfImage: i16,        //65-66   图像高度
    scanLineNumberOfImageTopLeft: i16, //67-68   图像左上角扫描线号
    pixelNumberOfImageTopLeft: i16, //69-70   图像左上角象元号
    sampleRatio: i16,              //71-72   抽样率
    pub latitudeOfNorth_: i16,     //73-74   地理范围（北纬）
    #[br(calc=latitudeOfNorth_ as f32/100.0)]
    pub latitudeOfNorth: f32, //73-74   地理范围（北纬）
    pub latitudeOfSouth_: i16,     //75-76   地理范围（南纬）
    #[br(calc=latitudeOfSouth_ as f32 /100.0)]
    pub latitudeOfSouth: f32,
    pub longitudeOfWest_: i16,
    #[br(calc=longitudeOfWest_ as f32 /100.0)]
    pub longitudeOfWest: f32, //77-78   地理范围（西经）
    pub longitudeOfEast_: i16,
    #[br(calc=longitudeOfEast_ as f32 /100.0)]
    pub longitudeOfEast: f32, //79-80   地理范围（东经）
    pub centerLatitudeOfProjection: i16, //81-82   投影中心纬度（度*100）
    pub centerLongitudeOfProjection: i16, //83-84   投影中心经度（度*100
    pub standardLatitude1: i16,          //85-86   投影标准纬度1（或标准经度）（度*100）
    pub standardLatitude2: i16,          //87-88   标准投影纬度2
    pub horizontalResolution: i16,       //89-90   投影水平分辨率
    pub verticalResolution: i16,         //91-92   投影垂直分辨率
    pub overlapFlagGeoGrid: i16,         //93-94   地理网格叠加标志
    pub overlapValueGeoGrid: i16,        //95-96   地理网格叠加值
    pub dataLengthOfColorTable: i16,     //97-98   雕色表数据长度
    pub dataLengthOfCalibration: i16,    //99-100  定标数据块长度
    pub dataLengthOfGeolocation: i16,    //101-102 定位数据块长度
    reserved: i16,                       //103-104 保留
    #[br(if (dataLengthOfColorTable!=0),pad_size_to=256,count=256)]
    colorR: Option<Vec<u8>>,
    #[br(if (dataLengthOfColorTable!=0),pad_size_to=256,count=256)]
    colorG: Option<Vec<u8>>,
    #[br(if (dataLengthOfColorTable!=0),pad_size_to=256,count=256)]
    colorB: Option<Vec<u8>>,
    #[br(if (dataLengthOfCalibration!=0),pad_size_to=256,count=dataLengthOfCalibration/2)]
    calibration: Option<Vec<u16>>,
}

#[derive(Debug)]
pub struct AwxReader(pub Product);

impl AwxReader {
    pub fn new(fname: &str) -> Result<AwxReader, MetError> {
        let mut f = File::open(fname)?;
        let mut d = Vec::new();
        f.read_to_end(&mut d)?;
        let mut reader = Cursor::new(&d);
        dbg!(d.len());
        let product: Product = reader.read_le()?;
        dbg!(&product.productCategory);
        dbg!(&product.manufacturer);
        // dbg!(&product.data1.unwrap()[900..1000]);
        // dbg!(product.header1.unwrap().channel);
        Ok(AwxReader(product))
    }
}

impl ToGrids for AwxReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        // let (lng1,lat1) = loc.lbt_grid_ij_to_lat_and_longitude_proc(0f32,0f32);
        // let (lng2,lat2) = loc.lbt_grid_ij_to_lat_and_longitude_proc(0f32,512f32);

        // let (lng3,lat3) = loc.lbt_grid_ij_to_lat_and_longitude_proc(512f32,0f32);
        // let (lng4,lat4) = loc.lbt_grid_ij_to_lat_and_longitude_proc(512f32,512f32);

        // println!("lng1 {} lat1 {}   lng2 {} lat2 {}",lng1,lat1,lng2,lat2);
        // println!("lng3 {} lat3 {}   lng4 {} lat4 {}",lng3,lat3,lng4,lat4);

        let p = &self.0;
        if p.productCategory == 1 {
            let header = p.header1.as_ref().unwrap();
            let product = format!("awx/{}", &header.satelliteName.to_string());
            let chanel = header.channel;
            let element = if chanel == 1 {
                "IR1".to_string()
            } else if chanel == 2 {
                "IR2".to_string()
            } else if chanel == 3 {
                "IR3".to_string()
            } else if chanel == 4 {
                "IR4".to_string()
            } else if chanel == 5 {
                "VIS".to_string()
            } else if chanel == 34 {
                "SAND".to_string()
            } else if chanel == 25938 {
                "VIS1.25".to_string()
            } else {
                "UNKNOWN".to_string()
            };
            let proj = header.flagOfProjection;
            let project = if proj == 1 {
                "兰伯特".to_string()
            } else if proj == 2 {
                "麦卡托投影".to_string()
            } else if proj == 3 {
                "极射投影".to_string()
            } else if proj == 4 {
                "等经纬度投影".to_string()
            } else if proj == 5 {
                "等面积投影".to_string()
            } else {
                "未投影".to_string()
            };

            println!("proj {}", project);
            println!(
                "res {}  {}",
                header.horizontalResolution, header.verticalResolution
            );
            println!(
                "center {}  {}",
                header.centerLatitudeOfProjection, header.centerLongitudeOfProjection
            );
            println!(
                "lats {}  {}",
                header.standardLatitude1, header.standardLatitude2
            );
            let lat0 = header.centerLatitudeOfProjection as f32 * 0.01;
            let lon0 = header.centerLongitudeOfProjection as f32 * 0.01;
            let xres = header.horizontalResolution as f32 * 0.01;
            let yres = header.verticalResolution  as f32* 0.01;
            let mut width = header.widthOfImage;
            let mut height = header.heightOfImage;
            let image_width = width;
            let image_height = height;
            let mut end_lat = header.latitudeOfNorth;
            let mut start_lat = header.latitudeOfSouth;
            let mut start_lng = header.longitudeOfWest;
            let mut end_lng = header.longitudeOfEast;
            let mut lat_gap = (end_lat - start_lat) / (height - 1) as f32;
            let mut lng_gap = (end_lng - start_lng) / (width - 1) as f32;
            let data = p.data1.as_ref().unwrap();

            // println!("data_len {}  {}",data.len(),(width as usize * height as usize));
            let data_size = width as usize * height as usize;
            // println!("width {}  height {}  datasize {} ",width,height,data_size);
            if proj == 1 {
                start_lng = 60.0;
                end_lng = 160.0;
                start_lat = -0.5;
                end_lat = 60.5;

                lng_gap = 0.05;
                lat_gap = 0.05;

                width = ((end_lng - start_lng) / lng_gap) as i16 + 1;
                height = ((end_lat - start_lat) / lat_gap) as i16 + 1;

                // width = f32::ab
            }
            // println!("width {}  height {}",width,height);
            let mut values = vec![0f32; width as usize * height as usize];
            if proj == 1 {
                let loc =
                    KJLocationer::with_params(lat0, lon0, image_width as i32, image_height as i32, xres, yres);
                let ni = width;
                let nj = height;
                values.iter_mut().enumerate().for_each(|(idx, v)| {
                    let r = idx / ni as usize;
                    let c = idx % ni as usize;
                    let lon = start_lng + c as f32 * lng_gap;
                    let lat = start_lat + r as f32 * lat_gap;
                    let (x, y) = loc.lbt_lat_lon_to_xy_coord_proc(lat, lon);
                    if x >= 0.0 && y >= 0.0 && x < image_width as f64 && y < image_height as f64 {
                        let ix = x as usize;
                        let iy = y as usize;
                        // let index = (height as usize - 1 - iy) * width as usize + ix;
                        let index = iy * image_width as usize + ix;
                        if index < data_size {
                            *v = data[index] as f32;
                        }
                    }
                });
            } else {
                values.iter_mut().enumerate().for_each(|(indx, v)| {
                    let r = indx / width as usize;
                    let c = indx % width as usize;
                    let r1 = height as usize - 1 - r;
                    let dindx = r1 * width as usize + c;
                    *v = data[dindx] as f32;
                });
            }

            let data_date = format!("{}{:02}{:02}", header.year, header.month, header.day);
            let data_time = format!("{:02}{:02}00", header.hour, header.minute);

            let grid = SingleGrid {
                ni: width as i64,
                nj: height as i64,
                lat_gap: lat_gap as f64,
                lng_gap: lng_gap as f64,
                start_lat: start_lat as f64,
                start_lng: start_lng as f64,
                end_lat: end_lat as f64,
                end_lng: end_lng as f64,
                level: None,
                element: element,
                values: values,
                data_date: data_date,
                data_time: data_time, //时次
                forecast_time: 0,     //时效
                center: String::from("satelite"),
                product: product.clone(),
                station: None,
            };
            // println!(
            //     "ni {} nj {} lat_gap {} lng_gap {} start_lat {} start lng {}",
            //     width, height, lat_gap, lng_gap, start_lat, start_lng
            // );
            return Some(vec![grid]);
        } else {
            unimplemented!();
        }
    }
}
