use crate::MetError;
use serde::*;
use serde_json;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorConfig {
    pub source: Vec<Source>,
    pub destination: Destination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub data_type: Option<String>,
    pub path: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Destination {
    pub data_type: Vec<String>,
    pub path: String,
}

pub fn get_config(fname: &str) -> Result<MonitorConfig, MetError> {
    let file = File::open(fname)?;
    let reader = BufReader::new(&file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}
