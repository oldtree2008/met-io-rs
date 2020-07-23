mod converter;
mod data_type;
mod error;
mod hdf5_error;
mod reader;
mod utils;

pub use converter::*;
pub use data_type::*;
use error::MetError;
pub use hdf5_error::Hdf5Error;
pub use reader::*;
pub use utils::interplate;
pub use utils::kjlocationer;
pub use utils::transforms;

use std::fs::File;
use std::io::Read;

pub const MISSING: f32 = 9999.0; //无效值

// pub trait MetReader {
//     fn read<T>(r: &[u8]) -> Result<T, MetError>;
//     fn read_file<T>(file_name: &str) -> Result<T, MetError> {
//         let mut f = File::open(file_name)?;
//         let mut data = Vec::new();
//         f.read_to_end(&mut data)?;
//         Self::read(&data)
//     }
// }
