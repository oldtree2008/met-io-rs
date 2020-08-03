use met_io_rs::*;

fn main() {
    let fname = r#"H:\data\FY2E\FY2E_2018_11_01_00_31.GPF"#;
    let pad = "palette/I-01.xml";
    let output = "d:/temp/demo4.png";
    let rd = GpfReader::new(fname).unwrap();
    // let grid = rd.to_grid_img();
    // grid2img(&grid, pad, output);
    // dbg!(
    //     &rd.wSatID,
    //     &rd.wPjType,
    //     &rd.fCLonRes,
    //     &rd.fCLatRes,
    //     &rd.fStdLat1,
    //     &rd.fStdLat2,
    //     &rd.fLtLat,
    //     &rd.fLtLon,
    //     &rd.fRtLat,
    //     &rd.fRtLon,
    //     &rd.fLbLat,
    //     &rd.fLbLon,
    //     &rd.fRbLat,
    //     &rd.fRbLon,
    //     &rd.wWidth,
    //     &rd.wHeight,
    // );
    let grids = rd.to_grids().unwrap();
    grids2diamond4s(&grids, "d:/temp/gpf1");
    // for g in grids.iter() {
    //     grid2diamond4(&g, "d:/temp/gpf");
    // }
}
