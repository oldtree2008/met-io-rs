use met_io_rs::*;
use std::convert::TryInto;
fn main() {
    let fname = r##"/mnt/h/data/20200704_164546.00.002.001_R1"##;
    let output = "/mnt/d/temp/xradar";
    let xr = XRadarReader::new(fname).unwrap();
    todiamond4(&xr, output);
}
