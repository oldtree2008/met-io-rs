use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetError {
    #[error("read satlite error `{0}`")]
    ReadSatError(String),
    #[error("read data error")]
    IO(#[from] std::io::Error),
    #[error("binreader error")]
    BinReadError(#[from] binread::Error),
    #[error("read hdf5 data error")]
    HDF5Error(#[from] hdf5::Error),
}
