use crate::SingleGrid;
use crate::{LatLng, LatLngBounds, NomGrid};
use chrono::prelude::*;
use std::fs::{create_dir_all, File};
use std::path::Path;

pub fn grid2nom(d: &SingleGrid, output: &str) {
    let dd;
    if d.lng_gap != d.lat_gap {
        dd = crate::normalize_grid(&d);
    } else {
        dd = d.clone();
    }

    let dd = &dd;
    let datastr = format!("{}{}", dd.data_date, dd.data_time);
    let dt = Utc.datetime_from_str(&datastr, "%Y%m%d%H%M%S").unwrap();

    let dst_file_name = if let Some(l) = &dd.level {
        format!(
            "{}/{}/{}/{}/{}{:02}.{:03}.NOM",
            output, &dd.product, &dd.element, l, &dd.data_date, &dd.data_time, &dd.forecast_time
        )
    } else {
        format!(
            "{}/{}/{}/{}{:02}.{:03}.NOM",
            output, &dd.product, &dd.element, &dd.data_date, &dd.data_time, &dd.forecast_time
        )
    };

    let path = Path::new(&dst_file_name);
    let parent = path.parent().unwrap();
    if !parent.exists() {
        create_dir_all(&parent).unwrap();
    }
    let file = File::create(&dst_file_name).unwrap();

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
    let bounds = LatLngBounds {
        _southWest: LatLng {
            lat: south as f32,
            lng: west as f32,
        },
        _northEast: LatLng {
            lat: north as f32,
            lng: east as f32,
        },
    };

    let mut matrix = vec![vec![crate::MISSING; dd.ni as usize]; dd.nj as usize];
    // let mut matrix = Vec::with_capacity(dd.nj as usize);

    // for _ in 0..dd.nj {
    //     let mut r = Vec::with_capacity(dd.ni as usize);
    //     for _ in 0..dd.ni {
    //         r.push(0f32);
    //     }
    //     matrix.push(r);
    // }

    if dd.start_lat < dd.end_lat {
        for (i, ddd) in dd.values.iter().enumerate() {
            let x = i / dd.ni as usize;
            let y = i % dd.ni as usize;
            matrix[x][y] = *ddd as f32;
        }
    } else {
        for (i, ddd) in dd.values.iter().enumerate() {
            let x = (dd.nj as usize - 1 - i) / dd.ni as usize;
            let y = i % dd.ni as usize;
            matrix[x][y] = *ddd as f32;
        }
    }

    let ng = NomGrid {
        res,
        bounds,
        data: matrix,
    };

    serde_json::to_writer(&file, &ng);
}
