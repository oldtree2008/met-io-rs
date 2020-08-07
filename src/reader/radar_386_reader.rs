#![allow(non_snake_case,non_camel_case_types)]
use crate::error::MetError;
use crate::RadialData;
use binread::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{Cursor, Read};
use std::result::Result;
use std::str::FromStr;

#[derive(Debug)]
pub struct Radar386Reader {
    fname: String,
    info: Radar386Info,
    data: Vec<u8>,
}

#[derive(Debug, BinRead)]
pub struct Radar386Info {
    perform: RADARPERFORMANCEPARAM_048,
    observe: RADAROBSERVATIONPARAM_048,
}

/**
 *
 * 雷达性能参数【54】
 */
#[derive(Debug, BinRead)]
pub struct RADARPERFORMANCEPARAM_048 {
    pub AntennaG: i32,     //天线法向增益，以0.01dB为计数单位【4】
    pub BeamH: i16,        //法线方向上的波束宽度，以1/100度为计数单位【2】
    pub BeamL: i16,        //切线方向上的波束宽度，以1/100度为计数单位【2】
    pub Polarizations: u8, //极化状况【1】 0=水平 1=垂直 2=双线偏振  3=其他待定
    pub SideLobe: i16,     //第一旁瓣，以0.01dB为计数单位【2】
    pub Power: i32,        //雷达脉冲峰值功率，以瓦为单位【4】
    pub WaveLength: i32,   //波长，以微米为计数单位【4】
    pub LineA: i16,        //接收机动态范围，以0.01dB为计数单位【2】
    pub LineMinPower: u16, //接收机最小可测功率，计数单位为0.01dBm【2】
    pub ClutterP: u8,      //地杂波消除阈值，计数单位为0.2dB【1】
    pub ClutterS: u8,      //海杂波消除阈值，计数单位为0.2dB【1】
    pub VelocityP: u8,     //速度处理方式【1】  0=无速度处理  1=PPP  2=FFT
    pub FilterP: u8, //地杂波消除方式【1】0=无地杂波消除 1=地杂波扣除法 2=地杂波+滤波器处理 3=滤波器处理 4=谱分析处理 5=其他处理法
    pub FilterS: u8, //海杂波消除方式【1】 待定
    pub NoiseT: u8,  //噪声消除阈值（0-255）【1】
    pub IntensityR: u8, //强度估算是否进行了距离订正【1】 0=无 1=以进行了距离订正
    #[br(count = 24)]
    Reserved: Vec<u8>, //保留【24】
}
/**
 * 雷达探测参数【65+475】
 */
#[derive(Debug, BinRead)]
struct RADAROBSERVATIONPARAM_048 {
    SType: u8, //扫描方式【1】 1= 对应346雷达搜索模式 2= 对应346雷达单强度PPI 3= 对应346雷达单强度RHI 4= 对应346雷达强度速度PPI 5= 对应346雷达体扫
    //11= 对应382雷达兼容模式PPI 12= 对应382雷达兼容模式体扫 13= 对应382雷达专用模式PPI 14= 对应382雷达专用模式体扫
    WeatherMode: u8, // 天气状况【1】
    SYear: i16,      //观测记录开始时间的年（2000-）【2】
    SMonth: u8,      //观测记录开始时间的月（1-12）【1】
    SDay: u8,        //观测记录开始时间的日（1-31）【1】
    SHour: u8,       //观测记录开始时间的时（00-23）【1】
    SMinute: u8,     //观测记录开始时间的分（00-59）【1】
    SSecond: u8,     //观测记录开始时间的秒（00-59）【1】*/
    // String end_time;    //[7]观测结束时间(7个字节UnsignedByte)
    Calibration: u8, //标校状态【1】 0=无标校 1=自动标校 2=一星期内人工标校 3=一月内人工标校 4=开机自动标校 其他码不用
    LayerNum: u8, //扫描层数【1】 注：当扫描方式是PPI或RHI时，该参数无效；对于382雷达，PPI扫描时为5
    #[br(count = 25)]
    LayerInfo: Vec<LAYERPARAM_048>, //层参数结构（各层扫描状态设置）【475】

    //注：对于346雷达，当扫描方式是PPI或RHI时，只在第一个元素中填写，其余为0；对于382雷达，当扫描方式是PPI或RHI时，只在前五个元素中填写，其余为0。
    RHIA: u16, //做RHI时的所在方位角，计数单位为1/100度，做PPI和立体扫描时为65535【2】
    RHIL: u16, //做RHI时的最低仰角，计数单位为1/100度，做其他扫描时为-32768【2】
    RHIH: u16, //做RHI时的最高仰角，计数单位为1/100度，做其他扫描时为-32768【2】
    #[br(count = 48)]
    Reserved: Vec<u8>, //保留【24】
}

/**
 *
 * 雷达层参数【19】
 */
#[derive(Debug, BinRead)]
struct LAYERPARAM_048 {
    Ambiguousp: u8, //本层退模糊状态【1】  0=无退模糊状态 1=软件退模糊 2=双T退模糊 3=批式退模糊 4=双T+软件退模糊 5=批式+软件退模糊 6=双PPI退模糊 9=其他方式
    PRF1: u16,      //本层第一脉冲重复频率，计数单位为1/10Hz【2】
    PRF2: u16,      //本层第二脉冲重复频率，计数单位为1/10Hz【2】
    Filter: u8,     //滤波器代号【1】
    PluseW: u16,    //本层的脉冲宽度，计数单位为微秒【2】
    MaxV: u16,      //本层的最大可测速度，计数单位为厘米/秒【2】
    MaxL: u16,      //本层的最大可测距离，以10米为计数单位【2】
    BinWidth: u16,  //本层数据的库长，以1/10米为计数单位【2】
    BinNumber: u16, //本层扫描的库数【2】
    RecodeNumber: u16, //本层扫描径向个数【2】
    DataForm: u8, //本层径向中的数据排列方式：【1】 11 单要素排列 ConZ 12 单要素排列 UnZ 21 按要素排列 ConZ+UnZ 22 按要素排列 ConZ+V+W 23 按要素排列 UnZ+V+W 24 按要素排列 ConZ+UnZ+V+W
}

/**
 * 数据记录【4144】
 * 数据记录存放探测的回波数据，以极坐标方式有序排列，其结构如下
 */
#[derive(Debug, BinRead)]
struct LineDataBlock {
    // #[br(seek_before = SeekFrom::Start(4144))]
    Elev: u16, //仰角，计数单位1/100度 【2】
    Az: u16,   //方位，计数单位1/100度 【2】
    #[br(count = 16)]
    Longtitude: Vec<u8>, //经度，以字符串记录【16】
    #[br(count = 16)]
    Latitude: Vec<u8>, //纬度，以字符串记录【16】
    Vs: f32,   //纵摇，计数单位【4】
    Hs: f32,   //横摇，计数单位【4】
    Course: i16, //航向，计数单位【2】
    Nv: i16,   //舰速，计数单位【2】
    #[br(count = 1024)]
    CorZ: Vec<u8>, //经过地物杂波消除的dBZ值=（CorZ-64）/2【1024】
    #[br(count = 1024)]
    UnZ: Vec<u8>, //不经过地物杂波消除的dBZ值=（UnZ-64）/2【1024】
    #[br(count = 1024)]
    V: Vec<u8>, //速度值，计数单位为最大可测速度的1/127【1024】
    //正值表示远离雷达的速度，负值表示朝向雷达的速度
    //无回波时计-128
    #[br(count = 1024)]
    W: Vec<u8>, //谱宽值，计数单位为最大可测速度的1/512【1024】
                //无回波时计零
}
//  1、数据块的具体内容由层参数结构中的DataForm决定；
//  2、对于382雷达，CorZ、UnZ、V、W四个参数只有前面的1000个字节有效。

impl Radar386Reader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        let mut file = File::open(fname)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let mut cursor = Cursor::new(&mut buf);
        let rad: Radar386Info = BinRead::read(&mut cursor)?;

        dbg!(
            &rad.observe.LayerInfo[0].BinNumber,
            &rad.observe.LayerInfo[0].BinWidth
        );
        Ok(Self {
            info: rad,
            data: buf,
            fname: String::from(fname),
        })
    }
}

impl TryInto<RadialData> for Radar386Reader {
    type Error = MetError;
    fn try_into(self) -> Result<RadialData, Self::Error> {
        let fnames: Vec<&str> = self.fname.split('.').collect();

        let dt: &str = &fnames[0];
        let dt = &dt[2..];
        let (date, time) = dt.split_at(8);
        let start_date = String::from(date);
        let start_time = String::from(time);
        //
        let end_time = String::from(time);

        let pt = &fnames[1];
        let pt = &pt[2..];
        let product = String::from(pt);
        let mut props = HashMap::new();
        props.insert(String::from("province"), "UNKNOWN".to_string());
        props.insert("area".to_string(), "UNKNOWN".to_string());

        let observe = &self.info.observe;
        let mut data = &self.data[54 + 65 + 475..];
        let mut data_cursor = Cursor::new(&mut data);

        dbg!(&observe.LayerNum);

        let mut lon = 0.0;
        let mut lat = 0.0;
        let mut eles = Vec::new();
        let mut azs = Vec::new();
        let mut rs = Vec::new();
        let mut data = Vec::new();
        let mut vol_ref = Vec::new();
        for info in observe.LayerInfo.iter() {
            // dbg!(info.RecodeNumber,info.BinNumber,info.BinWidth);
            let BinNumber = info.BinNumber;
            let BinWidth = info.BinWidth as f32 * 0.01; //米

            let rn = info.RecodeNumber;
            let mut first = true;
            let mut el_az = Vec::new();

            let mut az_range = Vec::new();
            let mut az_ref = Vec::new();

            for layeridx in 0..rn {
                let data_line: LineDataBlock = BinRead::read(&mut data_cursor)?;

                println!("layer idx {}   {:?}", layeridx, data_line.CorZ);
                // lon = *&data_line.Longtitude as f32 * 0.01;
                // lat = *&data_line.Latitude as f32 * 0.01;
                // if layeridx == 0 {
                //     // for (ii,ddd ) in data_line.CorZ.iter().enumerate() {
                //     //     if *ddd!=0 {
                //     //         println!("idx {}  {}",ii, ddd);
                //     //     }
                //     // }
                //     // let rt = data_line.CorZ.iter().filter(|&d| *d!=0).collect::<Vec<_>>();
                //     // println!("{:?}",rt);
                // }
                let elv: f32 = *&data_line.Elev as f32 * 0.01;
                let az: f32 = *&data_line.Az as f32 * 0.01;
                if first {
                    let lon_str = String::from_utf8_lossy(&data_line.Longtitude);
                    let lon_str = lon_str.trim_end_matches('\u{0}');
                    let lon_str = lon_str.trim_left_matches('E');
                    let lon_str = lon_str.split('/').collect::<Vec<_>>();
                    lon = f32::from_str(lon_str[0]).unwrap()
                        + (f32::from_str(lon_str[1]).unwrap()
                            + f32::from_str(lon_str[2]).unwrap() / 60.0)
                            / 60.0;

                    let lat_str = String::from_utf8_lossy(&data_line.Latitude);
                    let lat_str = lat_str.trim_end_matches('\u{0}');
                    let lat_str = lat_str.trim_left_matches('N');
                    let lat_str = lat_str.split('/').collect::<Vec<_>>();
                    lat = f32::from_str(lat_str[0]).unwrap()
                        + (f32::from_str(lat_str[1]).unwrap()
                            + f32::from_str(lat_str[2]).unwrap() / 60.0)
                            / 60.0;

                    // dbg!(lon_str, lat_str, lon, lat);
                    eles.push(elv);
                    first = false;
                }
                el_az.push(az);
                let mut bin_data = Vec::new();
                let mut ranges = Vec::new();
                for bin in 0..BinNumber {
                    let binidx = bin as f32 * 1024.0 / BinNumber as f32;
                    let binidx = binidx.floor();
                    let binidx = binidx as usize;
                    let bd = data_line.CorZ[binidx as usize];
                    // println!("{:?}",binidx);
                    if bd == 255 {
                        bin_data.push(crate::MISSING);
                    } else {
                        // let bd = (bd as f32 - 64.0) * 0.5;
                        let bd = (bd as f32) * 0.5;
                        bin_data.push(bd);
                    }
                    let r = (binidx as f32 * BinWidth) as f64;
                    ranges.push(r);
                }
                az_range.push(ranges);
                az_ref.push(bin_data);
            }
            rs.push(az_range);
            vol_ref.push(az_ref);
            azs.push(el_az);
        }
        data.push(vol_ref); // 反射率

        Ok(RadialData {
            eles: eles,
            azs: azs,
            rs: rs,
            data: data,
            start_time,
            start_date,
            elements: vec!["Z".to_string()],
            lon: lon,
            lat: lat,
            height: 0.0,
            props,
        })
    }
}
