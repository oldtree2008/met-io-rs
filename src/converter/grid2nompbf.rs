// use protobuf::parse_from_bytes;
use crate::nom::*;
use crate::{MetError, SingleGrid, ToGrids};
use protobuf::Message;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

pub fn tonompbfs<T>(reader: &T, output: &str) -> Result<(), MetError>
where
    T: ToGrids,
{
    if let Some(grids) = reader.to_grids() {
        Ok(grids2nompbfs(&grids, output))
    } else {
        Err(MetError::ToNomPbfsError)
    }
}

pub fn grids2nompbfs(d: &Vec<SingleGrid>, output: &str) {
    for dd in d.iter() {
        grid2nompbf(dd, output);
    }
}

pub fn grid2nompbf(d: &SingleGrid, output: &str) {
    let dd;
    if f64::abs(d.lng_gap) != f64::abs(d.lat_gap) {
        dd = crate::normalize_grid(&d);
    } else {
        dd = d.clone();
    }

    let dd = &dd;
    // let datastr = format!("{}{}", dd.data_date, dd.data_time);
    // let dt = Utc.datetime_from_str(&datastr, "%Y%m%d%H%M%S").unwrap();

    let dst_file_name = if let Some(l) = &dd.level {
        format!(
            "{}/{}/{}/{}/{}{:02}.{:03}.pb",
            output, &dd.product, &dd.element, l, &dd.data_date, &dd.data_time, &dd.forecast_time
        )
    } else {
        format!(
            "{}/{}/{}/{}{:02}.{:03}.pb",
            output, &dd.product, &dd.element, &dd.data_date, &dd.data_time, &dd.forecast_time
        )
    };

    let path = Path::new(&dst_file_name);
    let parent = path.parent().unwrap();
    if !parent.exists() {
        create_dir_all(&parent).unwrap();
    }
    let mut file = File::create(&dst_file_name).unwrap();

    let east;
    let west;
    if dd.start_lng < dd.end_lng {
        west = dd.start_lng;
        east = dd.end_lng;
    } else {
        west = dd.end_lng;
        east = dd.start_lng;
    }
    let south;
    let north;
    if dd.start_lat < dd.end_lat {
        south = dd.start_lat;
        north = dd.end_lat;
    } else {
        south = dd.end_lat;
        north = dd.start_lat;
    }

    let res = f32::abs(dd.lat_gap as f32);
    let mut bounds = LatLngBounds::new();
    let mut sw = LatLng::new();
    sw.lat = south as f32;
    sw.lng = west as f32;
    let mut ne = LatLng::new();
    ne.lat = north as f32;
    ne.lng = east as f32;
    bounds.set__northEast(ne);
    bounds.set__southWest(sw);

    let mut matrix = vec![crate::MISSING; dd.ni as usize * dd.nj as usize];

    if dd.start_lat < dd.end_lat {
        for (i, ddd) in dd.values.iter().enumerate() {
            matrix[i] = *ddd as f32;
        }
    } else {
        for (i, ddd) in dd.values.iter().enumerate() {
            let x = dd.nj as usize - 1 - i / dd.ni as usize;
            let y = i % dd.ni as usize;
            let idx = y * dd.ni as usize + x;
            matrix[idx] = *ddd as f32;
        }
    }

    let mut nom_grid = NomGrid::new();
    nom_grid.res = res;
    nom_grid.set_bounds(bounds);
    nom_grid.set_data(matrix);
    let data_bytes = nom_grid.write_to_bytes().unwrap();
    file.write_all(&data_bytes);
}
