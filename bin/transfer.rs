use met_io_rs::transforms;

pub fn main() {
    println!("transfer");
    //x -4652.235  y 4652.2334 z 520
    let (az, rang, elv) = transforms::cartesian_xyz_to_antenna(-4652.235, 4652.2334, 520.0, 0.0);
    println!("az {}  rang {} elv {}", az, rang, elv);
    // let (x, y, z) = transforms::antenna_to_cartesian_cwr(6600.0, 315.0, 4.5, 0.0);

    // println!("x {}  y {} z {}", x, y, z);

    let (az, rang, elv) = transforms::cartesian_to_antenna_cwr(460000.0, 460000.0, 1.0, 1000.0);
    println!("az {}  rang {} elv {}", az.to_degrees(), rang, elv);

    // let (lon1, lat1) = transforms::cartesian_to_geographic_aeqd(-150000.0, -150000.0, 114.0, 40.0);
    // let (lon2, lat2) = transforms::cartesian_to_geographic_aeqd(150000.0, 150000.0, 114.0, 40.0);
    // println!("lon1 {} lat1 {}  lon2 {} lat2 {}", lon1, lat1, lon2, lat2);

    // let (x1, y1) = transforms::geographic_to_cartesian_aeqd(lon1, lat1, 114.0, 40.0);
    // let (x2, y2) = transforms::geographic_to_cartesian_aeqd(lon2, lat2, 114.0, 40.0);
    // println!("x1 {} y1 {}  x2 {} y2 {}", x1, y1, x2, y2);

    // let step1 = (lon2 - lon1) / 3000.0 as f32;
    // let step2 = (lat2 - lat1) / 3000.0 as f32;
    // println!("step1 {}  step2 {}", step1, step2);

    // let ret = create_grid_extent(
    //     -150000.0, -150000.0, 150000.0, 150000.0, 114.0, 40.0, 3000, 3000,
    // );

    // println!("ret {:?}", ret);
}

fn create_grid_extent(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    lon0: f32,
    lat0: f32,
    row: usize,
    col: usize,
) -> ((f32, f32, f32, f32), (f32, f32), (usize, usize)) {
    let (lon1, lat1) = transforms::cartesian_to_geographic_aeqd(x1, y1, lon0, lat0);
    let (lon2, lat2) = transforms::cartesian_to_geographic_aeqd(x2, y2, lon0, lat0);
    let steplon = (lon2 - lon1) / (col - 1) as f32;
    let steplat = (lat2 - lat1) / (row - 1) as f32;
    let lon2 = lon1 + (col - 1) as f32 * steplon;
    let lat2 = lat1 + (row - 1) as f32 * steplat;
    ((lon1, lat1, lon2, lat2), (steplon, steplat), (row, col))
}
