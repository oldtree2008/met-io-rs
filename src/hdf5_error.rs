use hdf5;
use thiserror::Error;

#[derive(Error, Debug)]

pub enum Hdf5Error {
    #[error("read hdf5 data error")]
    HDF5Error(#[from] hdf5::Error),
}
