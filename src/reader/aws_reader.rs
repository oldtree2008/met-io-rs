use crate::MetError;
use binread::prelude::*;
use bitvec::prelude::*;
use std::fs::File;
use std::io::{Cursor, Read};

#[derive(BinRead, Debug)]
pub struct AwsReader {
    pub station_info: StationInfo,
}

#[derive(BinRead, Debug)]
#[br(big)]
pub struct StationInfo {
    //指示段  8bytes
    #[br(count = 4)]
    pub _id: Vec<u8>,
    #[br(count = 3)]
    pub _data_len: Vec<u8>,
    #[br(calc=i32::from_be_bytes([0,_data_len[0],_data_len[1],_data_len[2]]))]
    pub data_len: i32,
    pub version: u8,

    //标志段 23bytes
    #[br(count = 3)]
    pub _bz_len: Vec<u8>,
    #[br(calc=i32::from_be_bytes([0,_bz_len[0],_bz_len[1],_bz_len[2]]))]
    pub bz_len: i32,
    pub main_tab: u8,
    pub data_center: i16,
    pub child_center: i16,
    pub update_seq: u8,
    pub sbd_bz: u8,
    pub data_type: u8,
    pub sub_type: u8,
    pub local_sub_type: u8,
    pub main_version: u8,
    pub local_version: u8,
    // #[br(count=2)]
    pub year: i16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub reserved: u8,

    // //选编段
    // #[br(count = 3)]
    // pub _xbz_len: Vec<u8>,
    // #[br(calc=i32::from_be_bytes([0,_xbz_len[0],_xbz_len[1],_xbz_len[2]]))]
    // pub xbz_len: i32,
    // pub xbz_reserve: u8,
    // #[br(count=4)]
    // pub _xbz:Vec<u8>,

    //数据描述段
    #[br(count = 3)]
    pub _sjd_ms_len: Vec<u8>,
    #[br(calc=i32::from_be_bytes([0,_sjd_ms_len[0],_sjd_ms_len[1],_sjd_ms_len[2]]))]
    pub sjd_ms_len: i32,
    pub sjd_ms_reserve: u8,
    pub recorder_num: i16,
    pub compres_mod: u8,
    #[br(count = 2)]
    pub descr_seq: Vec<u8>,

    // 数据段
    #[br(count = 3)]
    pub _sjd_len: Vec<u8>,
    #[br(calc=i32::from_be_bytes([0,_sjd_len[0],_sjd_len[1],_sjd_len[2]]))]
    pub sjd_len: i32,
    pub sjd_reserved: u8,

    #[br(count=sjd_len)]
    pub sdj_data: Vec<u8>,
}

impl AwsReader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        let mut file = File::open(fname)?;
        let mut buf = Vec::new();

        file.read_to_end(&mut buf)?;
        let mut cursor = Cursor::new(&buf);
        dbg!(buf.len());

        let reader: AwsReader = cursor.read_be()?;
        let station_info = &reader.station_info;

        dbg!(&station_info);
        println!("{:#?}", station_info);

        let bits = BitSlice::<Msb0, _>::from_slice(&station_info.sdj_data).unwrap();
        let mut index = 0;

        let qh = bits[index..index + 7].load_be::<u8>();
        index += 7;
        let zh = bits[index..index + 10].load_be::<u16>();
        index += 10;
        let cz_lx = bits[index..index + 2].load_be::<u8>();
        index += 2;

        let gj_dq = bits[index..index + 10].load_be::<u16>();
        index += 10;

        let mut czbzs = Vec::new();
        for i in (1..10) {
            czbzs.push(bits[29 + (i - 1) * 8..29 + i * 8].load::<u8>());
        }
        index += 72;
        let year = bits[index..index + 12].load::<u16>();
        index += 12;
        let month = bits[index..index + 4].load::<u8>();
        index += 4;
        let day = bits[index..index + 6].load::<u8>();
        index += 6;

        let hour = bits[index..index + 5].load::<u8>();
        index += 5;

        let minute = bits[index..index + 6].load::<u8>();
        index += 6;

        let second = bits[index..index + 6].load::<u8>();
        index += 6;

        let lat = bits[index..index + 25].load::<u32>();
        index += 25;

        let lng = bits[index..index + 26].load::<u32>();
        index += 26;

        let height = bits[index..index + 17].load_be::<u32>();
        index += 17;
        let height1 = bits[index..index + 17].load_be::<u32>();
        index += 17;
        //地面限定符
        let dmxdf = bits[index..index + 5].load::<u8>();
        index += 5;

        //日照时制
        let rzsz = bits[index..index + 3].load::<u8>();
        index += 3;

        //质量控制
        let zl = bits[index..index + 4].load::<u8>();
        index += 4;
        let zl1 = bits[index..index + 4].load::<u8>();
        index += 4;

        //气压
        let press_sensor = bits[index..index + 6].load::<u8>();
        index += 6;
        let num = bits[index..index + 1].load::<u8>();
        index += 1;

        let added = bits[index..index + 6].load::<u8>();
        index += 6;

        //本站气压
        let press = bits[index..index + 14].load::<u16>();
        index += 14;

        //现象出现的时
        let xx_hour = bits[index..index + 5].load::<u8>();
        index += 5;
        let xx_minute = bits[index..index + 6].load::<u8>();
        index += 6;

        //一级统计
        let yjtj = bits[index..index + 6].load::<u8>();
        index += 6;
        // // println!("{:#?}",&bits);

        // //气温和湿度
        // let temp_sensor = bits[index..index + 6].load::<u8>();
        // index += 6;

        // let temp_num = bits[index..index + 1].load::<u8>();
        // index += 1;

        // //降水
        // let rain_sensor = bits[index..index + 6].load::<u8>();
        // index += 6;
        // let rain_num = bits[index..index + 1].load::<u8>();
        // index += 1;

        // //蒸发
        // let zf_sensor = bits[index..index + 6].load::<u8>();
        // index += 6;
        // let zf_num = bits[index..index + 1].load::<u8>();
        // index += 1;

        // //风
        // let wind_sensor = bits[index..index + 6].load::<u8>();
        // index += 6;
        // let wind_num = bits[index..index + 1].load::<u8>();
        // index += 1;
        // let wind_sensor_height = bits[index..index + 16].load::<u16>();
        // index += 16;

        // //当前时刻瞬时风向
        // let wind_add_des = bits[index..index + 6].load::<u8>();
        // index += 6;
        // let wind_direct = bits[index..index + 9].load::<u16>();
        // index += 9;
        // let wind_speed = bits[index..index + 12].load::<u16>();
        // index += 12;
        // let wind_time_des = bits[index..index + 5].load::<u8>();
        // index += 5;

        // //10min平均风向风速
        // let wind10_time_cycle = bits[index..index + 12].load::<u16>();
        // index += 12;
        // let wind10_add_des1 = bits[index..index + 6].load::<u8>();
        // index += 6;
        // let wind10_direct = bits[index..index + 9].load::<u16>();
        // index += 9;
        // let wind10_speed = bits[index..index + 12].load::<u16>();
        // index += 12;

        // //2min平均风向风速
        // let wind2_time_cycle = bits[index..index + 12].load::<u16>();
        // index += 12;
        // let wind2_add_des1 = bits[index..index + 6].load::<u8>();
        // index += 6;
        // let wind2_direct = bits[index..index + 9].load::<u16>();
        // index += 9;
        // let wind2_speed = bits[index..index + 12].load::<u16>();
        // index += 12;

        // //时间意义
        // let wind_time_des = bits[index..index + 5].load::<u8>();
        // index += 5;
        // let wind_time_cycle = bits[index..index + 12].load::<u16>();
        // index += 12;
        // //附加字段的意义
        // let wind_add_des = bits[index..index + 6].load::<u8>();
        // index += 6;
        // //小时内最大风速
        // let windhour_direct_max = bits[index..index + 9].load::<u16>();
        // index += 9;
        // let windhour_speed_max = bits[index..index + 12].load::<u16>();
        // index += 12;
        // let windhour_hour = bits[index..index + 5].load::<u16>();
        // index += 5;
        // let windhour_minute = bits[index..index + 6].load::<u16>();
        // index += 6;
        // //小时内极大风速
        // let windhour_direct_max1 = bits[index..index + 9].load::<u16>();
        // index += 9;
        // let windhour_speed_max1 = bits[index..index + 12].load::<u16>();
        // index += 12;
        // let windhour_hour1 = bits[index..index + 5].load::<u16>();
        // index += 5;
        // let windhour_minute1 = bits[index..index + 6].load::<u16>();
        // index += 6;

        // //过去6小时极大风向风速
        // let wind6h_time_cycle = bits[index..index + 12].load::<u16>();
        // index += 12;
        // let wind6h_add_des1 = bits[index..index + 6].load::<u8>();
        // index += 6;
        // let wind6h_direct = bits[index..index + 9].load::<u16>();
        // index += 9;
        // let wind6h_speed = bits[index..index + 12].load::<u16>();
        // index += 12;

        // //过去12小时极大风向风速
        // let wind12h_time_cycle = bits[index..index + 12].load::<u16>();
        // index += 12;
        // let wind12h_add_des1 = bits[index..index + 6].load::<u8>();
        // index += 6;
        // let wind12h_direct = bits[index..index + 9].load::<u16>();
        // index += 9;
        // let wind12h_speed = bits[index..index + 12].load::<u16>();
        // index += 12;

        // let wind_sensor_ground = bits[index..index + 16].load::<u16>();
        // index += 16;

        dbg!(&qh);
        dbg!(&zh);
        dbg!(&cz_lx);
        dbg!(&gj_dq);
        dbg!(&czbzs);
        dbg!(&year);
        dbg!(&month);
        dbg!(&day, &hour, &minute, &second);
        dbg!(&lat, &lng, &height, &height1);
        dbg!(&dmxdf, &rzsz);
        dbg!(&zl, &zl1);

        dbg!(
            &press_sensor,
            &num,
            &added,
            &press,
            &xx_hour,
            &xx_minute,
            &yjtj
        );

        // dbg!(&temp_sensor, &temp_num);
        // dbg!(&rain_sensor, &rain_num);
        // dbg!(&zf_sensor, &zf_num);
        // dbg!(
        //     &wind_sensor,
        //     &wind_num,
        //     &wind_sensor_height,
        //     &wind_add_des,
        //     &wind_direct,
        //     &wind_speed,
        //     &wind_time_des
        // );
        // dbg!(
        //     &wind10_direct,
        //     &wind10_speed,
        //     &wind10_add_des1,
        //     &wind10_time_cycle
        // );
        // dbg!(
        //     &wind2_direct,
        //     &wind2_speed,
        //     &wind2_add_des1,
        //     &wind2_time_cycle
        // );
        // dbg!(
        //     &wind_time_des,
        //     &wind_time_cycle,
        //     &wind_add_des,
        //     &windhour_direct_max,
        //     &windhour_speed_max,
        //     &windhour_hour,
        //     &windhour_minute
        // );
        // dbg!(
        //     &windhour_direct_max1,
        //     &windhour_speed_max1,
        //     &windhour_hour1,
        //     &windhour_minute1
        // );
        // dbg!(
        //     &wind6h_direct,
        //     &wind6h_speed,
        //     &wind6h_add_des1,
        //     &wind6h_time_cycle
        // );
        // dbg!(
        //     &wind12h_direct,
        //     &wind12h_speed,
        //     &wind12h_add_des1,
        //     &wind12h_time_cycle,
        //     &wind_sensor_ground
        // );

        // //地温
        // for i in (1..10) {
        //     let diwen_sensor = bits[index..index + 6].load::<u8>();
        //     index += 6;
        //     dbg!(&diwen_sensor);
        // }

        // let diwen_num = bits[index..index + 1].load::<u8>();
        // index += 1;
        // dbg!(&diwen_num);
        // let diwen_add_des = bits[index..index + 6].load::<u8>();
        // index += 6;
        // dbg!(&diwen_add_des);

        // let diwen_temp_surf = bits[index..index + 12].load::<u16>();
        // index += 12;
        // dbg!(&diwen_temp_surf);

        // let diwen12h_temp_surf = bits[index..index + 12].load::<u16>();
        // index += 12;
        // dbg!(&diwen12h_temp_surf);
        Ok(reader)
    }
}
