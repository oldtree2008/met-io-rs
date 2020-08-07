use met_io_rs::*;
use std::convert::TryInto;

fn main() {
    println!("radar 386");
    let fname = r#"H:\data\346\LD20190301023857090.VT346"#;
    // let fname = r#"H:\data\382\LD20190524052637458.VT382"#;
    let reader = Radar386Reader::new(fname).unwrap();
    let rad: RadialData = reader.try_into().unwrap();
    dbg!(&rad.eles);

    //5.6099997
    let ret = rad
        .ppi_to_grid(
            4.0299997, "REF", -100000.0, 100000.0, -100000.0, 100000.0, 300.0,
        )
        .unwrap();
    // let ret = rad
    //     .ppi_to_grid(
    //         0.42999998, "REF", -150000.0, 150000.0, -150000.0, 150000.0, 150.0, 0.0,
    //     )
    //     .unwrap();
    // println!("{:?}",ret.2);
    // dbg!(&ret.0,&ret.1);
    let pal = "palette/xradar.xml";
    let output = "ok346.png";

    grid2img(&ret, pal, output);
}
