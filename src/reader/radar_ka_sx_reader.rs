use crate::MetError;
use binread::prelude::*;
use serde::Serialize;
use serde_json::*;
use std::fs::File;
use std::io::{Cursor, Read};

const DATA_TYPE: [&'static str; 37] = [
    "dBT", "dBZ", "V", "W", "SQI", "CPA", "ZDR", "LDR", "CC", "PDP", "KDP", "CP", "Reserved",
    "HCL", "CF", "SNR", "Reserved", "Reserved", "Reserved", "Reserved", "Reserved", "Reserved",
    "Reserved", "Reserved", "Reserved", "Reserved", "Reserved", "Reserved", "Reserved", "Reserved",
    "Reserved", "Zc", "Vc", "Wc", "ZDRc", "FFT", "VIL",
];
#[derive(Debug, Serialize)]
struct OutputData {
    data_type: String,
    values: Vec<f32>,
}

#[derive(Debug)]
pub struct RadarKASXReader {
    common_block: CommonBlock,
    site_info: SiteInfo,
    task_info: TaskInfo,
    config_info: ConfigInfo,
    data_info: DataInfo,
}

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
    site_code: Vec<u8>,
    #[br(count = 32)]
    site_name: Vec<u8>,
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
    trans_peak: f32,
    antena_gain: f32,
    total_loss: f32,
    receiver_gain: f32,
    first_side: f32,
    wave_length: f32,
    receiver_range: f32,
    receiver_sensivity: f32,
    band_width: f32,
    max_detectable_dis: u16,
    dis_res: u16,
    #[br(count = 14)]
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
    start_time: i32,
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
    pulse_width2: f32,
    pulse_width3: f32,
    pulse_width4: f32,
    #[br(count = 28)]
    reserved: Vec<u8>,
}

#[derive(Debug, BinRead)]
struct ConfigInfo {
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

#[derive(Debug, BinRead)]
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
#[derive(Debug, BinRead)]
struct DataBlock {
    data_type: i32,
    scale: i32,
    offset: i32,
    bin_len: u16,
    flag: u16,
    len: i32,
    #[br(count = 12)]
    reserved: Vec<u8>,
    #[br(count=bin_len as i32 * len)]
    data: Vec<u8>,
}

impl RadarKASXReader {
    pub fn new(fname: &str) -> std::result::Result<Self, MetError> {
        let mut file = File::open(fname)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let mut cursor = Cursor::new(&buf);
        let common_block: CommonBlock = BinRead::read(&mut cursor)?;
        dbg!(&common_block);

        let site_info: SiteInfo = BinRead::read(&mut cursor)?;
        dbg!(&site_info);

        let task_info: TaskInfo = BinRead::read(&mut cursor)?;
        dbg!(&task_info);

        let config_info: ConfigInfo = BinRead::read(&mut cursor)?;
        dbg!(&config_info);

        let data_info: DataInfo = BinRead::read(&mut cursor)?;
        dbg!(&data_info);

        Ok(Self {
            common_block,
            site_info,
            task_info,
            config_info,
            data_info,
        })
    }

    pub fn output(&self, fname: &str) {
        let data_block = &self.data_info.data_block;
        let mut outputs = Vec::new();
        for d in data_block.iter() {
            let dt = d.data_type - 1;
            let dt = dt as usize;
            dbg!(dt);
            let data_type = String::from(DATA_TYPE[dt]);
            let data = &d.data;
            let mut cursor = Cursor::new(data);
            let mut values = vec![crate::MISSING; d.len as usize];
            let scale = d.scale as f32;
            let offset = d.offset as f32;
            if d.bin_len == 1 {
                values.iter_mut().enumerate().for_each(|(_i, value)| {
                    let sd: u8 = BinRead::read(&mut cursor).unwrap();
                    if sd < 5 {
                        *value = crate::MISSING; //sd as f32;
                    } else {
                        *value = (sd as f32 - offset) / scale;
                    }
                })
            } else {
                values.iter_mut().enumerate().for_each(|(i, value)| {
                    let sd: u16 = BinRead::read(&mut cursor).unwrap();
                    if sd < 5 {
                        *value = crate::MISSING; //sd as f32;
                    } else {
                        *value = (sd as f32 - offset) / scale;
                    }
                })
            }
            outputs.push(OutputData { data_type, values })
        }
        // dbg!(&outputs);
        let outf = File::create(fname).unwrap();

        serde_json::to_writer(&outf, &outputs);
    }
}
