use crate::*;
use anyhow::*;
use std::path::Path;
mod monitor_config;
mod output_type;
mod reader_type;

pub use monitor_config::*;
pub use output_type::OutputType::*;
pub use output_type::*;
pub use reader_type::ReaderType;
pub use reader_type::ReaderType::*;

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
        AWX => {
            let reader = AwxReader::new(fname).unwrap();
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        GPF => {
            let reader = GpfReader::new(fname).unwrap();
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        GRIB => {
            let reader = GribReader::new(fname).unwrap();
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        KJSAT => {
            let reader = KJSatReader::new(fname).unwrap();
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        KJH5 => {
            let reader = KJH5SatReader::new(fname).unwrap();
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        RAD386 => {
            let reader = Radar386Reader::new(fname).unwrap();
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        RADPD => {
            let reader = RadarPDReader::new(fname).unwrap();
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        RADPT => {
            let reader = RadarPTReader::new(fname).unwrap();
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
        XRAD => {
            let reader = XRadarReader::new(fname).unwrap();
            for o in ot {
                match o {
                    Diamond4 => todiamond4(&reader, output).unwrap(),
                    NomGrid => tonoms(&reader, output).unwrap(),
                    NOMProto => tonompbfs(&reader, output).unwrap(),
                }
            }
        }
    }

    Ok(())
}
