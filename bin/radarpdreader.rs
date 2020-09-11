use log::*;
use met_io_rs::*;
use serde_json::*;
use std::convert::TryInto;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn main() {
    //LGDPV20160610135814014.000
    let fname = r#"/mnt/e/data/单站/STANDARD_PRODUCT/LGCPZ20160610135814014.000"#;
    let fname = r#"/mnt/e/data/RADAR_STANDARD_PRODUCT   雷达产品/LGDPV20160610135814014.000"#;
    // let fname = r#"/mnt/e/data/单站/STANDARD_PRODUCT/LBCPZ20180614135631008.200"#;
    // // let fname = r#"/mnt/e/data/单站/STANDARD_PRODUCT/LGCRW20160610164007015.000"#;
    let p = Path::new("palette/xradar.xml");
    let reader = RadarPDReader::new(fname).unwrap();
    let output = "/mnt/d/temp/demo4";
    todiamond4(&reader, output);
    // // tonompbfs(&reader, output);
    tonoms(&reader, output);

    // let fname = r##"/mnt/d/temp/demo4/单站雷达/大校场/Z/1.49/20180614135631.000.NOM"##;
    // let reader = File::open(fname).unwrap();
    // let bufrd = BufReader::new(&reader);
    // let grid: NomGrid = serde_json::from_reader(bufrd).unwrap();

    // nom2img(&grid, "palette/xradar.xml", r#"/mnt/d/temp/demo.png"#)
}
