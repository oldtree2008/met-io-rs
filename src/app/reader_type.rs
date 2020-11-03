use crate::*;
use std::path::Path;
/// all the reader
pub enum ReaderType {
    AWX,    //卫星
    GPF,    //卫星
    GRIB,   //模式数据
    KJSAT,  //kj卫星
    KJH5,   //jk卫星hdf5
    RAD386, // jk雷达。  还有问题
    RADPD,  //单站雷达
    RADPT,  //雷达拼图
    RADX,   //X波段雷达
}

impl ReaderType {

    ///根据文件后缀或文件的特定字符，确定Reader的类型。
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
