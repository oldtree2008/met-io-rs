use crate::error::*;
use crate::{RadialData, SingleGrid, ToGrids};
use binread::prelude::*;
use binread::NullString;
use encoding_rs::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::result::Result;

/**
 * 雷达站的信息。[85]字节。
 */
#[derive(Debug, BinRead)]
#[br(little)]
struct RadarStation {
    // #[br(pad_after=40)]
    // station:NullString,
    #[br(count = 40)]
    station_: Vec<u8>, //[40]站名
    #[br(count = 10)]
    station_number_: Vec<u8>, //[10]区站号
    #[br(count = 20)]
    radar_type: Vec<u8>, //[20]雷达型号
    radar_class: u8, //[1]雷达类别 0:数字化天气雷达  1:多普勒天气雷达
    longitude: i32,  //[4]天线所在经度的数值，以度/100为记数单位(十进制) 东经(E)为正,西经(W)为负
    latitude: i32,   //[4]天线所在纬度的数值，以度/100为记数单位(十进制) 北纬为正,南纬为负
    height: i32,     //[4]天线的海拔高度以毫米为记数单位
    max_angle: i16,  //[2]测站四周地物阻挡的最大仰角(以度/100为记数单位)
}

/**
 * 雷达站的观测参数。[648]字节。
 */
#[derive(Debug, BinRead)]
struct ObservationParam {
    physical_type: u8,  //[1]物理量模式:1=强度,2=速度,3=谱宽,4=三要素(强度、速度、谱宽)
    product_style: i16, //[2]产品类型编码（见表2）
    intensity_r: u8,    //[1]强度估算是否进行了距离订正:1：已进行了距离订正; 0：无

    year: u16, //[7]观测结束时间(7个字节UnsignedByte)
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    sec: u8,
    cappi_layer: u8, //[1]体扫或cappi层数，其它为1
    #[br(count = 30)]
    layers: Vec<LayerParam>,
    // List<LayerParam> layers ;//[30个]各圈扫描状态设置
    //                          //注:当扫描方式为RHI或PPI时，只在第一个元素中填写，其他元素为0。
    RHIA: i16, //[2]作RHI时的所在方位角记数单位1/100度,作PPI和立体扫描时为65535(0xFFFF)
    RHIL: i16, //[2]作RHI时的最低仰角，记数单位为1/100度 其他扫描时为-32768(0x8000)
    RHIH: i16, //[2]作RHI时的最高仰角，记数单位为1/100度 其他扫描时为-32768(0x8000)
}
/**
 * 每一个仰角层上的参数。[21]字节。
 */
#[derive(Debug, BinRead)]
struct LayerParam {
    ambiguousp: u8, //[1]退模糊状态
    //	0-无退模糊
    //	1-软件退模糊
    //	2-双T退模糊
    //	3-批式退模糊
    //	4-双T加软件退模糊
    //	5-批式加软件退模糊
    //	6-双PPI退模糊
    //	9-其它方式
    rotate_speed: i16, //[2]本层天线转速，记数单位：0.01度/秒
    prf1: u16,         //[2]本层的第一种脉冲重复频率，记数单位：1/10HZ
    prf2: u16,         //[2]本层的第二种脉冲重复频率，记数单位：1/10HZ
    spulse_wide: u16,  //[2]本层的脉冲宽度，：微妙
    max_speed: i16,    //[2]本层的最大可测速度，：厘米/秒
    max_distance: u16, //[2]本层的最大可测距离，：10米

    gate_leng: u16,    //[2]本层库长(1/10米)
    gate_count: u16,   //[2]本层库数
    radial_count: u16, //[2]本层径向数
    elevation_ang: u16, //[2]本层的仰角，记数单位：1/1000度
                       //当扫描方式为RHI，填写0;
                       //当扫描方式为PPI时，第一个元素为做PPI时的仰角，计数单位为 1/100度，其它元素填写0。
                       //计算得出。
}

#[derive(Debug, BinRead)]
struct RadarData {
    az: u16,
    el: u16,
    #[br(count = 1000)]
    values: Vec<u8>,
}

#[derive(Debug)]
pub struct RadarPDReader(pub RadialData);

impl RadarPDReader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        let mut file = File::open(fname)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let mut cursor = Cursor::new(&buf);
        let station: RadarStation = BinRead::read(&mut cursor)?;
        let observe: ObservationParam = BinRead::read(&mut cursor)?;

        let mut st = Vec::new();
        for b in &station.station_ {
            if *b == 0 {
                break;
            }
            st.push(*b);
        }
        let st = GBK.decode(&st).0;
        let st = st.to_string();
        dbg!(&observe.physical_type, &observe.product_style);

        let mut props = HashMap::new();
        props.insert(String::from("product"), String::from("单站雷达"));
        props.insert(String::from("station"), st);

        let elements = if observe.physical_type == 1 {
            vec!["uZ".to_string(), "Z".to_string()] //, "uZ".to_string()
        } else if observe.physical_type == 2 {
            vec!["V".to_string()]
        } else if observe.physical_type == 3 {
            vec!["W".to_string()]
        } else if observe.physical_type == 4 {
            vec![
                "uZ".to_string(),
                "Z".to_string(),
                "V".to_string(),
                "W".to_string(),
            ]
        } else {
            println!("unkown physical type {}", observe.physical_type);
            vec![String::from("UNKNOWN")]
        };
        // let station = &self.station;
        let lon = station.longitude as f32 * 0.01;
        let lat = station.latitude as f32 * 0.01;
        let height = station.height as f32 * 0.001;
        // let observe = &self.observe;
        let start_date = format!("{}{:02}{:02}", observe.year, observe.month, observe.day);
        let start_time = format!("{:02}{:02}{:02}", observe.hour, observe.minute, observe.sec);

        let data = &buf[1024..];
        let mut cursor = Cursor::new(data);

        let mut eles = Vec::new();
        let mut azs = Vec::new();
        let mut rs = Vec::new();
        let mut data = Vec::new(); //所有数据
        let mut datas = Vec::new();
        let mut datas1 = Vec::new();
        let mut datas2 = Vec::new();
        let mut datas3 = Vec::new();
        for layer in &observe.layers {
            let mut first = true;
            let mut el_az = Vec::new();
            let mut el_range = Vec::new();
            let mut el_linedata = Vec::new();

            let bin_num = layer.gate_count;
            let bin_width = layer.gate_leng as f64 * 0.1;
            // dbg!(bin_num, bin_width);
            for r in 0..layer.radial_count {
                let v: RadarData = BinRead::read(&mut cursor)?;
                let el = v.el as f32 * 0.01;
                let az = v.az as f32 * 0.01;
                if first {
                    eles.push(el);
                    first = false;
                }
                el_az.push(az);

                let mut ranges = Vec::new();
                let mut line_data = Vec::new();
                for i in 0..bin_num {
                    let r = i as f64 * bin_width;
                    let mut ld = v.values[i as usize] as f32;

                    if ld != 0.0 {
                        ld = (ld - 64.0) * 0.5;
                    } else {
                        ld = crate::MISSING;
                    }
                    // if (ld >100.0 || ld< -100.0) && ld!=crate::MISSING {
                    //     println!("wield {} ",v.values[i as usize] );
                    // }
                    line_data.push(ld);
                    ranges.push(r);
                }
                el_range.push(ranges);
                el_linedata.push(line_data);
            }
            if observe.physical_type == 4 {
                let mut el_linedata1 = Vec::new();
                for r in 0..layer.radial_count {
                    let v1: RadarData = BinRead::read(&mut cursor)?;
                    let mut line_data = Vec::new();
                    for i in 0..bin_num {
                        let r = i as f64 * bin_width;
                        let mut ld = v1.values[i as usize] as f32;
                        if ld != 0.0 {
                            ld = (ld - 64.0) * 0.5;
                        } else {
                            ld = crate::MISSING;
                        }
                        line_data.push(ld);
                    }
                    el_linedata1.push(line_data);
                }
                datas1.push(el_linedata1);

                let mut el_linedata2 = Vec::new();
                for r in 0..layer.radial_count {
                    let v2: RadarData = BinRead::read(&mut cursor)?;
                    let mut line_data = Vec::new();
                    for i in 0..bin_num {
                        let r = i as f64 * bin_width;
                        let mut ld = v2.values[i as usize] as f32;
                        if ld != 0.0 {
                            ld = (ld - 64.0) * 0.5;
                        } else {
                            ld = crate::MISSING;
                        }
                        line_data.push(ld);
                    }
                    el_linedata2.push(line_data);
                }
                datas2.push(el_linedata2);

                let mut el_linedata3 = Vec::new();
                for r in 0..layer.radial_count {
                    let v3: RadarData = BinRead::read(&mut cursor)?;
                    let mut line_data = Vec::new();
                    for i in 0..bin_num {
                        let r = i as f64 * bin_width;
                        let mut ld = v3.values[i as usize] as f32;
                        if ld != 0.0 {
                            ld = (ld - 64.0) * 0.5;
                        } else {
                            ld = crate::MISSING;
                        }
                        line_data.push(ld);
                    }
                    el_linedata3.push(line_data);
                }
                datas3.push(el_linedata3);
            }
            if observe.physical_type == 1 {
                let mut el_linedata1 = Vec::new();
                for r in 0..layer.radial_count {
                    let v1: RadarData = BinRead::read(&mut cursor)?;
                    let mut line_data = Vec::new();
                    for i in 0..bin_num {
                        let r = i as f64 * bin_width;
                        let mut ld = v1.values[i as usize] as f32;
                        if ld != 0.0 {
                            ld = (ld - 64.0) * 0.5;
                        } else {
                            ld = crate::MISSING;
                        }
                        line_data.push(ld);
                    }
                    el_linedata1.push(line_data);
                }
                datas1.push(el_linedata1);
            }

            azs.push(el_az);
            rs.push(el_range);
            datas.push(el_linedata);
        }

        data.push(datas);
        if observe.physical_type == 1 {
            data.push(datas1);
        } else if observe.physical_type == 4 {
            data.push(datas1);
            data.push(datas2);
            data.push(datas3);
        }
        dbg!(
            &data.len(),
            &data[0].len(),
            &data[0][0].len(),
            &data[0][0][0].len()
        );
        println!("{:#?}  {} ", &azs.len(), &azs[0].len());
        println!("{:#?}  {}  {}", &rs.len(), &rs[0].len(), &rs[0][0].len());
        let mut rdata = RadialData::default();
        rdata.lon = lon;
        rdata.lat = lat;
        rdata.height = height;
        rdata.start_date = start_date;
        rdata.start_time = start_time;
        rdata.eles = eles;
        rdata.azs = azs;
        rdata.rs = rs;
        rdata.elements = elements;
        rdata.data = data;
        rdata.props = props;

        dbg!(buf.len());
        dbg!(cursor.position());
        Ok(Self(rdata))
    }
}

impl ToGrids for RadarPDReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let rd = &self.0;
        rd.to_grids()
    }
}
