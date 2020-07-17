#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct NomGrid {
    pub res: f32,
    pub bounds: LatLngBounds,
    pub data: Vec<Vec<f32>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LatLng {
    pub lat: f32,
    pub lng: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LatLngBounds {
    pub _southWest: LatLng,
    pub _northEast: LatLng,
}
