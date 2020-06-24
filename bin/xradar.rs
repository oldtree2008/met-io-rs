use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::result::Result;

use binread::prelude::*;
use binread::NullString;
use std::io::SeekFrom;
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
    header_len: i16,
    #[br(pad_size_to = 20)]
    radar_type: NullString,
    #[br(pad_size_to = 20)]
    province: NullString,
    #[br(pad_size_to = 20)]
    distribute: NullString,
    #[br(pad_size_to = 20)]
    site_name: NullString,
    #[br(pad_size_to = 20)]
    file_fmt: NullString,
    #[br(pad_size_to = 20)]
    scan_task: NullString,
    #[br(pad_size_to = 20)]
    reserve1: NullString,
    lon: i32,
    #[br(calc=lon as f32/360000.0)]
    lon_: f32,
    lat: i32,
    #[br(calc=lat as f32/360000.0)]
    lat_: f32,
    alt: i32,
    elva_max: i16,
    elva_best: i16,
    reserve2: i16,
}
#[derive(Debug, BinRead)]
#[br(little)]
pub struct Performance {
    gain: u16,          //天线增益
    v_width: u16,       //垂直波束宽度
    h_width: u16,       //水平波束宽度
    status: u16, //极化状况  - 0：为水平极化； - 1：垂直极化；- 2：为双极化（双偏振）；- 3：为圆偏振； - 4：其它
    wave_len: u32, //波长 以微米为单位
    max_p: u32,  //雷达峰值功率
    first_e: u16, //第一旁瓣电平
    receive_range: u16, //线性接收机动态范围
    agc_delay: u16, //AGC 延迟量
    receiver: u16, //对数接收机
    min_p: u16,  //线性接收机最小可测功率
    noise_: u16, //噪声消除量化阀值
    dpl_noise: u16, //多普勒杂波消除阀值
    sqi: u16,    //SQI 阀值
    v_pro: u16,  //速度处理方式 0:无速度处理； 1:PPI； 2:FFT
    di_pro: u16, //地物处理方式 - 0:无地物处理；  - 1:地物杂波图扣除法； - 2:滤波器处理；- 3:滤波器＋地物杂波图 法； - 4:谱分析法
    qd: u16,     //强度估算采用的通道 - 1:对数; - 2:线性
    iRangeReduction: u16,
}
#[derive(Debug, BinRead)]
#[br(little)]
pub struct Observe {
    prod_id: u16, //产品编号 - 0:PPI1 - 1:RHI - 2:立体扫描 - 3.反射率 - 4.速度 - 5.谱宽
    levels: u16,  //立体扫描层数
    year: u16,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    gps_time: u32,
    cal_:u16,//定标情况 - 0:没有定标 - 1:自动定标 - 2:一周内人工定标 - 3:一月内人工定标
    qdjf_num:u16,//强度积分次数
    sample_num:u16,//速度处理样本数
}
pub fn main() {
    let fname = r##"H:\xradar.data"##;

    let mut file = File::open(fname).unwrap();

    let mut d = Vec::new();
    file.read_to_end(&mut d).unwrap();
    let mut reader = Cursor::new(&d);

    let p: Product = reader.read_le().unwrap();

    dbg!(p);
}
