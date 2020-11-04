use met_io_rs::{AwxReader, ReaderFactory,ToGrids};
use std::path::Path;

pub fn main() {
    let path = Path::new("/mnt/d/demo/EILA19A0.AWX");
    // let fname = format!("{}",path.display());
    // let fname = path.file_name().unwrap();
    // let fname = fname.to_str().unwrap();
    // println!("{}",fname);
    // let reader = AwxReader::new(&fname).unwrap();

    // let path = Path::new("/mnt/d/demo/EILA19A1.AWX");
    if let Some(reader) = ReaderFactory::create_reader(&path) {
        if let Some(grids) = reader.to_grids() {

        }else {
            println!("no grid");
        }

    }else {
        println!("failed");
    }

    // dbg!(reader);
}
