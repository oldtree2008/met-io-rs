use met_io_rs::{todiamond4, CinRadarReader};
pub fn main() {
    let fname = r##"/mnt/e/radar/radar/BJXMY.20180612.050600.AR2/BJXMY.20180612.050600.AR2"##;
    let fname = r##"/mnt/e/临时/临时/SC雷达/Z_RADR_I_Z9796_20151205080100_O_DOR_SC_CAP.bin"##;
    // let fname = r##"/mnt/e/临时/临时/SC偏振雷达/202003180849090.05V"##; //解析错误 DP2020031808490.05V
    // let fname = r##"/mnt/e/临时/临时/SC偏振雷达/DP2020031808490.05V"##; //解析错误
    // let fname = r##"/mnt/e/临时/临时/SC雷达/201512050801000.05V"##;

    // let fname = r##"/mnt/e/临时/临时/云雷达/DTB20200318165435.090"##;//unknown type
    // let fname = r##"/mnt/e/临时/临时/云雷达/VTB20200318164112.009"##;//unknown type
    // let fname = r##"/mnt/e/雷达/Z_RADR_I_Z9010_20171028210600_O_DOR_SA_CAP.bin/Z_RADR_I_Z9010_20171028210600_O_DOR_SA_CAP.bin"##;
    // // let fname = r##"/mnt/e/青海数据/青海数据/青海数据/201908230248200.05V"##;//unknown type
    // let fname = r#"/mnt/h/陕西云雷达/陕西云雷达/Z_RADA_I_57131_20190302000000_O_YCCR_HTKAA_RAW_M.BIN"#;

    //标准格式
    let fname = r#"/mnt/e/标准格式/Z_RADR_I_Z9377_20201222012400_O_DOR_SB_CAP_FMT.bin"#;
    let fname = r#"/mnt/e/标准格式/Z_RADR_I_Z9570_20201222001048_O_DOR_SAD_CAP_FMT.bin"#; //error
                                                                                          //   let fname = r#"/mnt/e/标准格式/Z_RADR_I_Z9796_20201222000013_O_DOR_SCD_CAP_FMT.bin"#; //error
                                                                                          // let fname = r#"/mnt/e/标准格式/Z_RADR_I_Z9852_20201222013218_O_DOR_CDD_CAP_FMT.bin"#; //error
                                                                                          // let fname = r#"/mnt/e/标准格式/Z_RADR_I_Z9931_20201222040248_O_DOR_CC_CAP_FMT.bin"#;
                                                                                          // let fname = r#"/mnt/h/陕西云雷达/陕西云雷达/Z_RADA_I_57131_20190302000000_O_YCCR_HTKAA_RAW_M.BIN"#;

    let reader = CinRadarReader::new(fname).unwrap();

    let output = "/mnt/d/temp/demo4";
    todiamond4(&reader, output);
    println!("finished");
}
