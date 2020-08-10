use met_io_rs::*;
use std::convert::TryInto;
fn main() {
    let fname = r##"H:\data\20200704_164546.00.002.001_R1"##;
    let xr = XRadarReader::new(fname).unwrap();
    let rad: RadialData = xr.0;

    println!("eles {:?} {}  {}", rad.start_time, rad.lon, rad.lat);

    // let ret = rad
    //     .ppi_to_grid(0.54, -150000.0, 150000.0, -150000.0, 150000.0, 75.0, 0.0)
    //     .unwrap();

    // let ret = rad
    //     .ppi_to_grid(0.54, -150000.0, 150000.0, -150000.0, 150000.0, 75.0, 0.0)
    //     .unwrap();
    // println!("{:?} {}", ret.0, ret.1);

    let ret = rad.ppi_to_grid_lonlat(0.54, "反射率").unwrap();
    println!("{:?} {}  {}", ret.ni, ret.nj, ret.values.len());

    grid2diamond4(&ret, "d:/temp");

    grid2nom(&ret, "d:/temp");

    // let pal = "palette/xradar.xml";
    // let output = "oklatlon.png";

    // grid2img(&ret, pal, output);

    // for i in 0..14 {
    //     println!("azs {:?}",rad.azs[0]);
    // }
    // for i in 0..360 {
    //     println!("rs {:?}",rad.rs[0][i].len());
    // }
    // println!("data {:?}",rad.data[0][1]);
}
