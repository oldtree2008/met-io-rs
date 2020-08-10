use thiserror::Error;
use eccodes_rs;
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
    #[error("chrono parse error")]
    ChronoError(#[from] chrono::ParseError),
    #[error("reader to grids error")]
    ToGridsError,
    #[error("grids to Noms error")]
    ToNomsError,
    #[error("grids to NomPbfs error")]
    ToNomPbfsError,
    #[error("eccode-rs error")]
    ECCodesError(#[from] eccodes_rs::errors::EccodesError),
}
