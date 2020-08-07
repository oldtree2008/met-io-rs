mod converter;
mod data_type;
mod error;
mod hdf5_error;
mod protos;
mod reader;
mod utils;

pub use converter::*;
pub use data_type::*;
use error::MetError;
pub use hdf5_error::Hdf5Error;
pub use protos::nom;
pub use reader::*;
pub use utils::interplate;
pub use utils::kjlocationer;
pub use utils::transforms;

use std::fs::File;
use std::io::Read;

pub const MISSING: f32 = 9999.0; //无效值

use crate::SingleGrid;

pub trait ToGrids {
    fn to_grids(&self) -> Option<Vec<SingleGrid>>;
}

