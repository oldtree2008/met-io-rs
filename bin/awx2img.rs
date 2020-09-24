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
    let datas = [
        //EIEU052C.AWX
        "/mnt/e/data/awx/EILA19C0.AWX",
        "/mnt/e/data/awx/EILA19C1.AWX",
        "/mnt/e/data/awx/EILA19D1.AWX",
        "/mnt/e/data/awx/EILA19E0.AWX",
        "/mnt/e/data/awx/EILA19E1.AWX",
        "/mnt/e/data/awx/EILA19F0.AWX",
        "/mnt/e/data/awx/EILA19F1.AWX",
        "/mnt/e/data/awx/EILA19G0.AWX",
        "/mnt/e/data/awx/EILA19G1.AWX",
        "/mnt/e/data/awx/EILA19H1.AWX",
        "/mnt/e/data/awx/EILA19H9.AWX",
        "/mnt/e/data/awx/EILA19I0.AWX",
        "/mnt/e/data/awx/EILA19I1.AWX",
        "/mnt/e/data/awx/EILA19J0.AWX",
        "/mnt/e/data/awx/EILA19J1.AWX",
        "/mnt/e/data/awx/EILA19K0.AWX",
        "/mnt/e/data/awx/EILU0531.AWX",
        "/mnt/e/data/awx/EILU053A.AWX",
        "/mnt/e/data/awx/EILU053C.AWX",
        "/mnt/e/data/awx/EILU0540.AWX",
        "/mnt/e/data/awx/EILU0541.AWX",
        "/mnt/e/data/awx/EILU054A.AWX",
        "/mnt/e/data/awx/EILU054C.AWX",
        "/mnt/e/data/awx/EILU0550.AWX",
        "/mnt/e/data/awx/EILU055A.AWX",
        "/mnt/e/data/awx/EILY04C1.AWX",
        "/mnt/e/data/awx/EILY04D0.AWX",
        "/mnt/e/data/awx/EILY04D1.AWX",
        "/mnt/e/data/awx/EILY04E1.AWX",
        "/mnt/e/data/awx/EILY04F0.AWX",
        "/mnt/e/data/awx/EILY04F1.AWX",
        "/mnt/e/data/awx/EILY04G0.AWX",
        "/mnt/e/data/awx/EILY04G1.AWX",
        "/mnt/e/data/awx/EILY04H0.AWX",
        "/mnt/e/data/awx/EILY04H1.AWX",
        "/mnt/e/data/awx/EILY04H9.AWX",
        "/mnt/e/data/awx/EILY04I0.AWX",
        "/mnt/e/data/awx/EILY04I1.AWX",
        "/mnt/e/data/awx/EILY04J0.AWX",
        "/mnt/e/data/awx/EILY04J1.AWX",
        "/mnt/e/data/awx/EILY04K0.AWX",
        "/mnt/e/data/awx/EILY04K1.AWX",
        "/mnt/e/data/awx/EILY05A0.AWX",
        "/mnt/e/data/awx/EILY05A1.AWX",
        "/mnt/e/data/awx/EILY05B0.AWX",
        "/mnt/e/data/awx/EILY05B1.AWX",
        "/mnt/e/data/awx/EILY05B9.AWX",
        "/mnt/e/data/awx/EILY05C1.AWX",
        "/mnt/e/data/awx/EILY05D0.AWX",
        "/mnt/e/data/awx/EILY05D1.AWX",
        "/mnt/e/data/awx/EILY05E0.AWX",
        "/mnt/e/data/awx/EILY05E1.AWX",
        "/mnt/e/data/awx/EILY05F0.AWX",
        "/mnt/e/data/awx/EILY05F1.AWX",
        "/mnt/e/data/awx/EILY05G0.AWX",
        "/mnt/e/data/awx/EILY05G1.AWX",
        "/mnt/e/data/awx/EILY05H9.AWX",
        "/mnt/e/data/awx/EILY05I0.AWX",
        "/mnt/e/data/awx/EILY05I1.AWX",
        "/mnt/e/data/awx/ERLA19K0.AWX",
        "/mnt/e/data/awx/EIEU052C.AWX",
    ];

    for d in datas.iter() {
        println!("{}", d);
        let r = AwxReader::new(d);
        // let r = AwxReader::new(r##"/mnt/e/data/awx/B03_20200715_1640_HMW8.AWX"##);
        if let Ok(r) = r {
            let ret = r.to_grids().unwrap();
            grids2diamond4s(&ret, "/mnt/d/temp/awx");
        } else {
            println!("error reader {}", r.err().unwrap());
        }
    }
}
