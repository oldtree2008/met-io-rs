#![allow(non_snake_case)]
use crate::data_type::RadialData;
use crate::error::MetError;
use crate::{SingleGrid, ToGrids};
use binread::prelude::*;
use binread::NullString;
use encoding_rs::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::result::Result;
#[derive(Debug, BinRead)]
#[br(little)]
pub struct Product {
    address: Address,
    performance: Performance,
    observe: Observe,
}
#[derive(Debug, BinRead)]
#[br(little)]
struct Address {
    FileHeadLength: i16, //文件头长度
    #[br(pad_size_to = 20)]
    Mode: NullString, //雷达型号
    #[br(count = 20)]
    Province_: Vec<u8>,
    #[br(calc=GBK.decode(&Province_).0.trim_end_matches('\u{0}').to_string())]
    Province: String,
    #[br(count = 20)]
    Area_: Vec<u8>,
    #[br(calc=GBK.decode(&Area_).0.trim_end_matches('\u{0}').to_string())]
    Area: String,
    #[br(count = 20)]
    AreaName_: Vec<u8>, //区站名
    #[br(calc=GBK.decode(&AreaName_).0.trim_end_matches('\u{0}').to_string())]
    AreaName: String,
    #[br(count = 20)]
    VersionNum_: Vec<u8>, //文件版本格式号 [4 - 7]存放数据来源
    #[br(calc=GBK.decode(&VersionNum_).0.trim_end_matches('\u{0}').to_string())]
    VersionNum: String,
    #[br(pad_size_to = 20)]
    TaskName: NullString, //雷达扫描任务名称
    #[br(pad_size_to = 20)]
    NoData1: NullString, //保留1
    Longitude_: i32, //天线经度 单位取1 / 360000度(东经为正，西经为负)
    #[br(calc=Longitude_ as f32/360000.0)]
    Longitude: f32,
    Latitude_: i32, //天线纬度 单位取1 / 360000度(北纬为正，南纬为负)
    #[br(calc=Latitude_ as f32/360000.0)]
    Latitude: f32,
    Height: i32,      //天线海拔高度, 以mm为单位
    MaxElevate: i16,  //测站四周地物最大仰角,百分之一度为单位
    BestElevate: i16, //测站四周地物最佳仰角,百分之一度为单位
    NoData2: i16,     //保留2
}
#[derive(Debug, BinRead)]
#[br(little)]
pub struct Performance {
    AntennaGain: u16,        //天线增益，以.01db为单位
    VerticalBW: u16,         //垂直波束宽度
    HorizontalBW: u16,       //水平波束宽度
    Polarizate: u16, //极化状况  - 0：为水平极化； - 1：垂直极化；- 2：为双极化（双偏振）；- 3：为圆偏振； - 4：其它
    WaveLength: u32, //波长 以微米为单位
    PeakPower: u32,  //雷达峰值功率
    FirstSideLobeLevel: u16, //第一旁瓣电平，取绝对值(单位取百分之一dB)
    LineA: u16,      //线性接收机动态范围，百分之一 dB为单位
    AGCDelayNum: u16, //AGC延迟量 以微秒为单位
    LogA: u16,       //对数接收机，百分之一 dbmw为单位
    LineMinTestPower: u16, //线性接收机最小可测功率 百分之一，dbwm为单位
    NoiseT: u16,     //噪声消除量化阀值
    ClutterT: u16,   //多普勒杂波消除阀值，单位 .01db
    SQIT: u16,       //SQI 阀值
    VelocityP: u16,  //速度处理方式 0:无速度处理； 1:PPI； 2:FFT
    FilterP: u16, //地物处理方式 - 0:无地物处理；  - 1:地物杂波图扣除法； - 2:滤波器处理；- 3:滤波器＋地物杂波图 法； - 4:谱分析法
    IntensityR: u16, //强度估算采用的通道 - 1:对数; - 2:线性
    iRangeReduction: u16,
}
#[derive(Debug, BinRead)]
#[br(little)]
pub struct Observe {
    ProductNumber: u16, //产品编号 - 0:PPI1 - 1:RHI - 2:立体扫描 - 3.反射率 - 4.速度 - 5.谱宽
    SSLayerNumber: u16, //立体扫描层数
    StartYear: u16,
    StartMonth: u16,
    StartDay: u16,
    StartHour: u16,
    StartMinute: u16,
    StartSecond: u16,
    StartGPSTime: u32, //开始GPS时间，以微秒为单位
    Calibration: u16,  //定标情况 - 0:没有定标 - 1:自动定标 - 2:一周内人工定标 - 3:一月内人工定标
    IntensityI: u16,   //强度积分次数
    VelocityP: u16,    //速度处理样本数
    #[br(count = 30)]
    ID: Vec<u32>, //ID号
    #[br(count = 30)]
    ViewElement: Vec<u8>, //观测要素
    // -1：单强度
    // -2：单速度(单PRF)
    // -3：速度+谱宽(单PRF)
    // -4：单速度(双PRF)
    // -5：速度+谱宽(双PRF)
    // -6：强度+速度(单PRF)
    // -7：强度+速度(双PRF)
    // -8：三要素(单PRF)
    // -9：三要素(双PRF)
    // -10：四要素(ConR+R+V+W,单PRF)
    // -11：四要素(ConR+R+V+W,双PRF)
    #[br(count = 30)]
    SpeedDeambiguity: Vec<u8>, //速度退模糊
    // -0：无退模糊处理
    // -1：软件退模糊
    // -2：双PRF退模糊
    // -3：批示退模糊
    // -4：批示加软件退模糊
    // -5：双PRF退模糊
    // -6：双PRF+软件退模糊
    #[br(count = 30)]
    EFirstPrr: Vec<u16>, //各层第一种脉冲重复频率，计数单位1/10HZ
    #[br(count = 30)]
    ESecondPrr: Vec<u16>, //各层第二种脉冲重复频率，计数单位1/10HZ
    #[br(count = 30)]
    EPulse: Vec<u16>, //各层脉冲宽度，1/100微秒
    #[br(count = 30)]
    EMaxSpeed: Vec<u16>, //各层的最大可测速度，单位：厘米/秒
    #[br(count = 30)]
    ERDistanceNum: Vec<u16>, //各层的反射率距离库数
    #[br(count = 30)]
    ERadialNum: Vec<u16>, //各层采样的径向数
    #[br(count = 30)]
    EClutterL: Vec<u16>, //各层多普勒库长，米为单位
    #[br(count = 30)]
    ERlibL: Vec<u16>, //各层多普勒库长，米为单位
    #[br(count = 30)]
    EStartDistance: Vec<u16>, //各层径向上的第一个库(或者数据)的开始距离，米为单位
    #[br(count = 30)]
    PPIInFileSD: Vec<u32>, //各层PPI在文件中的开始位置，字节，含文件头
    #[br(count = 30)]
    elevation: Vec<u16>, //各层的仰角，单位/100度

    RadialArrangement: u8, //一个径向中的数据排列方式
    STOccupysize: u8,      //一个强度数据占用的字节数，百位数表示
    SPOccupysize: u8,      //一个速度数据占用的字节数，百位数表示
    SWOccupysize: u8,      //一个谱宽数据占用的字节数，百位数表示
    STNoEchoCT: i16,       //强度无回波的代码表
    SPNoEchoCT: i16,       //速度无回波的代码表
    SWNoEchoCT: i16,       //速度无回波的代码表
    STMinIncrement: i16,   //数据中的强度最小增量，
    SPMinIncrement: i16,   //数据中的速度最小增量，*1000
    SWMinIncrement: i16,   //数据中的谱宽最小增量，*1000
    Strength: i16,         //强度
    speed: i16,            //速度
    SpectrumWidth: i16,    //谱宽
    EndYear: u16,          //观测结束时间年
    EndMonth: u16,         //观测结束时间月
    EndDay: u16,           //观测结束时间日
    EndHour: u16,          //观测结束时间时
    EndMinute: u16,        //观测结束时间分
    EndSecond: u16,        //观测结束时间秒
    GPSTime: u32,          //GPS时间
    StructNum: u16,        //结构数组的大小
    RadarScanneMode: u16,  //雷达扫描模式
    #[br(count = 30)]
    BSEScanMode: Vec<i8>, //体扫各层的扫描方式
    #[br(count = 30)]
    ECDistanceNum: Vec<u16>, //各层的多普勒距离库数
    #[br(count = 8)]
    reserve: Vec<i8>,
}
#[derive(Debug)]
pub struct XRadarReader(pub (RadialData));

impl XRadarReader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        let mut file = File::open(fname)?;
        let mut d = Vec::new();
        file.read_to_end(&mut d)?;
        Self::new_from_slice(&d)
    }
    pub fn new_from_slice(d: &[u8]) -> Result<Self, MetError> {
        let mut reader = Cursor::new(&d);
        let p: Product = reader.read_le()?;
        // dbg!(&p.address);

        let mut props = HashMap::new();

        // let xr = self.0;
        // let p: Product = xr.0;
        // let d: Vec<u8> = xr.1;
        let _elNum = &p.observe.SSLayerNumber;

        // dbg!(_elNum);
        let radiohead_size = 64;
        let mut data = Vec::new(); //所有物理量数据
        let mut vol_ref = Vec::new();
        let mut eles = Vec::new();
        let mut azs = Vec::new();
        let mut rs = Vec::new();
        for el in 0..*_elNum {
            println!("layer {}", el);
            let refBins = &p.observe.ERDistanceNum[el as usize];
            let dplBins = &p.observe.ECDistanceNum[el as usize];

            //ERlibL
            let refWidth = &p.observe.ERlibL[el as usize];
            //EClutterL
            let dplWidth = &p.observe.EClutterL[el as usize];
            let offset = &p.observe.PPIInFileSD[el as usize];
            let length = radiohead_size + refBins * 7 + dplBins * 2;
            let az_num = &p.observe.ERadialNum[el as usize];
            // dbg!(refBins, dplBins, offset, length, az_num);
            let mut az_ref = Vec::new();
            let mut el_found = false;
            let mut el_az = Vec::new();

            let mut az_range = Vec::new();
            for az in 0..*az_num {
                let az_pos = offset + (az as u32) * length as u32 + 4;
                let az_value = u16::from_le_bytes([d[az_pos as usize], d[az_pos as usize + 1]]);
                let az_value = az_value as f32 / 100.0;
                // println!("az_value {}",az_value);
                let el_pos = offset + (az as u32) * length as u32 + 6;
                let el_value = u16::from_le_bytes([d[el_pos as usize], d[el_pos as usize + 1]]);
                let el_value = el_value as f32 / 100.0;
                // println!("el_value {}",el_value);
                if !el_found {
                    eles.push(el_value);
                    el_found = true;
                }
                el_az.push(az_value);
                //定位到数据位置
                let pos = offset + (az as u32) * length as u32 + radiohead_size as u32;
                // dbg!(offset);
                //提取反射率 一个字节
                let mut bin_data = Vec::new();
                let mut ranges = Vec::new();
                for bin in 0..*refBins {
                    let dpos = (pos + bin as u32) as usize;
                    let mut vv = d[dpos] as f32;
                    if vv > 2.0 && vv < 255.0 {
                        vv = vv * 0.5 - 33.0;
                        bin_data.push(vv);
                    } else {
                        bin_data.push(crate::MISSING);
                    }
                    // ranges.push((bin * (*refWidth)) as f64);
                    ranges.push(bin as f64 * ((*refWidth) as f64));
                }
                // println!("ranges {:?}",ranges);
                az_range.push(ranges);
                // println!("{:?}",bin_data);
                az_ref.push(bin_data);
            }
            az_ref.push(az_ref[0].clone());
            vol_ref.push(az_ref);
            // el_az.push(360.0);
            azs.push(el_az);
            rs.push(az_range)
        }

        data.push(vol_ref);

        let start_date = format!(
            "{}{:02}{:02}",
            &p.observe.StartYear, &p.observe.StartMonth, &p.observe.StartDay,
        );
        let start_time = format!(
            "{:02}{:02}{:02}",
            &p.observe.StartHour, &p.observe.StartMinute, &p.observe.StartSecond
        );
        let end_time = format!(
            "{}{:02}{:02}{:02}{:02}{:02}",
            &p.observe.EndYear,
            &p.observe.EndMonth,
            &p.observe.EndDay,
            &p.observe.EndHour,
            &p.observe.EndMinute,
            &p.observe.EndSecond
        );
        let prov = &p.address.Province;
        let ar = &p.address.Area;
        props.insert("province".to_string(), prov.clone());
        props.insert("area".to_string(), ar.clone());
        let radial_data = RadialData {
            _extents: (-150000.0, 150000.0, -150000.0, 150000.0),
            eles: eles,
            azs: azs,
            rs: rs,
            data: data,
            start_time,
            start_date,
            elements: vec!["Z".to_string()],
            // end_time,
            lon: *&p.address.Longitude,
            lat: *&p.address.Latitude,
            height: *&p.address.Height as f32 / 1000.0,
            props,
        };

        Ok(XRadarReader(radial_data))
    }
}

impl ToGrids for XRadarReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let radial_data = &self.0;
        radial_data.to_grids()
    }
}
