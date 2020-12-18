use std::fs::File;
use std::io::Read;

use crate::MetError;

mod cc_reader;
mod sab_reader;
mod sc_reader;
mod wsr98d_reader;

use sab_reader::SABReader;
use sc_reader::SCReader;

pub struct CinRadarReader;

impl CinRadarReader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        let mut buf = Vec::new();
        let mut f = File::open(fname)?;
        f.read_to_end(&mut buf)?;

        // dbg!(buf.len() % 3132);
        let flag = &buf[0..28];

        let flag1 = &flag[0..4];
        if flag1 == b"RSTM" {
            println!("WSR98D");
        }

        if &flag[14..16] == b"\x01\x00" {
            println!("SAB");
            let reader = SABReader::new(&buf);
        }

        // dbg!(flag1);

        let sc_flag = &buf[100..109];
        if sc_flag == b"CINRAD/SC" || sc_flag == b"CINRAD/CD" {
            println!("SC");
            let reader = SCReader::new(&buf);
        }
        // dbg!(sc_flag);

        let cc_flag = &buf[116..125];
        if cc_flag == b"CINRAD/CC" {
            println!("CC")
        }
        // dbg!(cc_flag);

        Ok(CinRadarReader)
    }
}
