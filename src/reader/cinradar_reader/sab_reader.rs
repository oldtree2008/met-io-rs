use crate::MetError;
use binread::prelude::*;
use std::io::Cursor;

pub struct SABReader<'a> {
    data: &'a [u8],
}
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

impl<'a> SABReader<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self, MetError> {
        println!("parse the sab radar file");
        // let mut rd = Cursor::new(data);
        let (radial_num, num) = SABReader::radial_num_SAB_CB(data);
        dbg!(&radial_num, &num);
        for i in 0..num {
            let from = radial_num * i;
            let end = (i + 1) * radial_num;
            let mut rd = Cursor::new(&data[from..end]);
            // rd.seek(SeekFrom::Start(step as u64))?;
            let header: Header = rd.read_le()?;
            println!("{:#?}", header);
            let start_index = header.ptr_of_reflectivity as usize + 28;
            let end_index = start_index + header.gate_size_of_reflectivity as usize;

            let dBZ = &data[start_index..end_index];
            let start_index = header.ptr_of_velocity as usize + 28;
            let end_index = start_index + header.gate_size_of_doppler as usize;
            let V = &data[start_index..end_index];
            let start_index = header.ptr_of_spectrum_width as usize + 28;
            let end_index = start_index + header.gate_size_of_doppler as usize;
            let W = &data[start_index..end_index];
        }

        Ok(SABReader { data })
    }

    pub fn radial_num_SAB_CB(data: &[u8]) -> (usize, usize) {
        let data_len = data.len();
        let radial_num = if data_len % 2432 == 0 {
            2432 //SAB
        } else if data_len % 4132 == 0 {
            4132 //CB
        } else {
            3132 //SB
        };
        (radial_num, data_len / radial_num)
    }
}