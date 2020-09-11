use crate::*;
use anyhow::*;
use log::*;
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
            let reader = AwxReader::new(fname);
            if let Ok(reader) = reader {
                for o in ot {
                    match o {
                        Diamond4 => todiamond4(&reader, output).unwrap(),
                        NomGrid => tonoms(&reader, output).unwrap(),
                        NOMProto => tonompbfs(&reader, output).unwrap(),
                    }
                }
            } else {
                println!("AwxReader {}  {}", fname, reader.err().unwrap());
            }
        }
        GPF => {
            let reader = GpfReader::new(fname);
            if let Ok(reader) = reader {
                for o in ot {
                    match o {
                        Diamond4 => todiamond4(&reader, output).unwrap(),
                        NomGrid => tonoms(&reader, output).unwrap(),
                        NOMProto => tonompbfs(&reader, output).unwrap(),
                    }
                }
            } else {
                println!("GpfReader {}  {}", fname, reader.err().unwrap());
            }
        }
        GRIB => {
            let reader = GribReader::new(fname);
            if let Ok(reader) = reader {
                for o in ot {
                    match o {
                        Diamond4 => todiamond4(&reader, output).unwrap(),
                        NomGrid => tonoms(&reader, output).unwrap(),
                        NOMProto => tonompbfs(&reader, output).unwrap(),
                    }
                }
            } else {
                println!("GribReader {}  {}", fname, reader.err().unwrap());
            }
        }
        KJSAT => {
            let reader = KJSatReader::new(fname);
            if let Ok(reader) = reader {
                for o in ot {
                    match o {
                        Diamond4 => todiamond4(&reader, output).unwrap(),
                        NomGrid => tonoms(&reader, output).unwrap(),
                        NOMProto => tonompbfs(&reader, output).unwrap(),
                    }
                }
            } else {
                println!("KJSatReader {}  {}", fname, reader.err().unwrap());
            }
        }
        KJH5 => {
            let reader = KJH5SatReader::new(fname);
            if let Ok(reader) = reader {
                for o in ot {
                    match o {
                        Diamond4 => todiamond4(&reader, output).unwrap(),
                        NomGrid => tonoms(&reader, output).unwrap(),
                        NOMProto => tonompbfs(&reader, output).unwrap(),
                    }
                }
            } else {
                println!("KJH5SatReader {}  {}", fname, reader.err().unwrap());
            }
        }
        RAD386 => {
            let reader = Radar386Reader::new(fname);
            if let Ok(reader) = reader {
                for o in ot {
                    match o {
                        Diamond4 => todiamond4(&reader, output).unwrap(),
                        NomGrid => tonoms(&reader, output).unwrap(),
                        NOMProto => tonompbfs(&reader, output).unwrap(),
                    }
                }
            } else {
                println!("Radar386Reader {}  {}", fname, reader.err().unwrap());
            }
        }
        RADPD => {
            let reader = RadarPDReader::new(fname);
            if let Ok(reader) = reader {
                for o in ot {
                    match o {
                        Diamond4 => todiamond4(&reader, output).unwrap(),
                        NomGrid => tonoms(&reader, output).unwrap(),
                        NOMProto => tonompbfs(&reader, output).unwrap(),
                    }
                }
            } else {
                println!("RadarPDReader {}  {}", fname, reader.err().unwrap());
            }
        }
        RADPT => {
            let reader = RadarPTReader::new(fname);
            if let Ok(reader) = reader {
                for o in ot {
                    match o {
                        Diamond4 => todiamond4(&reader, output).unwrap(),
                        NomGrid => tonoms(&reader, output).unwrap(),
                        NOMProto => tonompbfs(&reader, output).unwrap(),
                    }
                }
            } else {
                println!("RadarPTReader {}  {}", fname, reader.err().unwrap());
            }
        }
        XRAD => {
            let reader = XRadarReader::new(fname);
            if let Ok(reader) = reader {
                for o in ot {
                    match o {
                        Diamond4 => todiamond4(&reader, output).unwrap(),
                        NomGrid => tonoms(&reader, output).unwrap(),
                        NOMProto => tonompbfs(&reader, output).unwrap(),
                    }
                }
            } else {
                println!("XRadarReader {}  {}", fname, reader.err().unwrap());
            }
        }
    }

    Ok(())
}
