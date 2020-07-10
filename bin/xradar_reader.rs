use met_io_rs::radar_grid::RadialData;
use met_io_rs::*;
fn main() {
    let fname = r##"H:\data\20200704_164546.00.002.001_R1"##;
    let xr = XRadarReader::read(fname).unwrap();
    let rad: RadialData = xr.into();

    println!("eles {:?}", rad.eles);

    // let ret = rad
    //     .ppi_to_grid(0.54, -150000.0, 150000.0, -150000.0, 150000.0, 75.0, 0.0)
    //     .unwrap();

    let ret = rad
        .ppi_to_grid(0.54, -150000.0, 150000.0, -150000.0, 150000.0, 75.0, 0.0)
        .unwrap();    
    println!("{:?} {}", ret.0, ret.1);
    let pal = "palette/xradar.xml";
    let output = "ok.png";
    
    grid2img(&ret,pal,output);

    // println!("azs {:?}",rad.azs[0]);
    // println!("rs {:?}",rad.rs[0]);
    // println!("data {:?}",rad.data[0][0]);
}
