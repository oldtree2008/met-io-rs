use met_io_rs::*;

fn main() {
    let fname = r#"H:\data\FY2E\FY2E_2018_11_01_00_31.GPF"#;
    let pad = "palette/I-01.xml";
    let output = "d:/temp/demo4.png";
    let rd = GpfReader::new(fname).unwrap();
    let grids = rd.to_grids().unwrap();
    grids2diamond4s(&grids, "d:/temp/gpf1");
}
