use met_io_rs::*;
#[cfg(not(target_arch = "wasm32"))]
use KJH5SatReader;
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let files = [
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_00_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_01_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_03_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_04_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_05_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_06_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_07_10.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_08_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_09_03.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_11_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_12_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_14_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_15_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_16_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_19_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_20_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_21_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_16_23_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_00_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_01_11.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_02_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_03_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_04_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_05_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_06_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_07_15.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_08_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_09_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_10_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_11_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_12_01.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_13_07.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_14_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_15_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_16_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_19_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_21_00.HDF",
        "/mnt/e/kjdata/FY2G-0916-17/FY2G-0916-17/FY2G_2020_09_17_23_00.HDF",
    ];

    //let fname = r##"/mnt/e/data/FY2E/FY2E_2018_11_01_00_31.HDF"##;
    for fname in files.iter() {
        let rd = KJH5SatReader::new(fname).unwrap();
        todiamond4(&rd, "/mnt/d/tmp/hdf5");
    }
    // let grids = rd.to_grids().unwrap();
    // grids2diamond4s(&grids, "/mnt/d/tmp/hdf5");
}

#[cfg(target_arch = "wasm32")]
fn main() {}
