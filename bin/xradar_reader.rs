use met_io_rs::*;
use std::convert::TryInto;
fn main() {
    let fname = r##"/mnt/h/data/20200704_164546.00.002.001_R1"##;
    let output = "/mnt/d/temp/xradar";
    let xr = XRadarReader::new(fname).unwrap();
    // let rad: RadialData = xr.0;

    // println!("eles {:?} {}  {}", rad.start_time, rad.lon, rad.lat);

    todiamond4(&xr, output);
    // tonoms(&xr, output);

    // let ret = rad
    //     .ppi_to_grid(0.54, -150000.0, 150000.0, -150000.0, 150000.0, 75.0, 0.0)
    //     .unwrap();

    // let ret = rad
    //     .ppi_to_grid(0.54, -150000.0, 150000.0, -150000.0, 150000.0, 75.0, 0.0)
    //     .unwrap();
    // println!("{:?} {}", ret.0, ret.1);

    // let ret = rad.ppi_to_grid_lonlat(0.54, "Z").unwrap();
    // println!("{:?} {}  {}", ret.ni, ret.nj, ret.values.len());

    // grid2diamond4(&ret, "/mnt/d/temp/xradar");

    // grid2nom(&ret, "/mnt/d/temp/xradar");

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
