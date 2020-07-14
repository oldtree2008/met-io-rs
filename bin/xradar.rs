#![allow(no_snake_case)]
use binread::prelude::*;
use binread::NullString;
use encoding_rs::*;
use image::{imageops, GenericImageView, ImageBuffer, RgbaImage};

use met_io_rs::*;
use palette::*;
use std::convert::Into;
use std::convert::TryInto;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::SeekFrom;
use std::path::Path;

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Product {
    address: Address,
    performance: Performance,
    observe: Observe,
}
#[derive(Debug, BinRead)]
#[br(little)]
pub struct Address {
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
    Startsecond: u16,
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
    EndMouth: u16,         //观测结束时间月
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
pub struct XRadarReader;

impl XRadarReader {
    // pub fn new()->Option<RadialData> {
    //     None
    // }
}

// impl Into<RadialData> for XRadarReader {
//      fn into(self)->RadialData {
//         RadialData {
//             eles:vec![],
//             azs:vec![vec![]],
//             rs:vec![vec![]],
//             data:vec![vec![vec![]]],
//         }
//      }
// }

pub fn main() {
    let fname = r##"H:\data\20200704_164546.00.002.001_R1"##;
    let p = Path::new("palette/REFcolortable.xml");
    let p = Path::new("palette/xradar.xml");
    let pal = Palette::new_with_file(&p).unwrap();
    let mut file = File::open(fname).unwrap();

    let mut d = Vec::new();
    file.read_to_end(&mut d).unwrap();
    let mut reader = Cursor::new(&d);

    // fn read_le_i16(input: &mut &[u8]) -> i16 {
    //     let (int_bytes, rest) = input.split_at(std::mem::size_of::<i16>());
    //     *input = rest;
    //     i16::from_be_bytes(int_bytes.try_into().unwrap())
    // }

    let p: Product = reader.read_le().unwrap();

    let _elNum = &p.observe.SSLayerNumber;

    dbg!(&p);

    // let province = &p.address.Province_;
    // let province = String::from_utf16_lossy(province);
    // dbg!(province);

    dbg!(_elNum);
    let radiohead_size = 64;

    let mut vol_ref = Vec::new();
    for el in 0..*_elNum {
        println!("layer {}", el);
        let refBins = &p.observe.ERDistanceNum[el as usize];
        let dplBins = &p.observe.ECDistanceNum[el as usize];
        let offset = &p.observe.PPIInFileSD[el as usize];
        let length = radiohead_size + refBins * 7 + dplBins * 2;
        let az_num = &p.observe.ERadialNum[el as usize];
        dbg!(refBins, dplBins, offset, length, az_num);
        let mut az_ref = Vec::new();
        for az in 0..*az_num {
            //定位到数据位置
            let pos = offset + (az as u32) * length as u32 + radiohead_size as u32;
            // dbg!(offset);
            //提取反射率 一个字节
            let mut bin_data = Vec::new();
            for bin in 0..*refBins {
                let dpos = (pos + bin as u32) as usize;
                // if d[dpos]!=0u8 {
                //     dbg!(&d[dpos]);
                // }
                let mut vv = d[dpos] as f32;
                if vv > 2.0 && vv < 255.0 {
                    vv = vv * 0.5 - 33.0;
                    bin_data.push(vv);
                } else {
                    bin_data.push(999.0);
                }
            }
            // println!("{:?}",bin_data);
            az_ref.push(bin_data);
        }
        az_ref.push(az_ref[0].clone());
        vol_ref.push(az_ref);
    }

    fn find_index(azs: &Vec<f32>, az: f32) -> Option<usize> {
        let az_len = azs.len();
        for (i, a) in azs[0..az_len - 1].iter().enumerate() {
            if az >= azs[i] && az < azs[i + 1] {
                return Some(i);
            }
        }
        None
    }
    let h = 0.0;
    let elevation = 14.0;
    let z = 520.0;
    let res = 150.0;
    let R: usize = 1000;
    let W = 2 * R;
    let mut grid_value = vec![-999.0; 2 * R * 2 * R];
    let elv_values = &vol_ref[2];
    let mut elv_azs = Vec::new();
    for i in 0..=360 {
        elv_azs.push(i as f32);
    }
    grid_value.iter_mut().enumerate().for_each(|(i, d)| {
        let y = i / (2 * R);
        let x = i % (2 * R);
        let x = x as f32 - 1000.0;
        let y = y as f32 - 1000.0;
        // println!("x {} y {}",x,y);
        let dst = (x * x + y * y).sqrt();

        //for ppi
        let (az, rang, elv) =
            transforms::cartesian_to_antenna_cwr(x * 150.0, -y * 150.0, elevation, h);
        //for ppz
        // let (_, _, z) = transforms::cartesian_to_antenna_cwr(x, -y, elevation, h);
        // let (az, rang, elv) = transforms::cartesian_xyz_to_antenna(x, -y, z, h);
        // println!("elv {}",elv);

        if rang < 999.0 * 150.0
        /*&& x>-50.0 && x<50.0  && y>-50.0 && y<50.0*/
        {
            // let elv_idx = find_index(&elvs, elv);
            // if let Some(elv_idx) = elv_idx {
            //     let elv_azs = &elv_az[elv_idx]; // 第一层上的所有方位角
            //     let elv_values = &elv_az_range_value[elv_idx];
            // println!("elv_idx {:?}   elv {}", elv_idx, elv);

            let az = az.to_degrees();
            let idx = find_index(&elv_azs, az);
            if let Some(ii) = idx {
                let az0 = elv_azs[ii];
                let az1 = elv_azs[ii + 1];
                let rang0 = (rang / 150.0).floor() as usize;
                let rang1 = (rang / 150.0).ceil() as usize;
                let mut v00 = elv_values[ii][rang0] as f32;
                let mut v01 = elv_values[ii][rang1] as f32;
                let mut v10 = elv_values[ii + 1][rang0] as f32;
                let mut v11 = elv_values[ii + 1][rang1] as f32;

                let v = met_io_rs::interp_ppi(
                    az,
                    rang / 150.0,
                    az0,
                    az1,
                    rang0 as f32,
                    rang1 as f32,
                    v00 as f32,
                    v01 as f32,
                    v10 as f32,
                    v11 as f32,
                );
                // let v = (v - 64.0) / 2.0;
                *d = v;
                // println!(
                //     "az {} az0 {} az1 {} range {} range0 {} range1 {}  v00 {}  v01 {} v10 {} v11 {} v {}",
                //     // elv,
                //     az,
                //     &elv_azs[ii],
                //     &elv_azs[ii + 1],
                //     rang,
                //     rang0,
                //     rang1,
                //     v00,
                //     v01,
                //     v10,
                //     v11,
                //     v
                // );

                // println!("x {} y {} v {}",x,y,v);
            }
        }
        // }
    });

    let mut imgbuf = ImageBuffer::new(2000, 2000);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = y * 2000 as u32 + x;

        let v = grid_value[index as usize];
        let c = pal.get_color(v as f64).unwrap();
        *pixel = image::Rgba([c.r, c.g, c.b, c.a]);
    }
    imgbuf.save("radar2.png").unwrap();
}
