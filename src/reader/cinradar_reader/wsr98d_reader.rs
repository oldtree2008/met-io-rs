use crate::MetError;
use crate::STRadialData;
use binread::prelude::*;
use chrono::NaiveDateTime;
use encoding_rs::*;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::io::Cursor;

const DATA_TYPE: [&'static str; 37] = [
    "dBT", "dBZ", "V", "W", "SQI", "CPA", "ZDR", "LDR", "CC", "PDP", "KDP", "CP", "Reserved",
    "HCL", "CF", "SNR", "Reserved", "Reserved", "Reserved", "Reserved", "Reserved", "Reserved",
    "Reserved", "Reserved", "Reserved", "Reserved", "Reserved", "Reserved", "Reserved", "Reserved",
    "Reserved", "Zc", "Vc", "Wc", "ZDRc", "FFT", "VIL",
];

#[derive(Debug, BinRead)]
struct CommonBlock {
    magic_num: i32,     //魔术字 固定标志，用来指示雷达数据文件。
    major_version: u16, //主版本号
    minor_version: u16, //次版本号
    generic_type: i32,  //文件类型  1–基数据文件；  2–气象产品文件； 3–谱数据文件；
    product_type: i32,  //产品类型 文件类型为1时此字段无效。
    #[br(count = 16)]
    reserved: Vec<u8>,
}

#[derive(Debug, BinRead)]
struct SiteInfo {
    #[br(count = 8)]
    site_code_: Vec<u8>,
    #[br(calc=GBK.decode(&site_code_).0.trim_end_matches('\u{0}').to_string())]
    site_code: String,
    #[br(count = 32)]
    site_name_: Vec<u8>,
    #[br(calc=GBK.decode(&site_name_).0.trim_end_matches('\u{0}').to_string())]
    site_name: String,
    latitude: f32,
    longtitude: f32,
    antena_height: i32, //天线高
    ground_height: i32, //雷达塔楼地面海拔高度
    frequency: f32,
    beam_width_h: f32,  //水平波束宽
    beam_width_v: f32,  //垂直波束宽
    radar_version: i32, //雷达数据采集软件版本号
    radar_type: u16,    //1–SA
    // 2–SB
    // 3–SC
    // 33–CA
    // 34–CB
    // 35–CC
    // 36–CCJ
    // 37–CD
    // 65–XA
    // 66–KA
    // 67–W
    antenna_gain: i16,
    trans_loss: i16,
    recv_loss: i16,
    other_loss: i16,
    #[br(count = 46)]
    reserved: Vec<u8>,
}

#[derive(Debug, BinRead)]
struct TaskInfo {
    #[br(count = 32)]
    task_name: Vec<u8>,
    #[br(count = 128)]
    task_des: Vec<u8>,
    polarization_type: i32, //1 – 水平极化  2 – 垂直极化 3 – 水平/垂直同时    4 – 水平/垂直交替
    scan_type: i32, //0 – 体扫  1–单层PPI  2 – 单层RHI  3 – 单层扇扫   4 – 扇体扫  5 – 多层RHI   6 – 手工扫描  7 – 垂直扫描
    pulse_width: i32,
    start_time_: i32,
    #[br(calc={let t = NaiveDateTime::from_timestamp(start_time_ as i64,0);t.format("%Y%m%d").to_string()})]
    start_date: String,
    #[br(calc={let t = NaiveDateTime::from_timestamp(start_time_ as i64,0);t.format("%H%M%S").to_string()})]
    start_time: String,
    cut_num: i32,
    noise_h: f32,
    noise_v: f32,
    cali_h: f32,
    cali_v: f32,
    h_noise_t: f32,
    v_noise_t: f32,
    zdr_cali: f32,
    phidp_cali: f32,
    ldr_cali: f32,
    // pulse_width2: f32,
    // pulse_width3: f32,
    // pulse_width4: f32,
    #[br(count = 40)]
    reserved: Vec<u8>,
}

#[derive(Debug, BinRead)]
struct CutInfo {
    process_mode: i32,
    wave_form: i32, //0 – CS连续监测
    // 1 – CD连续多普勒
    // 2 – CDX多普勒扩展
    // 3 – Rx Test
    // 4 – BATCH批模式
    // 5 – Dual PRF双PRF
    // 6 - Staggered PRT 参差PRT
    // 7 - single PRF 单PRF
    // 8 –linear 线性调频
    // 9 - phase encoding 相位编码
    prf1: f32,
    prf2: f32,
    deal_mod: i32,
    az: f32,
    elev: f32,
    start_az: f32,
    end_az: f32,
    ang_res: f32,
    scan_speed: f32,
    log_res: i32,
    dop_res: i32,
    max_range1: i32,
    max_range2: i32,
    start_range: i32,
    sample1: i32,
    sample2: i32,
    phase_mod: i32,
    at_loss: f32,
    ny_speed: f32,
    moments_mask: i64,
    moments_size_mask: i64,
    mis_filter_mask: i32,
    sqi: f32,
    sig: f32,
    csr: f32,
    log: f32,
    cpa: f32,
    pmi: f32,
    dplog: f32,
    #[br(count = 4)]
    r: Vec<u8>,
    dbt_mask: i32,
    dbz_mask: i32,
    v_mask: i32,
    w_mask: i32,
    dp_mask: i32,
    #[br(count = 12)]
    mask_reserved: Vec<u8>,
    scan_sync: i32,
    direction: i32,
    ground_clutter_type: u16,
    ground_clutter_filter_type: u16,
    ground_clutter_width: u16,
    ground_clutter_filter_win: i16,
    pulse_width: u16,
    pulse_width1: u16,
    pulse_width2: u16,
    pulse_width3: u16,
    pulse_width4: u16,

    #[br(count = 62)]
    reserved: Vec<u8>,
}

impl PartialEq for CutInfo {
    fn eq(&self, other: &Self) -> bool {
        self.elev == other.elev
    }
}

#[derive(Debug, BinRead, Clone)]
struct DataInfo {
    radial_state: i32,
    spot_blank: i32,
    seq_num: i32,
    rad_num: i32,
    elev_num: i32,
    az: f32,
    el: f32,
    sec: i32,
    micro_sec: i32,
    data_len: i32,
    moment_num: i32,
    last_sec: i32,
    fft_point: i16,
    acc_power: i16,
    #[br(count = 12)]
    reserved: Vec<u8>,
    #[br(count=moment_num)]
    data_block: Vec<DataBlock>,
}

impl PartialEq for DataInfo {
    fn eq(&self, other: &Self) -> bool {
        self.el == other.el
    }
}

#[derive(Debug, BinRead, Clone)]
struct DataBlock {
    data_type_: i32,
    #[br(calc=if data_type_>0 && data_type_ <37 {String::from(DATA_TYPE[data_type_ as usize-1])}else {String::from("UNKNOWN")})]
    data_type: String,
    scale: i32,
    offset: i32,
    pub bin_len: u16,
    flag: u16,
    pub len: i32,
    #[br(count = 12)]
    reserved: Vec<u8>,
    #[br(count= len)]
    data: Vec<u8>,
}

pub struct WSR98DReader;

impl WSR98DReader {
    pub fn new(buf: &[u8]) -> Result<STRadialData, MetError> {
        println!("parse standard radar");
        let mut cursor = Cursor::new(buf);
        let h: CommonBlock = cursor.read_le()?;
        // dbg!(&h);

        let h: SiteInfo = cursor.read_le()?;
        // dbg!(&h);

        let site_code = h.site_code.clone();
        let site_name = h.site_name.clone();
        let latitude = h.latitude;
        let longtitude = h.longtitude;
        let antena_height = h.antena_height;
        let ground_height = h.ground_height;

        let h: TaskInfo = cursor.read_le()?;
        let start_date = h.start_date.clone();
        let start_time = h.start_time.clone();
        // dbg!(&h);

        let cut_num = h.cut_num;
        let mut cut_infos = Vec::with_capacity(cut_num as usize * 256);
        let mut idx_el = Vec::new();
        for i in 0..cut_num {
            let h: CutInfo = cursor.read_le()?;
            idx_el.push((i + 1, h.elev));
            // println!("{:?}", h);
            cut_infos.push(h);
        }

        println!("{:?}", idx_el);
        // cut_infos.dedup();
        // for c in cut_infos.iter() {
        //     println!("{:?}", c);
        // }
        // println!("{:?}", cut_infos.len());

        let log_res = cut_infos[0].log_res;
        let dop_res = cut_infos[0].log_res;
        // println!("log_rs {} dop_res {}",log_res,dop_res);

        let mut data_infos = Vec::new();
        loop {
            if let Ok(d) = cursor.read_le::<DataInfo>() {
                let radial_state = d.radial_state;
                data_infos.push(d);
                if radial_state == 4 || radial_state == 6 {
                    println!("sweep end");
                    break;
                }
            } else {
                break;
            }
        }
        dbg!(data_infos.len());

        let data = convert2radial(data_infos, &cut_infos);

        Ok(STRadialData {
            _extents: (-100000.0, 100000.0, -100000.0, 100000.0),
            site_code,
            site_name,
            latitude,
            longtitude,
            antena_height,
            ground_height,
            start_date,
            start_time,
            log_res,
            dop_res,
            idx_el,
            data,
        })
    }
}

fn convert2radial(
    data_infos: Vec<DataInfo>,
    cut_infos: &Vec<CutInfo>,
) -> HashMap<i32, Vec<(f32, f32, HashMap<String, Vec<f32>>)>> {
    let mut sweep_start_ray_index = Vec::new();
    let mut sweep_end_ray_index = Vec::new();
    for (i, d) in data_infos.iter().enumerate() {
        let state = d.radial_state;
        if state == 0 || state == 3 {
            sweep_start_ray_index.push(i)
        }

        if state == 2 || state == 4 {
            sweep_end_ray_index.push(i);
        }
        // println!("{:#?}", d);
    }

    // println!("start_index {:?}", sweep_start_ray_index);
    // println!("end_index {:?}", sweep_end_ray_index);

    let start_end = sweep_start_ray_index.iter().zip(sweep_end_ray_index.iter());

    let mut data_infos = data_infos;

    //elv index from 1-> az ->data_type->data
    let mut el_az_dt_data = HashMap::new();
    let mut sorted_data = Vec::new();

    for (s, e) in start_end {
        let d = &mut data_infos[*s..=*e];
        d.sort_by(|a, b| a.az.partial_cmp(&b.az).unwrap());
        sorted_data.extend_from_slice(d);
    }

    for dd in sorted_data.iter() {
        // println!(
        //     "el {:?} {} az {:?}   {} ",
        //     dd.el, dd.elev_num, dd.az, dd.moment_num
        // );
        let mut dt_data = HashMap::new();
        for ddd in &dd.data_block {
            // println!("  {}    {}", ddd.data_type, ddd.len / ddd.bin_len as i32);
            let mut own_data: Vec<f32>; // = Vec::with_capacity(ddd.len as usize);
            let dt_slice = &ddd.data;
            let offset = ddd.offset;
            let scale = ddd.scale;
            if ddd.bin_len == 2 {
                own_data = dt_slice
                    .chunks_exact(2)
                    .map(|v| {
                        let vv = v.as_ref();
                        let vv = i16::from_le_bytes([vv[0], vv[1]]);
                        // vv as f32
                        if vv < 5 {
                            return crate::MISSING;
                        }
                        (vv - offset as i16) as f32 / scale as f32
                    })
                    .collect();
            } else {
                own_data = dt_slice
                    .iter()
                    .map(|v| {
                        if *v < 5 {
                            return crate::MISSING;
                        }
                        (*v as f32 - offset as f32) / scale as f32
                        // *v as f32
                    })
                    .collect();
            }
            // if &ddd.data_type == "dBT" {
            //     let print_data: Vec<&f32> =
            //         own_data.iter().filter(|d| d != &&crate::MISSING).collect();
            //     println!(
            //         "{:?}  {:?}  {:?}  {:?} ",
            //         dd.el,
            //         dd.az,
            //         ddd.data_type.clone(),
            //         print_data
            //     );
            // }
            // println!("{:?}",own_data);
            dt_data.insert(ddd.data_type.clone(), own_data);
        }

        let key = dd.elev_num;
        let el = cut_infos[dd.elev_num as usize - 1].elev;
        if !el_az_dt_data.contains_key(&key) {
            el_az_dt_data.insert(key, vec![(el, dd.az, dt_data)]);
        } else {
            let v = el_az_dt_data.get_mut(&key).unwrap();
            v.push((el, dd.az, dt_data));
        }
    }

    // println!("keys {:?}", el_az_dt_data.keys());
    // for i in 1..=11 {
    //     println!("keys {:?}", el_az_dt_data[&i][0].0);
    // }
    el_az_dt_data
}
