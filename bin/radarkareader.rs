use met_io_rs::*;

fn main() {
    let fname = r#"D:\雷达\云雷达\数据样例\20170719_023607_005_01.dat"#;

    let reader = RadarKAReader::new(fname);
    // println!("{:?}",reader);
}
