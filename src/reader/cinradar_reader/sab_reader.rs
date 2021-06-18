use crate::MetError;
use crate::{RadialDataEx, SingleGrid, ToGrids};
use binread::prelude::*;
use std::collections::HashMap;
use std::io::Cursor;

pub struct SABReader(pub RadialDataEx);
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

impl SABReader {
    pub fn new(data: &[u8]) -> Result<Self, MetError> {
        // let mut rd = Cursor::new(data);
        let (
            radial_num,
            num,
            gates_number_of_reflectivity,
            gates_number_of_V,
            gates_number_of_W,
            gate_size_of_reflectivity,
            gate_size_of_doppler,
        ) = SABReader::radial_num_SAB_CB(data)?;
        dbg!(&radial_num, &num);

        let mut eles = Vec::new();
        let mut azs = Vec::new();
        let mut az_child = Vec::new();
        //let mut rs = Vec::new();
        let mut rsZ = Vec::new();
        let mut rsV = Vec::new();
        let mut rsW = Vec::new();
        let mut rsZ_child = Vec::new();
        let mut rsV_child = Vec::new();
        let mut rsW_child = Vec::new();
        let mut dataZ = Vec::new();
        let mut dataZ_child = Vec::new();
        let mut dataV = Vec::new();
        let mut dataV_child = Vec::new();
        let mut dataW = Vec::new();
        let mut dataW_child = Vec::new();
        //let mut radial_data = Vec::new();
        let mut unsorted_data = Vec::new();
        for i in 0..num {
            let from = radial_num * i;
            let end = (i + 1) * radial_num;
            let mut rd = Cursor::new(&data[from..end]);
            // rd.seek(SeekFrom::Start(step as u64))?;
            let header: Header = rd.read_le()?;
            //println!("{:#?}", header);
            let el = (header.el as f32 / 8.0) * (180.0 / 4096.0);
            let az = (header.az as f32 / 8.0) * (180.0 / 4096.0);
            /*println!(
                "el {} el_number:{} az:{} radial_number:{} status:{}   {}",
                el,
                header.el_number,
                az,
                header.radial_number,
                header.radial_status,
                header.resolution_of_velocity
            );*/

            let start_index = from + 128;
            let end_index = start_index + gates_number_of_reflectivity;
            let dBZ = &data[start_index..end_index];
            let dBZ: Vec<f32> = dBZ
                .iter()
                .map(|&d| {
                    if d == 0 {
                        crate::MISSING
                    } else {
                        (d as f32 - 2f32) / 2f32 - 32f32
                    }
                })
                .collect();

            //println!("{:?} {}", dBZ, dBZ.len());
            let start_index = end_index;
            let end_index = end_index + gates_number_of_V;
            let V = &data[start_index..end_index];

            let V: Vec<f32> = V
                .iter()
                .map(|&d| {
                    if d == 0 {
                        crate::MISSING
                    } else {
                        /*if d == 1 {
                            println!("d ==1");
                        }*/
                        if header.resolution_of_velocity == 2 {
                            (d as f32 - 2f32) / 2f32 - 63.5f32
                        } else {
                            (d as f32 - 2f32) - 127f32
                        }
                    }
                })
                .collect();
            //println!("{:?} {}", V, V.len());
            let start_index = end_index;
            let end_index = end_index + gates_number_of_W;
            let W = &data[start_index..end_index];
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
            unsorted_data.push((az, dBZ, V, W));
            //az_child.push(az);
            let mut rsZ_grand_child: Vec<f64> = (0..gates_number_of_reflectivity)
                .into_iter()
                .map(|d| d as f64 * gate_size_of_reflectivity as f64)
                .collect();
            let mut rsV_grand_child: Vec<f64> = (0..gates_number_of_V)
                .into_iter()
                .map(|d| d as f64 * gate_size_of_doppler as f64)
                .collect();
            let mut rsW_grand_child: Vec<f64> = (0..gates_number_of_W)
                .into_iter()
                .map(|d| d as f64 * gate_size_of_doppler as f64)
                .collect();

            rsZ_child.push(rsZ_grand_child);
            rsV_child.push(rsV_grand_child);
            rsW_child.push(rsW_grand_child);
            //dataZ_child.push(dBZ);
            //dataV_child.push(V);
            //dataW_child.push(W);
            if header.radial_status == 2 || header.radial_status == 4 {
                unsorted_data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                eles.push(el);

                for un in unsorted_data {
                    az_child.push(un.0);
                    dataZ_child.push(un.1);
                    dataV_child.push(un.2);
                    dataW_child.push(un.3);
                }
                azs.push(az_child);
                az_child = Vec::new();
                rsZ.push(rsZ_child);
                rsZ_child = Vec::new();
                rsV.push(rsV_child);
                rsV_child = Vec::new();
                rsW.push(rsW_child);
                rsW_child = Vec::new();

                dataZ.push(dataZ_child);
                dataZ_child = Vec::new();
                dataV.push(dataV_child);
                dataV_child = Vec::new();
                dataW.push(dataW_child);
                dataW_child = Vec::new();

                unsorted_data = Vec::new();
            }

            //println!("{:?} {}", W, W.len());
            //dbg!(header.ptr_of_reflectivity,header.ptr_of_velocity,header.ptr_of_spectrum_width);
            //dbg!(header.gates_number_of_reflectivity,header.gate_size_of_reflectivity,header.gates_number_of_doppler,header.gate_size_of_doppler);
        }

        //println!("{}  {} {}",dataZ.len(),dataZ[1].len(),dataZ[10][300].len());
        let start_date = "20210611".to_string(); //format!("{}{:02}{:02}", observe.year, observe.month, observe.day);
        let start_time = "120331".to_string(); // format!("{:02}{:02}{:02}", observe.hour, observe.minute, observe.sec);

        let mut props = HashMap::new();
        props.insert(String::from("product"), String::from("单站雷达"));
        props.insert(String::from("station"), "9433".to_string());

        let mut rdata = RadialDataEx::default();
        rdata.lon = 125.246; //lon;
        rdata.lat = 43.899; //lat;
        rdata.height = 0f32; // 289.800; //height;
        rdata.start_date = start_date;
        rdata.start_time = start_time;
        rdata.bin_length = 300.0;
        rdata.eles = eles;
        rdata.azs = azs;
        rdata.rs = vec![rsZ, rsV, rsW];
        rdata.elements = vec!["Z".to_string(), "V".to_string(), "W".to_string()]; //elements;
        rdata.data = vec![dataZ, dataV, dataW];
        rdata.props = props;
        Ok(SABReader(rdata))
    }

    pub fn radial_num_SAB_CB(
        data: &[u8],
    ) -> Result<(usize, usize, usize, usize, usize, f32, f32), MetError> {
        let data_len = data.len();
        let (
            radial_num,
            gates_number_of_reflectivity,
            gates_number_of_V,
            gates_number_of_W,
            gate_size_of_reflectivity,
            gate_size_of_doppler,
        ) = if data_len % 2432 == 0 {
            println!("parse the sab radar file");
            //径向数据的长度固定，为2432字节
            //反射率距离库长为1000米，最大距离库数为460；
            //速度和谱宽距离库长为250米，最大距离库数为920。
            (2432, 460, 920, 920, 1000f32, 250f32) //SAB
        } else if data_len % 4132 == 0 {
            println!("parse the cb radar file");
            //径向数据的长度固定，为4132字节。
            //反射率距离库长为500米，最大距离库数为800；
            //速度和谱宽距离库长为125米，最大距离库数为1600。
            (4132, 800, 1600, 1600, 500f32, 125f32) //CB
        } else if data_len % 1632 == 0 {
            println!("parse the cb radar file");
            //径向数据的长度固定，为1632字节。
            //反射率距离库长为300米，最大距离库数为500；
            //速度和谱宽距离库长为300米，最大距离库数为500。
            (1632, 500, 500, 500, 300f32, 300f32) //CB
        } else {
            return Err(MetError::UnknowCinRadError);
        };
        Ok((
            radial_num,
            data_len / radial_num,
            gates_number_of_reflectivity,
            gates_number_of_V,
            gates_number_of_W,
            gate_size_of_reflectivity,
            gate_size_of_doppler,
        ))
    }
}

impl ToGrids for SABReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let rd = &self.0;
        rd.to_grids()
    }
}
