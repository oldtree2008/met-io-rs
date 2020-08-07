use log::*;
use met_io_rs::*;
use std::convert::TryInto;
pub fn main() {
    let fname = r#"H:\data\单站\STANDARD_PRODUCT\LBCPZ20180614131507008.200"#;
    let reader = RadarPDReader::new(fname).unwrap();
    let rad: RadialData = reader.try_into().unwrap();
    dbg!(&rad.eles);

    // let ret = rad
    //     .ppi_to_grid(
    //         1.4699999, "Z", -150000.0, 150000.0, -150000.0, 150000.0, 150.0,
    //     )
    //     .unwrap();
    // // println!("{:?}",ret.2);
    // // dbg!(&ret.0,&ret.1);
    // let pal = "palette/xradar.xml";
    // let output = "okpd.png";

    // grid2img(&ret, pal, output);

    // let grid = rad.ppi_to_grid_lonlat(1.4699999, "Z").unwrap();

    // grid2diamond4(&grid, "d:/temp/demo");
    todiamond4(&rad, "d:/temp/demo4");
}
