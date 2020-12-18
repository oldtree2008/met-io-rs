use met_io_rs::*;

fn main() {
    let fname = r##"/mnt/e/青海数据/青海数据/青海数据/201908230248200.05V"##; //201908100005290.17V
                                                                              //201908100011190.17V   201908100017090.17V   201908100046170.17V
                                                                              // let fname = r##"/mnt/e/青海数据/青海数据/青海数据/201908100005290.17V"##;
                                                                              // let fname = r##"/mnt/e/青海数据/青海数据/青海数据/201908100011190.17V"##;
                                                                              // let fname = r##"/mnt/e/青海数据/青海数据/青海数据/201908100017090.17V"##;
                                                                              // let fname = r##"/mnt/e/青海数据/青海数据/青海数据/201908100046170.17V"##;
                                                                              // let fname = r##"/mnt/e/青海数据/青海数据/青海数据/201707012259490.05V"##;

    // let fname = r##"/mnt/e/临时/临时/SC偏振雷达/202003180849090.05V"##;
    let fname = r##"/mnt/e/临时/临时/SC偏振雷达/DP2020031808490.05V"##;
    let reader = RadarVBTGSReader::new(fname).unwrap();
    todiamond4(&reader, "/mnt/d/temp/gs1");
}
