mod error;
mod grid2img;
mod hdf5_error;
mod hdf5_reader;
mod satlite_reader;
mod xradar_reader;

pub use grid2img::*;
pub mod transforms;
use error::MetError;
pub mod radar_grid;

pub use xradar_reader::*;

use std::fs::File;
use std::io::Read;

pub use hdf5_error::Hdf5Error;
pub use hdf5_reader::*;
pub use satlite_reader::*;

pub trait MetReader {
    fn read<T>(r: &[u8]) -> Result<T, MetError>;
    fn read_file<T>(file_name: &str) -> Result<T, MetError> {
        let mut f = File::open(file_name)?;
        let mut data = Vec::new();
        f.read_to_end(&mut data)?;
        Self::read(&data)
    }
}
