use met_io_rs::*;

fn main() {
    let fname = r#"D:\陕西云雷达\陕西云雷达\Z_RADA_I_57131_20190302000000_O_YCCR_HTKAA_RAW_M.BIN"#;

    let reader = RadarKASXReader::new(fname).unwrap();
    reader.output("d:/demo/output.json");
}
