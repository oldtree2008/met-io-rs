use met_io_rs::*;
pub fn main() {
    let fname = r#"H:\data\拼图\L0DPT20100609090000005.001"#;
    let reader = RadarPTReader::new(fname).unwrap();
    // todiamond4(&reader, "d:/temp/pt");
    tonoms(&reader, "d:/temp/pt");
    tonompbfs(&reader, "d:/temp/pt");
    // let grids = reader.to_grids().unwrap();

    // grids2diamond4s(&grids, "d:/temp/demo");
}
