use met_io_rs::*;

fn main() {
    let fname = r##"/mnt/f/kj/data/T511/GRB/KTDIA2018013112925003.grb"##;
    let fname = r##"/mnt/f/kj/data/T511/GRB/KTDIA2018013112925006.grb"##;
    let output = r##"/mnt/d/temp/grib2"##;
    let reader = GribReader::new(fname).unwrap();

    todiamond4(&reader,output);
}