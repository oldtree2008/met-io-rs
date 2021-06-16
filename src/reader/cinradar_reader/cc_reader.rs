#![allow(non_snake_case)]
use crate::MetError;
use crate::{RadialData, SingleGrid, ToGrids};
use binread::prelude::*;
use encoding_rs::*;
use rayon::prelude::*;
use std::io::{Cursor, Read, SeekFrom};
use std::mem::*;
use std::path::Path;

#[derive(Debug, BinRead)]
struct Head {
    #[br(count = 16)]
    FileType: Vec<u8>,
    #[br(count = 30)]
    Country_: Vec<u8>,
    #[br(calc=GBK.decode(&Country_).0.trim_end_matches('\u{0}').to_string())]
    Country: String,
    #[br(count = 20)]
    Province_: Vec<u8>,
    #[br(calc=GBK.decode(&Province_).0.trim_end_matches('\u{0}').to_string())]
    Province: String,
    #[br(count = 40)]
    Station_: Vec<u8>,
    #[br(calc=GBK.decode(&Station_).0.trim_end_matches('\u{0}').to_string())]
    Station: String,
    #[br(count = 10)]
    StationNumber_: Vec<u8>,
    //#[br(calc=StationNumber_.to_string())]
    //StationNumber: String,
    #[br(count = 20)]
    RadarType_: Vec<u8>,
    #[br(calc=GBK.decode(&RadarType_).0.trim_end_matches('\u{0}').to_string())]
    RadarType: String,
    #[br(count = 16)]
    Longitude_: Vec<u8>,
    #[br(calc=GBK.decode(&Longitude_).0.trim_end_matches('\u{0}').to_string())]
    Longitude: String,
    #[br(count = 16)]
    Latitude_: Vec<u8>,
    #[br(calc=GBK.decode(&Latitude_).0.trim_end_matches('\u{0}').to_string())]
    Latitude: String,
    LongitudeValue_: i32,
    #[br(calc=LongitudeValue_ as f32 /100.0)]
    LongitudeValue: f32,
    LatitudeValue_: i32,
    #[br(calc=LatitudeValue_ as f32 /100.0)]
    LatitudeValue: f32,
    Height_: i32,
    #[br(calc=Height_ as f32 /1000.0)]
    Height: f32, //单位为米
    MaxAngle_: i16, // 测站四周地物最大遮挡仰角，以1/100度为计数单位
    #[br(calc=MaxAngle_ as f32 /100.0)]
    MaxAngle: f32,
    OptiAngle_: i16, // 测站四周地物最大遮挡仰角，以1/100度为计数单位
    #[br(calc=OptiAngle_ as f32 /100.0)]
    OptiAngle: f32, //测站的最佳观测仰角（地物回波强度<10dBZ），以1/100度为计数单位
    //MangFreq: i16,
    ucSYear1: u8,
    ucSYear2: u8,
    ucSMonth: u8,
    ucSDay: u8,
    ucSHour: u8,
    ucSMinute: u8,
    ucSSecond: u8,
    ucTimeFrom: u8,
    ucEYear1: u8,
    ucEYear2: u8,
    ucEMonth: u8,
    ucEDay: u8,
    ucEHour: u8,
    ucEMinute: u8,
    ucESecond: u8,
    ucScanMode: u8,
    ulSmilliSecond: u32,
    usRHIA: u16,
    usRHIL: u16,
    usRHIH: u16,
    usEchoType: u16,
    usProdCode: u16,
    ucCalibration: u8,
    #[br(count = 3)]
    remain: Vec<u8>,
}

#[derive(Debug, BinRead)]
struct PerformanceInfo {
    AntennaG: u32, // 天线增益以0.001dB为计数单位
    VerBeamW: u16, //垂直波束宽度，以1/100度为计数单位
    HorBeamW: u16, //水平波束宽度，以1/100度为计数单位
    Polarizations: u8,
    /*    偏振状况                                    （188）
        0=水平
        1=垂直
        2=双线偏振
        3=圆偏振
        4=其他
    */
    SideLobe: u16,    // 第一旁瓣，以0.01dB为计数单位
    Power: i32,       //           雷达脉冲峰值功率，以瓦为单位
    WaveLength: i32,  //波长，以微米为计数单位
    LogA: u16,        //       对数接收机动态范围，以以0.01dB为计数单位
    LineA: u16,       //       线性接收机动态范围，以以0.01dB为计数单位
    AGCP: u16,        //       AGC延迟量，以微秒为计数单
    LogMinPower: u16, // 对数接收机最小可测功率，计数单位为0.01dBm
    LinMinPower: u16, // 线性接收机最小可测功率，计数单位为0.01dBm
    ClutterT: u8,     //     杂波消除阈值，计数单位为0.01dB
    VelocityP: u8,    /*     速度处理方式                                （210）*/
}
#[derive(Debug, BinRead)]
struct ObservationInfo {
    Stype: u8,   /*         扫描方式                                   （216）
                 1=RHI
                 10=PPI
                 1xx=VOL，xx为扫描层数 */
    Syear: u16,  //    观测记录开始时间的年（1000-
    SMonth: u8,  //        观测记录开始时间的月（1-12
    SDay: u8,    //          观测记录开始时间的日（1-31）
    SHour: u8,   //         观测记录开始时间的时（00-23）
    SMinute: u8, //       观测记录开始时间的分（00-59）
    SSecond: u8, //       观测记录开始时间的秒（00-59
    TimeP: u8,   /*         时间来源                                   （224）
                 0=计算机时钟，但一天内未进行对时
                 1=计算机时钟，但一天内进行了对时
                 2=GPS
                 3=其他 */
    SMillisecond: u32, //  秒的小数位（计数单位为微秒）          （255-228
    Calibration: u8,   /*      标校状态                                  （229）
                       0=无标校
                       1=自动标校
                       2=一星期内人工标校
                       3=一月内人工标校
                       其它码不用*/
    IntensityI: u8, //        强度积分次数（32-128）
    VelocityP: u8,  //        速度处理样本（31-255）（样本数减1）

    #[br(count = 30)]
    layer_info: Vec<LayerInfo>,
    RHIA: u16,  //          做RHI时的所在方位角，计数单位为1/100度，做PPI和立体扫描时为65535
    RHIL: i16,  //                 做RHI时的最低仰角，计数单位为1/100度，做其他扫描时为-32768
    RHIH: i16,  //                做RHI时的最高仰角，计数单位为1/100度，做其他扫描时为-32768
    Eyear: u16, //         观测记录结束时间的年（1000-）
    EMonth: u8, //        观测记录结束时间的月（1-12）
    EDay: u8,   //          观测记录结束时间的日（1-31）
    EHour: u8,  //         观测记录结束时间的时（00-23）
    EMinute: u8, //       观测记录结束时间的分（00-59）
    ESecond: u8, //       观测记录结束时间的秒（00-59
    ETenth: u8, //         观测记录结束时间的1/100秒（00-99
}

#[derive(Debug, BinRead)]
#[br(little)]
struct LayerInfo {
    Ambiguousp: u8, /*     本层退速度模糊状态                           （1）
                    0=无退速度模糊状态
                    1=软件退速度模糊
                    2=双T退速度模糊
                    3=批式退速度模糊
                    4=双T+软件退速度模糊
                    5=批式+软件退速度模糊
                    6=双PPI退速度模糊
                    9=其它方式*/
    Arotate: u16, //          本层天线转速，计数单位为0.01度/秒，当扫描方式为RHI或PPI时，只在第一个元素中填写，其它元素为0
    PRF1: u16,    //          本层第一脉冲重复频率，计数单位：1/10Hz
    PRF2: u16,    //         本层第二脉冲重复频率，计数单位：1/10Hz
    PulseW: u16,  //        本层脉冲的宽度，计数单位为微秒
    MaxV: u16,    //          本层的最大可测速度，计数单位为厘米/秒
    MaxL: u16,    //         本层的最大可测距离，以10米为计数单位
    // ZbinWidth: u16, //      本层强度数据的库长，以1/10米为计数单位
    binWidth: u16, //      本层速度数据的库长，以1/10米为计数单位
    // WbinWidth: u16, //      本层谱宽数据的库长，以1/10米为计数单位
    binNumber: u16, //     本层扫描强度径向的库数
    // VbinNumber: u16, //     本层扫描速度径向的库数
    // WbinNumber: u16, //     本层扫描谱宽径向的库数
    RecordNumber: u16, //     本层扫描径向个数
    SwpAngles: i16, //                本层的仰角，计数单位为1/100度，当扫描方式为RHI，不填此数组，当扫描方式为PPI时，第一个元素为做PPI时的仰角，计数单位为1/100，其它元素填-32768
}
pub struct CCReader;

impl CCReader {
    pub fn new(data: &[u8]) -> Result<Self, MetError> {
        // let mut f = File::open(fname)?;
        // let mut data = Vec::new();
        // f.read_to_end(&mut data)?;
        let mut cursor = Cursor::new(&data);
        let p: Head = cursor.read_le()?;
        dbg!(&p);

        Ok(CCReader)
    }
}
