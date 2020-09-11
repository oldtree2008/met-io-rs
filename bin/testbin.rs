use met_io_rs::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let fname = r##"/mnt/e/data/20200704_164546.00.002.001_R1"##;
    let fname = r##"/mnt/h/data/20200704_164546.00.002.001_R1"##;
    // let fname = r#"/mnt/e/data/单站/STANDARD_PRODUCT/LGCPZ20160610135814014.000"#;
    let fname = r#"/mnt/e/data/单站/STANDARD_PRODUCT/LBCPZ20180614135631008.200"#;
    // // let fname = r#"/mnt/e/data/单站/STANDARD_PRODUCT/LGCRW20160610164007015.000"#;
    // let p = Path::new("palette/xradar.xml");
    let reader = RadarPDReader::new(fname).unwrap();

    // let output = "/mnt/e/temp/xradar";
    // let xr = XRadarReader::new(fname).unwrap();
    let mut grids = reader.to_grids().unwrap();
    dbg!(grids.len());

    let mut f = File::create("demo.data").unwrap();

    // let data = vec![1.0f32,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0];
    for grid in grids.iter_mut() {
        for d in grid.values.iter_mut() {
            if *d == 9999.0 {
                *d = 0.0
            }
            // *d = 10.0 % 2.0;
            let bs = d.to_le_bytes();
            f.write_all(&bs).unwrap();
        }
    }
}
