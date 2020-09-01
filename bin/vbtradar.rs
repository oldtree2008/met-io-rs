#![allow(non_snake_case)]
use binread::prelude::*;
use binread::NullString;
use encoding_rs::*;
use image::{imageops, GenericImageView, ImageBuffer, RgbaImage};
use kdtree::distance::squared_euclidean;
use kdtree::ErrorKind;
use kdtree::KdTree;
use met_io_rs::*;
use palette::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::*;
use std::io::{Cursor, Read, SeekFrom};
use std::mem::*;
use std::path::Path;

#[derive(Debug, BinRead)]
struct Product {
    // file_info: FileInfo,
    site_info: SiteInfo,
    performance_info: PerformanceInfo,
    observation_info: ObservationInfo,
}

#[derive(Debug, BinRead)]
struct FileInfo {
    #[br(pad_size_to = 4)]
    FileId: NullString,
    VersionNo: f32,
    FileHeaderLength: u32,
}
#[derive(Debug, BinRead)]
struct SiteInfo {
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
    #[br(calc=GBK.decode(&StationNumber_).0.trim_end_matches('\u{0}').to_string())]
    StationNumber: String,
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
    MangFreq: i16,
}

#[derive(Debug, BinRead)]
struct PerformanceInfo {
    AntennaG: u32, // 天线增益以0.001dB为计数单位
    VerBeamW: u16, //垂直波束宽度，以1/100度为计数单位
    HorBeamW: u16, //水平波束宽度，以1/100度为计数单位
    Polarizations: u8, /*    偏振状况                                    （188）
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
    VelocityP: u8,    /*     速度处理方式                                （210）
                                        0=无速度处理
                                        1=PPP
                                        2=FFT */
                      // FilterP: u8, /*     地物杂波消除方式                             （211）
                      //              0=无杂波消除
                      //              1=地物杂波图扣除法
                      //              2=地物杂波图+滤波器处理
                      //              3=滤波器处理
                      //              4=谱分析处理
                      //              5=其他处理法 */
                      // NoiseT: u8, //       噪声消除阈值（0-255）
                      // SQIT: u8,   //       SQI，以0.01为计数单位
                      // IntensityC: u8, /*    RVP强度值估算采用的通道                     （214）
                      //             1=对数通道
                      //             2=线性通道 */
                      // IntensityR: u8, /*    强度估算是否进行了距离订正                   （215）
                      //                 0=无
                                      // 1=已进行了距离订正 */
}

#[derive(Debug, BinRead)]
struct ObservationInfo {
    Stype: u8,   /*         扫描方式                                   （216）
                 1=RHI
                 10=PPI
                 1xx=VOL，xx为扫描层数 */
    Syear: u16,  //    观测记录开始时间的年（2000-
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
    // ZStartBin: u16, //       强度有效数据开始库数
    // VStartBin: u16, //       速度有效数据开始库数
    // WStartBin: u16, //       谱宽有效数据开始库数
    #[br(count = 30)]
    layer_info: Vec<LayerInfo>,
    RHIA: u16,  //          做RHI时的所在方位角，计数单位为1/100度，做PPI和立体扫描时为65535
    RHIL: i16,  //                 做RHI时的最低仰角，计数单位为1/100度，做其他扫描时为-32768
    RHIH: i16,  //                做RHI时的最高仰角，计数单位为1/100度，做其他扫描时为-32768
    Eyear: u16, //         观测记录结束时间的年（2000-）
    EMonth: u8, //        观测记录结束时间的月（1-12）
    EDay: u8,   //          观测记录结束时间的日（1-31）
    EHour: u8,  //         观测记录结束时间的时（00-23）
    EMinute: u8, //       观测记录结束时间的分（00-59）
    ESecond: u8, //       观测记录结束时间的秒（00-59
    ETenth: u8, //         观测记录结束时间的1/100秒（00-99
                // ZBinByte: u8, //      原始强度数据中库长无变化填0            （1372-1373）原始强度数据中库长有变化填占用的字节数
                // #[br(if (ZBinByte!=0), count=5)]
                // ZBinRange: Option<Vec<BinRange>>,
                // VBinByte: u8, //      原始强度数据中库长无变化填0            （1372-1373）原始强度数据中库长有变化填占用的字节数
                // #[br(if (VBinByte!=0), count=5)]
                // VBinRange: Option<Vec<BinRange>>,
                // WBinByte: u8, //      原始强度数据中库长无变化填0            （1372-1373）原始强度数据中库长有变化填占用的字节数
                // #[br(if (WBinByte!=0), count=5)]
                // WBinRange: Option<Vec<BinRange>>,
}

#[derive(Debug, BinRead)]
#[br(little)]
struct LayerInfo {
    // DataType: u8,
    /*       本层观测要素                                  （0）
    1=单要素
    2=三要素单PRF
    3=三要素双PRF
    4=双线偏振
    5=双线偏振多普勒
    6=双波长（不同天线）
    7=双波长（共用天线）*/
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
    VbinWidth: u16, //      本层速度数据的库长，以1/10米为计数单位
    // WbinWidth: u16, //      本层谱宽数据的库长，以1/10米为计数单位
    ZbinNumber: u16, //     本层扫描强度径向的库数
    // VbinNumber: u16, //     本层扫描速度径向的库数
    // WbinNumber: u16, //     本层扫描谱宽径向的库数
    RecordNumber: u16, //     本层扫描径向个数
    SwpAngles: i16, //                本层的仰角，计数单位为1/100度，当扫描方式为RHI，不填此数组，当扫描方式为PPI时，第一个元素为做PPI时的仰角，计数单位为1/100，其它元素填-32768
                    // DataForm: i8,
                    /*               本层径向中的数据排列方式：                   （30）
                        11 单要素排列：CorZ
                        12 单要素排列：UnZ
                        13 单要素排列：V
                        14 单要素排列：W
                        21 按要素排列：CorZ+UnZ
                        22 按要素排列：CorZ+V+W
                        23 按要素排列：UnZ+V+W
                        24 按要素排列：CorZ+UnZ+V+W   //单发
                        4x 双偏振按要素排列模式
                        48 CorZ+UnZ+V+W+ZDR+PHDP+KDP+LDRH+ROHV
                        6x 双偏振多普勒按要素排列模式
                    60  CorZ+UnZ+V+W+ZDR      //单发
                    61  CorZ+V+W+ZDR+PHDP+LDRH+ROHV+KDP  //交替
                    62  CorZ+V+W+ZDR+PHDP+ROHV+KDP   //双发
                    8x 双波长按要素排列方式*/
                    // Dbegin: u32, //       本层数据纪录开始位置（字节数）

                    // #[br(seek_before = SeekFrom::Start(Dbegin as u64),count=1)]
                    // records:Vec<Record>,
}
#[derive(Debug, BinRead)]
struct BinRange {
    Code: i16,      //                   强度变库长结构代码
    Begin: i16,     //                  开始库的距离，以10米为计数单位
    End: i16,       //                   结束库的距离，以10米为计数单位
    BinLength: i16, //               库长，以1/10米为计数单位
}

#[derive(Debug, BinRead)]
// #[br(import(cn:u32))]
struct Record {
    Elev: i16, //        仰角，计数单位为1/100度
    #[br(if(Elev != -32768))]
    recorder: Option<RecordWrapper>,
}

#[derive(Debug, BinRead)]
struct RecordWrapper {
    Az: u16,  //            方位，计数单位为1/100度
    Hh: u8,   //            时
    Hm: u8,   //           分
    Hs: u8,   //            秒
    Min: u32, //        秒的小数（计数单位为微秒）

    #[br(count = 1000)]
    CorZ: Vec<u8>, //[* n为文件头中体扫各对应层（或PPI、RHI开始层）对应的强度、速度、谱宽的各个径向的库数。]*       经过杂波消除的dBZ值=（CorZ-64）/2
    // #[br(args(recordNumber),count=recordNumber)]
    #[br(count = 1000)]
    UnZ: Vec<u8>,
    // #[br(args(recordNumber),count=recordNumber)]
    #[br(count = 1000)]
    V: Vec<i8>,
    // #[br(args(recordNumber),count=recordNumber)]
    #[br(count = 1000)]
    W: Vec<u8>,
    // #[br(args(recordNumber),count=recordNumber)]
    #[br(count = 1000)]
    ZDR: Vec<i16>,
    // #[br(args(recordNumber),count=recordNumber)]
    #[br(count = 1000)]
    PHDP: Vec<i16>,
    // #[br(args(recordNumber),count=recordNumber)]
    #[br(count = 1000)]
    KDP: Vec<i16>,
    // #[br(args(recordNumber),count=recordNumber)]
    #[br(count = 1000)]
    LDRH: Vec<i16>,
    // #[br(args(recordNumber),count=recordNumber)]
    #[br(count = 1000)]
    ROHV: Vec<i16>,
}

pub struct Vol {
    pub vol_azimuth: Vec<Vec<u16>>,
    pub vol_range: Vec<Vec<u16>>,
    pub vol_value: Vec<Vec<Vec<u8>>>,
    pub fix_elevation: Vec<i16>,
}

impl Vol {
    pub fn new(
        azimuth: HashMap<i16, Vec<u16>>,
        range: HashMap<i16, Vec<u16>>,
        value: HashMap<i16, HashMap<u16, Vec<u8>>>,
    ) -> Self {
        let mut vol_azimuth = Vec::new();
        let mut keys: Vec<&i16> = azimuth.keys().collect();
        keys.sort();
        println!("{:?}", keys);

        for k in keys.iter() {
            let azs = &azimuth[k];
            let mut azss: Vec<u16> = Vec::new();
            azss.extend(azs);
            azss.sort();
            println!("{:?}", azss);
            vol_azimuth.push(azss);
        }

        let mut fix_elevation = Vec::new();
        fix_elevation.extend(keys);

        let mut vol_range = Vec::new();
        let mut keys: Vec<&i16> = range.keys().collect();
        keys.sort();
        println!("{:?}", keys);
        for k in keys.iter() {
            let r = &range[k];
            let mut rs: Vec<u16> = Vec::new();
            rs.extend(r);
            rs.sort();
            vol_range.push(rs);
        }
        let mut vol_value = Vec::new();
        let mut keys: Vec<&i16> = value.keys().collect();
        keys.sort();
        println!("{:?}", keys);
        for k in keys.iter() {
            let am = &value[k];
            let akeys: Vec<&u16> = am.keys().collect();
            let mut makeys: Vec<u16> = Vec::new();
            makeys.extend(akeys);
            makeys.sort();
            let mut avs = Vec::new();
            for ak in makeys.iter() {
                let al: &Vec<u8> = &am[ak];
                let mut mal = Vec::new();
                mal.extend(al);
                avs.push(mal);
            }
            vol_value.push(avs);
        }

        Vol {
            vol_azimuth,
            vol_range,
            vol_value,
            fix_elevation,
        }
    }
}

pub fn main() {
    //VTB20180521065950.011   VTB20180521225200.011
    // let fname = "h:/data/VTB20180521065344.011";
    let fname = "h:/data/VTB20180521065950.011";
    let fname = r##"/mnt/e/青海数据/青海数据/青海数据/201707012259490.05V"##;
    // let fname = "h:/data/20200704_164546.00.002.001_R1";

    let p = Path::new("palette/REFcolortable.xml");
    let pal = Palette::new_with_file(&p).unwrap();
    // let fname = "h:/data/VTB20180521225200.011";
    let mut f = File::open(fname).unwrap();
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();
    let mut cursor = Cursor::new(&data);
    let p: Product = cursor.read_le().unwrap();
    dbg!(&p);

    dbg!(size_of::<Product>());

    // let mut data = &data[464..];
    // dbg!(&data[0..20]);
    // let mut cursor = Cursor::new(&data);
    // let step:u8 = cursor.read_le().unwrap();
    for _ in (0..998) {
        let step: u8 = cursor.read_le().unwrap();
        dbg!(step);
    }

    println!("-------------------------------");
    for _ in (0..998) {
        let step: u8 = cursor.read_le().unwrap();
        dbg!(step);
    }
    // let startaz:u16 = cursor.read_le().unwrap();
    // let startel:u16 = cursor.read_le().unwrap();
    // let endaz:u16 = cursor.read_le().unwrap();
    // let endel:u16 = cursor.read_le().unwrap();

    // dbg!(startaz,startel,endaz,endel);

    // dbg!(data.len());
    // let first_layer = &p.observation_info.layer_info[0];
    // dbg!(first_layer);
    // let pos = first_layer.Dbegin as usize;
    // let recordNumber = first_layer.RecordNumber;
    // let zbinNum = first_layer.ZbinNumber;

    // let mut kdtree = KdTree::new(3);

    // let mut vol_azimuth: HashMap<i16, Vec<u16>> = HashMap::new();
    // let mut vol_range: HashMap<i16, Vec<u16>> = HashMap::new();
    // let mut vol_value: HashMap<i16, HashMap<u16, Vec<u8>>> = HashMap::new();
    // let mut elv_az = Vec::new();
    // let mut elv_az_range = Vec::new();
    // let mut elv_az_range_value = Vec::new();
    // let mut elvs = Vec::new();
    // for (i, l) in p.observation_info.layer_info.iter().enumerate() {
    //     // let l = &p.observation_info.layer_info[10];
    //     if l.Dbegin == 0 {
    //         continue;
    //     }
    //     let pos = l.Dbegin as usize;
    //     println!("{}-----------------------------------", i);
    //     let mut azs = Vec::new();
    //     let mut az_ranges = Vec::new();
    //     let mut az_ranges_value = Vec::new();
    //     let mut first = false;
    //     for r in 0..recordNumber {
    //         let p = pos + (r * (zbinNum * 4 + zbinNum * 5 * 2 + 11)) as usize;
    //         let mut cursor = Cursor::new(&data[p..]);
    //         let record = Record::read(&mut cursor).unwrap();
    //         let Elev = record.Elev;
    //         if Elev == -32768 {
    //             break;
    //         }

    //         let Elev = Elev as f32 / 100.0;
    //         dbg!(Elev);
    //         if !first {
    //             elvs.push(Elev);
    //             first = true;
    //         }

    //         let rec = record.recorder.unwrap();
    //         let Az = rec.Az;
    //         let Az = rec.Az as f32 / 100.0;
    //         dbg!(Az);

    //         let h = 0.0;
    //         let CorZ = rec.CorZ;

    //         println!("h {} m {}  s {}", rec.Hh, rec.Hm, rec.Hs);
    //         if rec.Hh > 24 {
    //             break;
    //         }
    //         let mut ranges = Vec::new();
    //         azs.push(Az);
    //         for i in (0..1000) {
    //             ranges.push(i);
    //         }
    //         az_ranges.push(ranges);

    //         az_ranges_value.push(CorZ);
    //     }
    //     elv_az.push(azs);
    //     elv_az_range.push(az_ranges);
    //     elv_az_range_value.push(az_ranges_value);
    // }

    // dbg!(&elvs);

    // let h = 0.0;
    // let elevation = 4.0;
    // let z = 520.0;
    // let res = 150.0;
    // let R: usize = 500;
    // let W = 2 * R;
    // let mut grid_value = vec![-999.0; 2 * R * 2 * R];
    // let elv_azs = &elv_az[4]; // 第一层上的所有方位角
    // let elv_values = &elv_az_range_value[4];
    // for v in elv_az_range_value.iter() {
    // println!("{:?}", &elv_az_range_value[8]);
    // }

    // fn find_index(azs: &Vec<f32>, az: f32) -> Option<usize> {
    //     let az_len = azs.len();
    //     for (i, a) in azs[0..az_len - 1].iter().enumerate() {
    //         if az >= azs[i] && az < azs[i + 1] {
    //             return Some(i);
    //         }
    //     }
    //     None
    // }

    // grid_value.iter_mut().enumerate().for_each(|(i, d)| {
    //     let y = i / (2 * R);
    //     let x = i % (2 * R);
    //     let x = x as f32 - 500.0;
    //     let y = y as f32 - 500.0;

    //     let x = x * res;
    //     let y = y * res;
    //     //for ppz
    //     // let (_, _, z) = transforms::cartesian_to_antenna_cwr(x, -y, elevation, h);
    //     let (az, rang, elv) = transforms::cartesian_xyz_to_antenna(x, y, z, h);
    //     // if rang>6600.0 && rang<8600.0 {
    //         // println!("elv {} az {} range {} x {} y {}",elv,az,rang,x,y);
    //     // }
    //     // let mut az_found = false;
    //     if rang < 500.0 * res {
    //         let rang_idx = rang / res;
    //         let elv_idx = find_index(&elvs, elv);
    //         if let Some(elv_idx) = elv_idx {
    //             let elv_azs = &elv_az[elv_idx];
    //             let elv_values = &elv_az_range_value[elv_idx];
    //             // println!(
    //             //     "elv_idx {:?}   elv {}  az {}  rang {} x {} y {}",
    //             //     elv_idx, elv, az, rang, x, y
    //             // );
    //             let idx = find_index(&elv_azs, az);
    //             let mut v1 = 0.0;
    //             let mut v2 = 0.0;
    //             if let Some(ii) = idx {
    //                 let az0 = elv_azs[ii];
    //                 let az1 = elv_azs[ii + 1];
    //                     // println!(
    //                     //     "elv_idx {:?}   elv {} az0 {} az {} az1 {} rang {} x {} y {}",
    //                     //     elv_idx, elv, az0,az, az1,rang, x, y
    //                     // );
    //                 // }
    //                 let rang0 = rang_idx.floor() as usize;
    //                 let rang1 = rang_idx.ceil() as usize;
    //                 let mut v00 = elv_values[ii][rang0] as f32;
    //                 let mut v01 = elv_values[ii][rang1] as f32;
    //                 let mut v10 = elv_values[ii + 1][rang0] as f32;
    //                 let mut v11 = elv_values[ii + 1][rang1] as f32;
    //                 if v00 == 255.0 {
    //                     v00 = 999.0
    //                 }
    //                 if v01 == 255.0 {
    //                     v01 = 999.0
    //                 }
    //                 if v10 == 255.0 {
    //                     v10 = 999.0
    //                 }
    //                 if v11 == 255.0 {
    //                     v11 = 999.0
    //                 }
    //                 let v = met_io_rs::interplate::interp_ppi(
    //                     az,
    //                     rang_idx,
    //                     az0,
    //                     az1,
    //                     rang0 as f32,
    //                     rang1 as f32,
    //                     v00 as f32,
    //                     v01 as f32,
    //                     v10 as f32,
    //                     v11 as f32,
    //                 );
    //                 let v = (v - 64.0) / 2.0;
    //                 // println!("elv {} az {} rang {} x {} y {} v {}", elv, az,rang, x, y, v1);
    //                 println!(
    //                 "x {} y {} elv {} az {} az0 {} az1 {} range {} range0 {} range1 {}  v00 {}  v01 {} v10 {} v11 {} v {}",
    //                 x,
    //                 y,
    //                 elv,
    //                 az,
    //                 &elv_azs[ii],
    //                 &elv_azs[ii + 1],
    //                 rang_idx,
    //                 rang0,
    //                 rang1,
    //                 v00,
    //                 v01,
    //                 v10,
    //                 v11,
    //                 v
    //             );
    //             *d = v;
    //             }
    //         }
    //     }
    // });

    // grid_value.iter_mut().enumerate().for_each(|(i, d)| {
    //     let y = i / (2 * R);
    //     let x = i % (2 * R);
    //     let x = x as f32 - 1000.0;
    //     let y = y as f32 - 1000.0;
    //     // println!("x {} y {}",x,y);
    //     let dst = (x * x + y * y).sqrt();

    //     //for ppi
    //     let (az, rang, elv) = transforms::cartesian_to_antenna_cwr(x * 1500.0, -y * 1500.0, elevation, h);
    //     //for ppz
    //     // let (_, _, z) = transforms::cartesian_to_antenna_cwr(x, -y, elevation, h);
    //     // let (az, rang, elv) = transforms::cartesian_xyz_to_antenna(x, -y, z, h);
    //     // println!("elv {}",elv);
    //     let mut az_found = false;
    //     if rang < 999.0 * 1500.0
    //     /*&& x>-50.0 && x<50.0  && y>-50.0 && y<50.0*/
    //     {
    //         // let elv_idx = find_index(&elvs, elv);
    //         // if let Some(elv_idx) = elv_idx {
    //         //     let elv_azs = &elv_az[elv_idx]; // 第一层上的所有方位角
    //         //     let elv_values = &elv_az_range_value[elv_idx];
    //             // println!("elv_idx {:?}   elv {}", elv_idx, elv);

    //             let az = az.to_degrees();
    //             let idx = find_index(&elv_azs, az);
    //             if let Some(ii) = idx {
    //                 let az0 = elv_azs[ii];
    //                 let az1 = elv_azs[ii + 1];
    //                 let rang0 = (rang/1500.0).floor() as usize;
    //                 let rang1 = (rang/1500.0).ceil() as usize;
    //                 let mut v00 = elv_values[ii][rang0] as f32;
    //                 let mut v01 = elv_values[ii][rang1] as f32;
    //                 let mut v10 = elv_values[ii + 1][rang0] as f32;
    //                 let mut v11 = elv_values[ii + 1][rang1] as f32;
    //                 if v00 == 255.0 {
    //                     v00 = 999.0
    //                 }
    //                 if v01 == 255.0 {
    //                     v01 = 999.0
    //                 }
    //                 if v10 == 255.0 {
    //                     v10 = 999.0
    //                 }
    //                 if v11 == 255.0 {
    //                     v11 = 999.0
    //                 }
    //                 let v = radar_grid::interp_ppi(
    //                     az,
    //                     rang/1500.0,
    //                     az0,
    //                     az1,
    //                     rang0 as f32,
    //                     rang1 as f32,
    //                     v00 as f32,
    //                     v01 as f32,
    //                     v10 as f32,
    //                     v11 as f32,
    //                 );
    //                 let v = (v - 64.0) / 2.0;
    //                 *d = v;
    //                 // println!(
    //                 //     "az {} az0 {} az1 {} range {} range0 {} range1 {}  v00 {}  v01 {} v10 {} v11 {} v {}",
    //                 //     // elv,
    //                 //     az,
    //                 //     &elv_azs[ii],
    //                 //     &elv_azs[ii + 1],
    //                 //     rang,
    //                 //     rang0,
    //                 //     rang1,
    //                 //     v00,
    //                 //     v01,
    //                 //     v10,
    //                 //     v11,
    //                 //     v
    //                 // );

    //                 println!("x {} y {} v {}",x,y,v);
    //             }
    //         }
    //     // }
    // });

    // dbg!(&vol_azimuth);
    // let mut keys: Vec<&i16> = vol_azimuth.keys().collect();
    // keys.sort();
    // dbg!(keys);
    // let vol = Vol::new(
    //     vol_azimuth,
    //     vol_range,
    //     vol_value
    // );

    //  let (mut az, rang, mut elv) = transforms::cartesian_xyz_to_antenna(200.0, -200.0, 1.0, 0.0);
    //  println!("az {}  rang {} elv {}", f32::to_degrees(az), rang, f32::to_degrees(elv));
    //  let az = f32::to_degrees(az) * 100.0;
    //  let elv = f32::to_degrees(elv) * 100.0;
    //  println!("az {}  rang {} elv {}", az, rang, elv);

    //  let fix_elevation = &vol.fix_elevation;
    //  let vol_azimuth = &vol.vol_azimuth;
    //  let vol_range = &vol.vol_range;
    //  let azi_len = vol_azimuth.len();
    //  let fix_len = fix_elevation.len();
    //  if elv >= fix_elevation[0] as f32 && elv <= fix_elevation[fix_len as usize-1] as f32 {
    //     println!("ok");
    //  }

    //  let mut ie_found = false;
    //  for ie in 1..fix_len {
    //      //find ie
    //     if elv <= fix_elevation[ie] as f32 && !ie_found{
    //         println!("elv {} ie {}  {}  {}",elv,ie,fix_elevation[ie-1],fix_elevation[ie]);
    //         ie_found = true;
    //         let mut iaz_0_found = false;
    //         for (iaz_0,tmp_az) in vol_azimuth[ie-1].iter().enumerate() {
    //             //found az
    //             println!("tmp_a {} ",tmp_az);
    //             if az>= *tmp_az as f32/100.0 && !iaz_0_found{
    //                 println!("iaz0 {} ",iaz_0);
    //                 iaz_0_found = true;
    //             }
    //         }

    //         let mut range_0_found = false;
    //         for (range_0,tmp_range) in vol_range[ie -1].iter().enumerate() {
    //             //found az
    //             // println!("tmp_range {} ",tmp_range);
    //             if rang >= *tmp_range as f32/100.0 && !range_0_found{
    //                 println!("range0 {} ",range_0);
    //                 range_0_found = true;
    //             }
    //         }

    //         // let az_last_0;

    //     }
    //  }

    // dbg!(&vol_range);

    // println!("kdtree created");
    // let h = 0.0;
    // let z = 34.0;
    // let R: usize = 1000;
    // let W = 2 * R;
    // let mut grid_value = vec![-999.0; 2 * R * 2 * R];
    // grid_value.par_iter_mut().enumerate().for_each(|(i, d)| {
    //     let y = i / (2 * R);
    //     let x = i % (2 * R);
    //     let x = x as f32 - 1000.0;
    //     let y = y as f32 - 1000.0;
    //     let dst = (x * x + y * y).sqrt();

    //     if dst < 1000.0 {
    //         // println!("x {} y {}",x,y);
    //         let (az, rang, elv) = transforms::cartesian_xyz_to_antenna(x, y, z, h);
    //         // if elv > 15.0 {
    //         //     println!("elv {}", elv);
    //         // }
    //         let ret = kdtree
    //             .nearest(&[elv, az, rang], 4, &squared_euclidean)
    //             .unwrap();

    //         let di = ret[0].0;
    //         let vv = *ret[0].1 as f64;
    //         let mut nom = 0.0;
    //         let mut denom = 0.0;
    //         for (dist, value) in ret.iter() {
    //             // println!("dist {}",dist);
    //             if *dist < 15.0 {
    //                 nom += (**value) as f32 / dist;
    //                 denom += 1.0 / dist;
    //             }
    //         }
    //         if denom != 0.0 {
    //             let v = nom / denom;
    //             *d = vv;
    //         }
    //     }
    // });

    // let mut imgbuf = ImageBuffer::new(1000, 1000);
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let index = y * 1000 as u32 + x;

    //     let v = grid_value[index as usize];
    //     let c = pal.get_color(v as f64).unwrap();
    //     *pixel = image::Rgba([c.r, c.g, c.b, c.a]);
    // }
    // imgbuf.save("radar01.png").unwrap();
}
