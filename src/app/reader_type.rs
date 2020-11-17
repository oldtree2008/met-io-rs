use crate::*;
use std::path::Path;
/// all the reader
pub enum ReaderType {
    AWX(AwxReader), //卫星
    GPF(GpfReader), //卫星
    #[cfg(not(target_arch = "wasm32"))]
    GRIB(GribReader), //模式数据
    KJSAT(KJSatReader), //kj卫星
    #[cfg(not(target_arch = "wasm32"))]
    KJH5(KJH5SatReader), //jk卫星hdf5
    RAD386(Radar386Reader), // jk雷达。  还有问题
    RADPD(RadarPDReader), //单站雷达
    RADPT(RadarPTReader), //雷达拼图
    RADX(XRadarReader), //X波段雷达
}

impl ReaderType {
    ///根据文件后缀或文件的特定字符，确定Reader的类型。
    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_from_path(path: &Path) -> Option<ReaderType> {
        //file_name不包括路径只是文件名称
        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_str().unwrap();
        //fname包括路径
        let fname = format!("{}", path.display());
        let fname = fname.as_str();
        if fname.ends_with(".awx") || fname.ends_with(".AWX") {
            if let Ok(reader) = AwxReader::new(fname) {
                return Some(AWX(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".GPF") {
            if let Ok(reader) = GpfReader::new(fname) {
                return Some(GPF(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".GRB") || fname.ends_with(".grb") {
            if let Ok(reader) = GribReader::new(fname) {
                return Some(GRIB(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".KJ") || fname.ends_with(".kj") {
            if let Ok(reader) = KJSatReader::new(fname) {
                return Some(KJSAT(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".HDF") || fname.ends_with(".HDF") {
            if let Ok(reader) = KJH5SatReader::new(fname) {
                return Some(KJH5(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".VT346") || fname.ends_with(".VT382") {
            if let Ok(reader) = Radar386Reader::new(fname) {
                return Some(RAD386(reader));
            } else {
                return None;
            }
        } else if fname.ends_with(".200") || fname.ends_with(".000") {
            if let Ok(reader) = RadarPDReader::new(fname) {
                return Some(RADPD(reader));
            } else {
                return None;
            }
        } else if &file_name[3..5] == "PT" {
            if let Ok(reader) = RadarPTReader::new(fname) {
                return Some(RADPT(reader));
            } else {
                return None;
            }
        } else {
            None
        }
    }
}
