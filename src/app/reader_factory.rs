use std::path::Path;

use crate::{
    AwxReader, GpfReader, KJSatReader, Radar386Reader, RadarPDReader,
    RadarPTReader, ToGrids,
};
#[cfg(not(target_arch = "wasm32"))]
use crate :: {
    GribReader, KJH5SatReader
};
pub struct ReaderFactory;

impl ReaderFactory {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn create_reader(path: &Path) -> Option<Box<dyn ToGrids>> {
        //file_name不包括路径只是文件名称
        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_str().unwrap();
        //fname包括路径
        let fname = format!("{}", path.display());
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
        } else if &file_name[3..5] == "PT" {
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
