use crate::*;
use std::path::Path;
pub enum ReaderType {
    AWX,
    GPF,
    GRIB,
    KJSAT,
    KJH5,
    RAD386,
    RADPD,
    RADPT,
    RADX,
}

impl ReaderType {
    pub fn try_from_path(path: &Path) -> Option<ReaderType> {
        let fname = path.file_name().unwrap();
        let fname = fname.to_str().unwrap();
        if fname.ends_with(".awx") || fname.ends_with(".AWX") {
            return Some(AWX);
        } else if fname.ends_with(".GPF") {
            return Some(GPF);
        } else if fname.ends_with(".GRB") || fname.ends_with(".grb") {
            return Some(GRIB);
        } else if fname.ends_with(".KJ") || fname.ends_with(".kj") {
            return Some(KJSAT);
        } else if fname.ends_with(".HDF") || fname.ends_with(".HDF") {
            return Some(KJH5);
        } else if fname.ends_with(".VT346") || fname.ends_with(".VT382") {
            return Some(RAD386);
        } else if fname.ends_with(".200") || fname.ends_with(".000") {
            return Some(RADPD);
        } else if &fname[3..5] == "PT" {
            return Some(RADPT);
        } else {
            None
        }
    }
}
