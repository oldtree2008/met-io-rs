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

pub const MISSING: f32 = 9999.0; //无效值

use crate::SingleGrid;

pub trait ToGrids {
    fn to_grids(&self) -> Option<Vec<SingleGrid>>;
}
// pub trait Radar :ToGrids  {
//     fn elevations(&self)->Vec<f32>;
//     fn elements(&self)->Vec<String>;
//     fn set_extents(&mut self);
//     fn extents(&self)->(f32,f32,f32,f32);
//     fn to_grid(&self,elv:f32,element:&str)->Option<SingleGrid>;
//     fn to_grids(&self)->Option<Vec<SingleGrid>> {
//         let mut grids = Vec::new();
//         for elv in self.elevations().iter() {
//             for element in self.elements().iter() {
//                if let Some(grid) = self.to_grid(*elv, element) {
//                 grids.push(grid);
//                }
//             }
//         }
//         Some(grids)
//     }
// }
