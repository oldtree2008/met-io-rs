use super::Hdf5Error;
use hdf5;
use hdf5::Dataset;

#[derive(Debug)]
pub struct Hdf5Reader(Option<hdf5::File>);

impl Hdf5Reader {
    pub fn new(fname: &str) -> Result<Hdf5Reader, Hdf5Error> {
        let file = hdf5::File::open(fname)?;
        Ok(Hdf5Reader(Some(file)))
    }

    pub fn member_names(&self) -> hdf5::Result<Vec<String>> {
        let f = &self.0.as_ref().unwrap();
        f.member_names()
    }
    pub fn dataset(&self, name: &str) -> hdf5::Result<Dataset> {
        let f = &self.0.as_ref().unwrap();
        f.dataset(name)
    }
    pub fn shape(&self, name: &str) -> Vec<usize> {
        let f = &self.0.as_ref().unwrap();
        let d = f.dataset(name).unwrap();
        d.shape()
    }
}
