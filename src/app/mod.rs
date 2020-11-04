use crate::*;
use anyhow::*;
use log::*;
use std::path::Path;
mod monitor_config;
mod output_type;
mod reader_factory;
mod reader_type;

pub use monitor_config::*;
pub use output_type::OutputType::*;
pub use output_type::*;
pub use reader_factory::*;
pub use reader_type::ReaderType;
pub use reader_type::ReaderType::*;

/// 从监控的源路径转换到目的路径。从文件名称的后缀或文件特征字符，确定Reader类型。从而选择对应的
/// Reader 解析文件。调用对应的转换函数。
pub fn convert_data(
    fname: &str,
    output: &str,
    rti: Option<ReaderType>,
    oti: Option<Vec<OutputType>>,
) -> Result<()> {
    let rt;
    if rti.is_none() {
        let rtt = ReaderType::try_from_path(&Path::new(fname));
        if rtt.is_none() {
            return Ok(());
        } else {
            rt = rtt.unwrap();
        }
    } else {
        rt = rti.unwrap();
    }
    let ot;
    if oti.is_none() {
        ot = vec![NOMGrid];
    } else {
        ot = oti.unwrap();
    }

    match rt {
        AWX(reader) => {
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        GPF(reader) => {
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        GRIB(reader) => {
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        KJSAT(reader) => {
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        KJH5(reader) => {
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        RAD386(reader) => {
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        RADPD(reader) => {
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        RADPT(reader) => {
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        _ => {
            //   for o in ot {
            //       match o {
            //           Diamond4 => todiamond4(&reader, output).unwrap(),
            //           NomGrid => tonoms(&reader, output).unwrap(),
            //           NOMProto => tonompbfs(&reader, output).unwrap(),
            //       }
            //   }
        }
    }

    Ok(())
}
