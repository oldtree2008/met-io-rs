use crate::MetError;
use crate::{RadialData, SingleGrid, ToGrids};
use binread::prelude::*;
use std::collections::{HashMap, HashSet};
use std::io::Cursor;

pub struct CBReader(pub RadialData);
#[derive(Debug, BinRead)]
pub struct Header {
    #[br(count = 14)]
    reserve0: Vec<u8>,
    flag: u16, //1-标识雷达数据
    #[br(count = 12)]
    reserve1: Vec<u8>,
    mseconds: u32,                     //径向数据收集时间(毫秒,自 00:00 开始)
    julian_data: u16,                  //儒略日(Julian)表示,自 1970 年 1 月 1 日开始
    urange: u16,                       //不模糊距离(表示:数值/10.=千米)
    az: u16,                           //方位角(编码方式:[数值/8.]*[180./4096.]=度)
    radial_number: u16,                //当前仰角内径向数据序号
    radial_status: u16,                //径向数据状态
    el: u16,                           //仰角 (编码方式:[数值/8.]*[180./4096.]=度)
    el_number: u16,                    //体扫内的仰角数
    range2first_gate_of_ref: u16,      // 反射率数据的第一个距离库的实际距离(单位:米)
    range2first_gate_of_dop: u16,      //多普勒数据的第一个距离库的实际距离(单位:米)
    gate_size_of_reflectivity: u16,    //反射率数据的距离库长(单位:米)
    gate_size_of_doppler: u16,         //多普勒数据的距离库长(单位:米)
    gates_number_of_reflectivity: u16, // 反射率的距离库数
    gates_number_of_doppler: u16,      //多普勒的距离库数
    cutsector_number: u16,             //扇区号
    calibration_const: u32,            //系统订正常数
    ptr_of_reflectivity: u16, //反射率数据指针(偏离雷达数据信息头的字节数) 表示第一个反射率数据的位置
    ptr_of_velocity: u16,     //速度数据指针(偏离雷达数据信息头的字节数),表示第一个速度数据的位置
    ptr_of_spectrum_width: u16, //谱宽数据指针(偏离雷达数据信息头的字节数),表示第一个谱宽数据的位置
    resolution_of_velocity: u16, //多普勒速度分辨率。 2:表示 0.5 米/秒
    vcp_number: u16,          //体扫(VCP)模式
    #[br(count = 14)]
    reserve2: Vec<u8>, //# 保留
    nyquist: u16,             //Nyquist 速度(表示:数值/100. = 米/秒)
    #[br(count = 38)]
    reserve3: Vec<u8>,
}

impl CBReader {
    pub fn new(data: &[u8]) -> Result<Self, MetError> {
        println!("parse the cb radar file");
        // let mut rd = Cursor::new(data);
        let radial_num = 1632;
        //let mut radial_data = Vec::new();
        let mut eles = Vec::new();
        let mut azs = Vec::new();
        let mut az_child = Vec::new();
        let mut rs = Vec::new();
        let mut rs_child = Vec::new();
        let mut dataZ = Vec::new();
        let mut dataZ_child = Vec::new();
        let mut dataV = Vec::new();
        let mut dataV_child = Vec::new();
        let mut dataW = Vec::new();
        let mut dataW_child = Vec::new();
        let mut radial_data = Vec::new();
        for i in 0..3600 {
            let from = radial_num * i;
            let end = (i + 1) * radial_num;
            let mut rd = Cursor::new(&data[from..end]);
            // rd.seek(SeekFrom::Start(step as u64))?;
            let header: Header = rd.read_le()?;
            //println!("{:#?}", header);
            let el = (header.el as f32 / 8.0) * (180.0 / 4096.0);
            let az = (header.az as f32 / 8.0) * (180.0 / 4096.0);
            /*
            dbg!(
                el,
                header.ptr_of_reflectivity,
                header.ptr_of_velocity,
                header.ptr_of_spectrum_width
            );*/
            let start_index = from + header.ptr_of_reflectivity as usize + 28;
            let end_index = start_index + header.gates_number_of_reflectivity as usize;
            let dBZ = &data[start_index..end_index];
            /*
            for d in dBZ.iter() {
                print!("{:0x} ", d);
            }
            println!("");
            */
            let dBZ: Vec<f32> = dBZ
                .iter()
                .map(|&d| {
                    if d == 0 {
                        crate::MISSING
                    } else {
                        //(d as f32 - 2f32) / 2f32 - 32f32
                        (d as f32 - 64f32) / 2f32
                    }
                })
                .collect();

            let start_index = from + header.ptr_of_velocity as usize + 28;
            let end_index = start_index + header.gates_number_of_doppler as usize;
            let V = &data[start_index..end_index];
            /*
            for d in V.iter() {
                print!("{:0x} ", d);
            }
            println!("");
            */
            let V: Vec<f32> = V
                .iter()
                .map(|&d| {
                    if d == 0 {
                        crate::MISSING
                    } else {
                        (d as f32 - 2f32) / 2f32 - 63.5f32
                    }
                })
                .collect();
            /*  println!(
                "el {} el_number:{} az:{} radial_number:{} status:{}   {:?}",
                el,
                header.el_number,
                az,
                header.radial_number,
                header.radial_status,
                dBZ
            );*/

            let start_index = from + header.ptr_of_spectrum_width as usize + 28;
            let end_index = start_index + header.gates_number_of_doppler as usize;
            let W = &data[start_index..end_index];
            /*
            for d in W.iter() {
                print!("{:0x} ", d);
            }
            println!("");
            */

            let W: Vec<f32> = W
                .iter()
                .map(|&d| {
                    if d == 0 {
                        crate::MISSING
                    } else {
                        (d as f32 - 2f32) / 2f32 - 63.5f32
                    }
                })
                .collect();

            az_child.push(az);
            let mut rs_grand_child: Vec<f64> = (0..500)
                .into_iter()
                .map(|d| d as f64 * 300 as f64)
                .collect();
            rs_child.push(rs_grand_child);
            dataZ_child.push(dBZ);
            dataV_child.push(V);
            dataW_child.push(W);
            if header.radial_status == 2 || header.radial_status == 4 {
                eles.push(el);
                azs.push(az_child);
                az_child = Vec::new();
                rs.push(rs_child);
                rs_child = Vec::new();
                dataZ.push(dataZ_child);
                dataZ_child = Vec::new();
                dataV.push(dataV_child);
                dataV_child = Vec::new();
                dataW.push(dataW_child);
                dataW_child = Vec::new();
            }
            //radial_data.push((el,az,dBZ.to_vec(),V.to_vec(),W.to_vec(),header.radial_status));
        }
        println!("{:#?}", eles);
        println!("{:#?} {}", azs.len(), azs[0].len());
        println!(
            "{:#?} {}  {}  {}",
            rs.len(),
            rs[0].len(),
            rs[0][1].len(),
            rs[8][2][499]
        );

        println!("{} {}", dataZ[0][0][0], dataZ[2][360][250]);

        radial_data.push(dataZ);
        radial_data.push(dataV);
        radial_data.push(dataW);

        println!(
            "element:{} elevate:{} az:{} range:{}",
            radial_data.len(),
            radial_data[0].len(),
            radial_data[0][0].len(),
            radial_data[0][0][0].len()
        );

        let start_date = "20210611".to_string(); //format!("{}{:02}{:02}", observe.year, observe.month, observe.day);
        let start_time = "120331".to_string(); // format!("{:02}{:02}{:02}", observe.hour, observe.minute, observe.sec);

        let mut props = HashMap::new();
        props.insert(String::from("product"), String::from("单站雷达"));
        props.insert(String::from("station"), "9433".to_string());

        let mut rdata = RadialData::default();
        rdata.lon = 125.246; //lon;
        rdata.lat = 43.899; //lat;
        rdata.height = 0f32; // 289.800; //height;
        rdata.start_date = start_date;
        rdata.start_time = start_time;
        rdata.bin_length = 300.0;
        rdata.eles = eles;
        rdata.azs = azs;
        rdata.rs = rs;
        rdata.elements = vec!["Z".to_string(), "V".to_string(), "W".to_string()]; //elements;
        rdata.data = radial_data;
        rdata.props = props;

        Ok(CBReader(rdata))
    }
}

impl ToGrids for CBReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let rd = &self.0;
        rd.to_grids()
    }
}
