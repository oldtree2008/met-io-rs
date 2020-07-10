use met_io_rs::transforms;

pub fn main() {
    println!("transfer");
    //x -4652.235  y 4652.2334 z 520
    let (az, rang, elv) = transforms::cartesian_xyz_to_antenna(-4652.235, 4652.2334, 520.0, 0.0);
    println!("az {}  rang {} elv {}", az, rang, elv);
    // let (x, y, z) = transforms::antenna_to_cartesian_cwr(6600.0, 315.0, 4.5, 0.0);

    // println!("x {}  y {} z {}", x, y, z);

    // let (az, rang, elv) = transforms::cartesian_to_antenna_cwr(14400.0, 148950.0, 19.0, 0.0);
    // println!("az {}  rang {} elv {}", az.to_degrees(), rang, elv);
}
