use met_io_rs::*;
use KJH5SatReader;

pub fn main() {
    let fname = r##"H:\data\FY2E\FY2E_2018_11_01_00_31.HDF"##;
    let rd = KJH5SatReader::read(fname).unwrap();
    let grids = rd.to_grids().unwrap();
    grids2diamond4s(&grids,"d:/tmp/hdf5");
}
