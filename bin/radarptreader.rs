use met_io_rs::*;
pub fn main() {
    let fname = r#"/mnt/e/data/testdata/拼图/L0DPT20100609090000005.001"#;
    let fname = r#"/mnt/e/data/testdata/拼图/L0DPT20100609090000120.001"#;
    let reader = RadarPTReader::new(fname).unwrap();
    todiamond4(&reader, "/mnt/d/temp/pt1");
    tonoms(&reader, "/mnt/d/temp/pt1");
    // tonompbfs(&reader, "d:/temp/pt");
    // let grids = reader.to_grids().unwrap();

    // grids2diamond4s(&grids, "d:/temp/demo");
}
