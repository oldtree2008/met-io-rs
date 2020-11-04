use std::path::Path;

use crate::{
    AwxReader, GpfReader, GribReader, KJH5SatReader, KJSatReader, Radar386Reader, RadarPDReader,
    RadarPTReader, ToGrids,
};

pub struct ReaderFactory;

impl ReaderFactory {
    pub fn create_reader(path: &Path) -> Option<Box<dyn ToGrids>> {
        let fname = format!("{}",path.display());
        let fname = fname.as_str();
        if fname.ends_with(".awx") || fname.ends_with(".AWX") {
            if let Ok(reader) = AwxReader::new(fname) {
                return Some(Box::new(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".GPF") {
            if let Ok(reader) = GpfReader::new(fname) {
                return Some(Box::new(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".GRB") || fname.ends_with(".grb") {
            if let Ok(reader) = GribReader::new(fname) {
                return Some(Box::new(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".KJ") || fname.ends_with(".kj") {
            if let Ok(reader) = KJSatReader::new(fname) {
                return Some(Box::new(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".HDF") || fname.ends_with(".HDF") {
            if let Ok(reader) = KJH5SatReader::new(fname) {
                return Some(Box::new(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".VT346") || fname.ends_with(".VT382") {
            if let Ok(reader) = Radar386Reader::new(fname) {
                return Some(Box::new(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".200") || fname.ends_with(".000") {
            if let Ok(reader) = RadarPDReader::new(fname) {
                return Some(Box::new(reader));
            } else {
                return None;
            }
        } else if &fname[3..5] == "PT" {
            if let Ok(reader) = RadarPTReader::new(fname) {
                return Some(Box::new(reader));
            } else {
                return None;
            }
        } else {
            None
        }
    }
}
