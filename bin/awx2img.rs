use image::{imageops, GenericImageView, ImageBuffer, RgbaImage};
use met_io_rs::*;
use palette::*;
use std::path::Path;

pub fn main() {
    println!("awx2img");
    // let p = Path::new("palette/V-01_x.xml");
    let p = Path::new("palette/I-01.xml");
    let pal = Palette::new_with_file(&p).unwrap();
    let c = pal.get_color(2.0);
    //B03_20200715_1640_HMW8
    let r = AwxReader::new(r##"/mnt/e/data/awx/EIEU052C.AWX"##);
    // let r = AwxReader::read(r##"h:\data\B03_20200715_1640_HMW8.AWX"##);
    let r = r.unwrap();

    let ret = r.to_grids().unwrap();
    grids2diamond4s(&ret, "/mnt/d/temp/awx");
}
