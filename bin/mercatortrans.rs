use met_io_rs::mercator_trans::*;
fn main() {
    let trans = MercatorTrans::new(155.0, -4.0, 85.0, 40.0, 1.7, 1751, 1101);
    // let (x, y) = trans.latlon2xycoords(40.0, 85.0);

    // dbg!(x, y);
    let (x,y) = trans.latlon_to_pixels(40.0, 85.0, 1.7);
    dbg!(x, y);
    let (x1,y1) = trans.latlon_to_pixels(-4.0, 155.0, 1.7);
    dbg!(x1-x, y-y1);
}
