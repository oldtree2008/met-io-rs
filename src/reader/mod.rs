mod awx_reader;
mod gpf_reader;
mod hdf5_reader;
mod kj_sat_reader;
mod kjh5_sat_reader;
mod radar_386_reader;
mod radar_pd_reader;
mod xradar_reader;

pub use awx_reader::*;
pub use gpf_reader::*;
pub use hdf5_reader::*;
pub use kj_sat_reader::*;
pub use kjh5_sat_reader::*;
pub use radar_386_reader::*;
pub use radar_pd_reader::*;
pub use xradar_reader::*;

use crate::SingleGrid;

pub trait ToGrids {
    fn to_grids(&self) -> Option<Vec<SingleGrid>>;
}
