use geo;
use geojson::{Value, GeoJson};
use geo::prelude::Simplify;
use met_io_rs::*;
use std::convert::TryInto;

fn main() {
    let fname = r##"/mnt/e/临时/临时/SC偏振雷达/202003180849090.05V"##;
    let reader = CinRadarReader::new(fname).unwrap();
    // let grids = reader.ppi_to_grid_lonlat(0.48339844,"dbZ").unwrap;
    let grids = reader.to_grids().unwrap();
    let grid = &grids[0];
    let ret = contour(grid, &vec![0.5]);
    println!("{:?}", ret);
    
    let ret= ret[5].clone().geometry.unwrap().value;
    dbg!(&ret);
    let r :geo::MultiPolygon<f64> = ret.try_into().unwrap();
    dbg!(&r);

    let r = r.simplify(&1.0);
    dbg!(&r);
}
