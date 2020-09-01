use met_io_rs::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let fname = r##"/mnt/e/data/20200704_164546.00.002.001_R1"##;
    let output = "/mnt/e/temp/xradar";
    let xr = XRadarReader::new(fname).unwrap();
    let mut grids = xr.to_grids().unwrap();
    dbg!(grids.len());
    let fname = r##"/mnt/h/data/20200704_164546.00.002.001_R1"##;
    let mut f = File::create("demo.data").unwrap();

    // let data = vec![1.0f32,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0];
    for grid in grids.iter_mut() {
        for d in grid.values.iter_mut() {
            if *d == 9999.0 {
                *d = 0.0
            }
            let bs = d.to_le_bytes();
            f.write_all(&bs).unwrap();
        }
    }
}
