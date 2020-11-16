use met_io_rs::*;
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let fname = r##"/mnt/e/kjdata/监测预警产品/监测预警产品/飞机颠簸/FY2G_DB202006121201.hdf"##;
    let datas = [
        "/mnt/e/kjdata/监测预警产品/监测预警产品/雷暴/FY2G_LB202006121201.HDF",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/雷暴/FY2G_LB202008230701.HDF",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/雷暴/FY2G_LB202008250801.HDF",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/雷暴/FY2G_LB202008260208.HDF",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/雷暴/FY2G_LB202008290201.HDF",
        // "/mnt/e/kjdata/监测预警产品/监测预警产品/雷暴/FY2G_LB202009110708.HDF",error
        // "/mnt/e/kjdata/监测预警产品/监测预警产品/雷暴/FY2G_LB202009160903.HDF",error
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机颠簸/FY2G_DB202006121201.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机颠簸/FY2G_DB202008310904.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机颠簸/FY2G_DB202008312301.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机颠簸/FY2G_DB202009012200.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机颠簸/FY2G_DB202009020000.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009012100.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009040405.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009040903.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009012200.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009040501.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009041001.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009032200.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009040601.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009041101.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009040000.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009040701.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009040301.hdf",
        "/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009040801.hdf",
    ];
    // let fname = r##"/mnt/e/kjdata/监测预警产品/监测预警产品/雷暴/FY2G_LB202006121201.hdf"##;
    // let fname = r##"/mnt/e/kjdata/监测预警产品/监测预警产品/飞机积冰/FY2G_JB202009012100.hdf"##;
    for fname in datas.iter() {
        let reader = WarnH5Reader::new(fname).unwrap();
        // reader.to_grids();
        let output = r##"/mnt/d/temp/warn/noms"##;
        tonoms(&reader, output).unwrap();
        // todiamond4(&reader, output).unwrap();
    }
}
#[cfg(target_arch = "wasm32")]
fn main() {}