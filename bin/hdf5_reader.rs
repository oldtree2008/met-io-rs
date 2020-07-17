use met_io_rs::*;
use rayon::prelude::*;
use KJH5SatReader;

pub fn main() {
    let fname = r##"H:\data\FY2E\FY2E_2018_11_01_00_31.HDF"##;
    let sgrids = KJH5SatReader::read(fname).unwrap();
    sgrids.par_iter().for_each(|d| {
        dbg!(d.ni, d.nj, d.values.len());
        grid2diamond4(d, "d:/temp");
    })
}
