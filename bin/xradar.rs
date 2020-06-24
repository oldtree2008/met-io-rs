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
    FileHeadLength: i16, //文件头长度
    #[br(pad_size_to = 20)]
    Mode: NullString, //雷达型号
    #[br(pad_size_to = 20)]
    Province: NullString,
    #[br(pad_size_to = 20)]
    Area: NullString,
    #[br(pad_size_to = 20)]
    AreaName: NullString, //区站名
    #[br(pad_size_to = 20)]
    VersionNum: NullString, //文件版本格式号 [4 - 7]存放数据来源
    #[br(pad_size_to = 20)]
    TaskName: NullString, //雷达扫描任务名称
    #[br(pad_size_to = 20)]
    NoData1: NullString, //保留1
    Longitude_: i32, //天线经度 单位取1 / 360000度(东经为正，西经为负)
    #[br(calc=Longitude_ as f32/360000.0)]
    Longitude: f32,
    Latitude_: i32, //天线纬度 单位取1 / 360000度(北纬为正，南纬为负)
    #[br(calc=Latitude_ as f32/360000.0)]
    Latitude: f32,
    Height: i32,      //天线海拔高度, 以mm为单位
    MaxElevate: i16,  //测站四周地物最大仰角,百分之一度为单位
    BestElevate: i16, //测站四周地物最佳仰角,百分之一度为单位
    NoData2: i16,     //保留2
}
#[derive(Debug, BinRead)]
#[br(little)]
pub struct Performance {
    AntennaGain: u16,        //天线增益，以.01db为单位
    VerticalBW: u16,         //垂直波束宽度
    HorizontalBW: u16,       //水平波束宽度
    Polarizate: u16, //极化状况  - 0：为水平极化； - 1：垂直极化；- 2：为双极化（双偏振）；- 3：为圆偏振； - 4：其它
    WaveLength: u32, //波长 以微米为单位
    PeakPower: u32,  //雷达峰值功率
    FirstSideLobeLevel: u16, //第一旁瓣电平，取绝对值(单位取百分之一dB)
    LineA: u16,      //线性接收机动态范围，百分之一 dB为单位
    AGCDelayNum: u16, //AGC延迟量 以微秒为单位
    LogA: u16,       //对数接收机，百分之一 dbmw为单位
    LineMinTestPower: u16, //线性接收机最小可测功率 百分之一，dbwm为单位
    NoiseT: u16,     //噪声消除量化阀值
    ClutterT: u16,   //多普勒杂波消除阀值，单位 .01db
    SQIT: u16,       //SQI 阀值
    VelocityP: u16,  //速度处理方式 0:无速度处理； 1:PPI； 2:FFT
    FilterP: u16, //地物处理方式 - 0:无地物处理；  - 1:地物杂波图扣除法； - 2:滤波器处理；- 3:滤波器＋地物杂波图 法； - 4:谱分析法
    IntensityR: u16, //强度估算采用的通道 - 1:对数; - 2:线性
    iRangeReduction: u16,
}
#[derive(Debug, BinRead)]
#[br(little)]
pub struct Observe {
    ProductNumber: u16, //产品编号 - 0:PPI1 - 1:RHI - 2:立体扫描 - 3.反射率 - 4.速度 - 5.谱宽
    SSLayerNumber: u16, //立体扫描层数
    StartYear: u16,
    StartMonth: u16,
    StartDay: u16,
    StartHour: u16,
    StartMinute: u16,
    Startsecond: u16,
    StartGPSTime: u32, //开始GPS时间，以微秒为单位
    Calibration: u16,  //定标情况 - 0:没有定标 - 1:自动定标 - 2:一周内人工定标 - 3:一月内人工定标
    IntensityI: u16,   //强度积分次数
    VelocityP: u16,    //速度处理样本数
    #[br(count = 30)]
    ID: Vec<u32>, //ID号
    #[br(count = 30)]
    ViewElement: Vec<u8>, //观测要素
    // -1：单强度
    // -2：单速度(单PRF)
    // -3：速度+谱宽(单PRF)
    // -4：单速度(双PRF)
    // -5：速度+谱宽(双PRF)
    // -6：强度+速度(单PRF)
    // -7：强度+速度(双PRF)
    // -8：三要素(单PRF)
    // -9：三要素(双PRF)
    // -10：四要素(ConR+R+V+W,单PRF)
    // -11：四要素(ConR+R+V+W,双PRF)
    #[br(count = 30)]
    SpeedDeambiguity: Vec<u8>, //速度退模糊
    // -0：无退模糊处理
    // -1：软件退模糊
    // -2：双PRF退模糊
    // -3：批示退模糊
    // -4：批示加软件退模糊
    // -5：双PRF退模糊
    // -6：双PRF+软件退模糊
    #[br(count = 30)]
    EFirstPrr: Vec<u16>, //各层第一种脉冲重复频率，计数单位1/10HZ
    #[br(count = 30)]
    ESecondPrr: Vec<u16>, //各层第二种脉冲重复频率，计数单位1/10HZ
    #[br(count = 30)]
    EPulse: Vec<u16>, //各层脉冲宽度，1/100微秒
    #[br(count = 30)]
    EMaxSpeed: Vec<u16>, //各层的最大可测速度，单位：厘米/秒
    #[br(count = 30)]
    ERDistanceNum: Vec<u16>, //各层的反射率距离库数
    #[br(count = 30)]
    ERadialNum: Vec<u16>, //各层采样的径向数
    #[br(count = 30)]
    EClutterL: Vec<u16>, //各层多普勒库长，米为单位
    #[br(count = 30)]
    ERlibL: Vec<u16>, //各层多普勒库长，米为单位
    #[br(count = 30)]
    EStartDistance: Vec<u16>, //各层径向上的第一个库(或者数据)的开始距离，米为单位
    #[br(count = 30)]
    PPIInFileSD: Vec<u32>, //各层PPI在文件中的开始位置，字节，含文件头
    #[br(count = 30)]
    elevation: Vec<u16>, //各层的仰角，单位/100度
 
    RadialArrangement: u8, //一个径向中的数据排列方式
    STOccupysize: u8, //一个强度数据占用的字节数，百位数表示
    SPOccupysize: u8, //一个速度数据占用的字节数，百位数表示
    SWOccupysize: u8, //一个谱宽数据占用的字节数，百位数表示
    STNoEchoCT:i16,//强度无回波的代码表
    SPNoEchoCT:i16,//速度无回波的代码表
    SWNoEchoCT:i16,//速度无回波的代码表
    STMinIncrement:i16,			//数据中的强度最小增量，
	SPMinIncrement:i16,			//数据中的速度最小增量，*1000
    SWMinIncrement:i16,			//数据中的谱宽最小增量，*1000
    Strength:i16,						//强度
	speed:i16,					//速度
	SpectrumWidth:i16,			//谱宽
	EndYear:u16,				//观测结束时间年
	EndMouth:u16,		//观测结束时间月
	EndDay:u16,		//观测结束时间日
	EndHour:u16,			//观测结束时间时
	EndMinute:u16,		//观测结束时间分
	EndSecond:u16,		//观测结束时间秒
	GPSTime:u32,				//GPS时间
    StructNum:u16,			//结构数组的大小
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
