use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct NomGrid {
    res:f32,
    bounds:LatLngBounds,
    data:Vec<Vec<f32>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LatLng {
    pub lat:f32,
    pub lng:f32,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct LatLngBounds {
    pub _southWest:LatLng,
    pub _northEast:LatLng,
}