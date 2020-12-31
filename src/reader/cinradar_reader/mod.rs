use std::fs::File;
use std::io::Read;

use crate::{MetError, STRadialData, ToGrids};

mod cc_reader;
mod sab_reader;
mod sc_reader;
mod wsr98d_reader;

use common_data::SingleGrid;
use sab_reader::SABReader;
use sc_reader::SCReader;
use wsr98d_reader::WSR98DReader;

pub enum CinRadarReader {
    WSR98D(STRadialData),
    // SAB(SABReader),
    // SC(SCReader),
}

impl CinRadarReader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        let mut buf = Vec::new();
        let mut f = File::open(fname)?;
        f.read_to_end(&mut buf)?;
        // dbg!(buf.len() % 3132);
        let flag = &buf[0..28];
        let flag1 = &flag[0..4];
        //标准格式
        if flag1 == b"RSTM" {
            println!("WSR98D");
            let reader = WSR98DReader::new(&buf).unwrap();
            // let r = reader.el2idx(0.48339844);
            // println!("{:?}",r);
            // let r = reader.el2idx(19.511719);
            // println!("{:?}",r);
            // let r = reader.get_nearest_4v("dBT", 0.48339844, 0.1, 1000.0);
            // let r = reader.ppi_to_grid_lonlat(0.48339844, "dBZ");
            // println!("r {:?}", r);
            // let r = reader.get_elevate_element();
            // println!("r {:?}", r);
            return Ok(Self::WSR98D(reader));
        } else {
            // if &flag[14..16] == b"\x01\x00" {
            //     println!("SAB");
            //     let reader = SABReader::new(&buf)?;
            //     return Ok(Self::SAB(reader));
            // }

            // // dbg!(flag1);

            // let sc_flag = &buf[100..109];
            // if sc_flag == b"CINRAD/SC" || sc_flag == b"CINRAD/CD" {
            //     println!("SC");
            //     let reader = SCReader::new(&buf)?;
            //     return Ok(Self::SC(reader));
            // }
            // dbg!(sc_flag);

            // let cc_flag = &buf[116..125];
            // if cc_flag == b"CINRAD/CC" {
            //     println!("CC")
            // }
        }
        // dbg!(cc_flag);

        // Ok(CinRadarReader)
        Err(MetError::UnknowCinRadError)
    }
}

impl ToGrids for CinRadarReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        match self {
            Self::WSR98D(std) => std.to_grids(),
            _ => None,
        }
    }
}
