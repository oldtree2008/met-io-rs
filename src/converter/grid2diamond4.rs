use crate::data_type::SingleGrid;
use chrono::prelude::*;
use rayon::prelude::*;
use std::fs::{create_dir_all, File};
use std::io::*;
use std::path::Path;

pub fn grids2diamond4s(grids: &Vec<SingleGrid>, output: &str) {
    grids.par_iter().for_each(|grid| {
        grid2diamond4(grid, output);
    });
}

/// output 是输出目录
pub fn grid2diamond4(grid: &SingleGrid, output: &str) {
    let datastr = format!("{}{}", grid.data_date, grid.data_time);
    let dt = Utc.datetime_from_str(&datastr, "%Y%m%d%H%M%S").unwrap();
    //todo
    let dst_file_name = if let Some(l) = &grid.level {
        format!(
            "{}/{}/{}/{}/{}{:02}.{:03}",
            output,
            &grid.product,
            &grid.element,
            l,
            &grid.data_date,
            &grid.data_time,
            &grid.forecast_time
        )
    } else {
        format!(
            "{}/{}/{}/{}{:02}.{:03}",
            output,
            &grid.product,
            &grid.element,
            &grid.data_date,
            &grid.data_time,
            &grid.forecast_time
        )
    };
    let path = Path::new(&dst_file_name);
    let parent = path.parent().unwrap();
    if !parent.exists() {
        create_dir_all(&parent).unwrap();
    }
    let file = File::create(&dst_file_name).unwrap();
    let mut buf = BufWriter::new(file);
    writeln!(buf, "diamond 4 {} ", grid.data_des).unwrap();
    //20200704_164546
    writeln!(
        buf,
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} ",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
        grid.lng_gap,
        grid.lat_gap,
        grid.start_lng,
        grid.end_lng,
        grid.start_lat,
        grid.end_lat,
        grid.ni,
        grid.nj
    )
    .unwrap();

    let mut min = f32::MAX;
    let mut max = f32::MIN;
    grid.values.iter().for_each(|&d| {
        if d != crate::MISSING {
            if d < min {
                min = d;
            }
            if d > max {
                max = d;
            }
        }
    });
    let step = (max - min) / 10.0;

    writeln!(buf, "{:.2} {:.2} {:.2} {} {} ", step, min, max, 0, 0).unwrap();
    for (i, v) in grid.values.iter().enumerate() {
        write!(buf, "{:.*} ", 2, v).unwrap();
        if (i + 1).rem_euclid(10usize) == 0 {
            writeln!(buf).unwrap();
        }
    }
    buf.flush();
}
