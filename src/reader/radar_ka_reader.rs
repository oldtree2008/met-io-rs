use crate::error::MetError;
use binread::prelude::*;
use std::fs::File;
use std::io::{Cursor, Read, SeekFrom};

#[derive(Debug, BinRead)]
pub struct RadarKAReader {
    jx_count: u32,   //径向个数
    start_time: u32, //开始时刻
    end_time: u32,   //结束时刻
    #[br(count=jx_count)]
    data_size: Vec<u32>, //每个径向大小
    #[br(count = 30)]
    jx_info: Vec<JXInfo>,
}

#[derive(Debug, BinRead)]
struct JXInfo {
    #[br(count = 4)]
    bits_data: Vec<u8>,

    extends: u32,
    #[br(count = 12)]
    bin_nums: Vec<u32>,
    #[br(count = 12)]
    data_offset: Vec<u32>,
    pl: f32,
    mc_pl: f32,
    pl1: f32,
    pl2: f32,
    start_az: i32,
    end_az: i32,
    start_el: i32,
    end_el: i32,
    bin_width: i32,
    #[br(count = 2)]
    noise: Vec<f32>,
    max_speed: i32,

    #[br(count = 12036)]
    data: Vec<u8>,
}

impl RadarKAReader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        let mut f = File::open(fname)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        let mut cursor = Cursor::new(&buf);

        let rd: RadarKAReader = BinRead::read(&mut cursor)?;
        // println!("hello {}",rd.layer_info.len());
        let layer_info = &rd.jx_info;

        // dbg!(&layer_info.bits_data,&layer_info.extends,&layer_info.bin_nums, &layer_info.bin_width, &layer_info.data_offset);

        // dbg!(&rd.pad);
        // let layer_info = &rd.jx_info1;

        // dbg!(&layer_info.bits_data,&layer_info.extends,&layer_info.bin_nums, &layer_info.bin_width, &layer_info.data_offset);

        // let layer_info = &rd.jx_info2;

        // dbg!(&layer_info.bits_data,&layer_info.extends,&layer_info.bin_nums, &layer_info.bin_width, &layer_info.data_offset);

        for l in layer_info.iter() {
            dbg!(&l.bin_nums, &l.bin_width, &l.data_offset);
        }
        // let data = &data[527..527*2];
        // dbg!(data);

        Ok(rd)
    }
}
