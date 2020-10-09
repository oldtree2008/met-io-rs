use met_io_rs::*;
use KJH5SatReader;

pub fn main() {
    let fname = r##"/mnt/e/data/FY2E/FY2E_2018_11_01_00_31.HDF"##;
    let rd = KJH5SatReader::new(fname).unwrap();
    todiamond4(&rd, "/mnt/d/tmp/hdf5");
    // let grids = rd.to_grids().unwrap();
    // grids2diamond4s(&grids, "/mnt/d/tmp/hdf5");
}
