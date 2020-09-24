use met_io_rs::kjlocationer::KJLocationer;
use met_io_rs::*;
fn main() {
    // let fname = "/mnt/e/data/201805311031_ch4_lbt_fy2e.kj";
    let fname = "/mnt/e/data/kj/202001191259_ch6_mkt_fy2g.kj";
    //202001191329_ch1_lbt_fy2g.kj
    let fname = "/mnt/e/data/kj/202001191329_ch1_lbt_fy2g.kj";
    let fname = "/mnt/e/data/kj/202001191329_ch3_lbt_fy2g.kj";
    // let fname = "/mnt/e/data/kj/202001191329_ch3_mkt_fy2g.kj";
    let pad = "palette/I-01.xml";
    // let pad = "palette/v-05.xml";//V-01_x

    // let output = "d:/temp/kjsat2.png";

    // let loc = KJLocationer::new(1).unwrap();
    // let (lon, lat) = loc.lbt_grid_ij_to_lat_and_longitude_proc(0.0, 0.0);
    // println!("lon {} lat {}", lon, lat);
    // let (lon, lat) = loc.lbt_grid_ij_to_lat_and_longitude_proc(0.0, 1024.0);
    // println!("lon {} lat {}", lon, lat);

    // let (lon, lat) = loc.lbt_grid_ij_to_lat_and_longitude_proc(1028.0, 0.0);
    // println!("lon {} lat {}", lon, lat);
    // let (lon, lat) = loc.lbt_grid_ij_to_lat_and_longitude_proc(1028.0, 1024.0);
    // println!("lon {} lat {}", lon, lat);
    // // // let (x,y) = loc.lbt_lat_lon_to_xy_coord_proc(10.616785 ,  144.0);

    // // // println!("x {} y {}",x,y);
    // let (x, y) = loc.lbt_lat_lon_to_xy_coord_proc(48.162987, 62.9151);
    // println!("x {} y {}", x, y);
    // let (x, y) = loc.lbt_lat_lon_to_xy_coord_proc(10.616785, 128.39842);
    // println!("x {} y {}", x, y);
    // let (x, y) = loc.lbt_lat_lon_to_xy_coord_proc(52.5865, 132.0744);
    // println!("x {} y {}", x, y);
    // let (x, y) = loc.lbt_lat_lon_to_xy_coord_proc(12.730557, 119.4446);
    // println!("x {} y {}", x, y);
    let reader = KJSatReader::new(fname).unwrap();
    dbg!(
        &reader.east,
        &reader.west,
        &reader.south,
        &reader.north,
        &reader.centerloni,
        &reader.centerlati,
        &reader.startline, //好像没什么用
        &reader.endline,
        &reader.startcol,
        &reader.endcol,
        &reader.res,
        &reader.time,
    );

    dbg!(&reader.data_date_time());
    dbg!(&reader.data_prod_ele());
    dbg!(&reader.proj());

    let grids = reader.to_grids();
    let grids = grids.unwrap();
    for grid in grids.iter() {
        grid2diamond4(grid, "/mnt/d/temp/demo");
        grid2nom(grid, "/mnt/d/temp/demo");
    }

    // dbg!(&reader.values);
    // grid2img(&reader.to_grid_img(), pad, output);
    // dbg!(reader);
}
