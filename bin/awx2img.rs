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
    let r = AwxReader::new(r##"D:\BaiduNetdiskDownload\ANI_IR1_R04_20200509_0900_FY2G.AWX"##);
    // let r = AwxReader::read(r##"h:\data\B03_20200715_1640_HMW8.AWX"##);
    let r = r.unwrap();
    // let p = r.0;
    // let header = &p.header1.as_ref().unwrap();
    // let data = &p.data1.as_ref().unwrap();
    // dbg!(
    //     &p.productCategory,
    //     &header.satelliteName.to_string(),
    //     &header.channel,
    //     &header.flagOfProjection,
    //     &header.widthOfImage,
    //     &header.heightOfImage,
    //     &header.latitudeOfNorth,
    //     &header.latitudeOfSouth,
    //     &header.longitudeOfWest,
    //     &header.longitudeOfEast,
    //     &header.dataLengthOfCalibration
    // );
    let ret = r.to_grids().unwrap();
    grids2diamond4s(&ret, "d:/temp/awx");
    // grid2nom(&ret, "d:/temp");
    // dbg!(header.widthOfImage);
    // dbg!(header.heightOfImage);
    // let w = header.widthOfImage;
    // let h = header.heightOfImage;
    // let mut imgbuf = ImageBuffer::new(w as u32, h as u32);
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let index = y * w as u32 + x;
    //     let v = data[index as usize];
    //     let c = pal.get_color(v as f64).unwrap();
    //     *pixel = image::Rgba([c.r, c.g, c.b, c.a]);
    // }
    // imgbuf.save("demo1.png").unwrap();
}
