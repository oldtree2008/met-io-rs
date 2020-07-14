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
    let r = AwxReader::read(r##"D:\BaiduNetdiskDownload\ANI_IR1_R04_20200509_0900_FY2G.AWX"##);
    let r = r.unwrap();
    let p = r.0;
    let header = &p.header1.unwrap();
    let data = &p.data1.unwrap();

    dbg!(header.widthOfImage);
    dbg!(header.heightOfImage);
    let w = header.widthOfImage;
    let h = header.heightOfImage;
    let mut imgbuf = ImageBuffer::new(w as u32, h as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = y * w as u32 + x;
        let v = data[index as usize];
        let c = pal.get_color(v as f64).unwrap();
        *pixel = image::Rgba([c.r, c.g, c.b, c.a]);
    }
    imgbuf.save("demo.png").unwrap();
}
