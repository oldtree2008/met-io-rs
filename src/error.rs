#[cfg(not(target_arch = "wasm32"))]
use eccodes_rs;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum MetError {
    #[error("read satlite error `{0}`")]
    ReadSatError(String),
    #[error("read data error")]
    IO(#[from] std::io::Error),
    #[error("binreader error")]
    BinReadError(#[from] binread::Error),
    #[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(not(target_arch = "wasm32"))]
    #[error("eccode-rs error")]
    ECCodesError(#[from] eccodes_rs::errors::EccodesError),
    #[error("serdejson error")]
    SerdeError(#[from] serde_json::error::Error),
}
