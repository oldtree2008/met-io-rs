use crate::data_type::SingleGrid;
use crate::{MetError, ToGrids};
use chrono::prelude::*;
use rayon::prelude::*;
use std::fs::{create_dir_all, File};
use std::io::*;
use std::path::Path;
use std::result::Result;

pub fn todiamond4<T>(reader: &T, output: &str) -> Result<(), MetError>
where
    T: ToGrids,
{
    if let Some(grids) = reader.to_grids() {
        grids2diamond4s(&grids, output)
    } else {
        Err(MetError::ToGridsError)
    }
}

pub fn grids2diamond4s(grids: &Vec<SingleGrid>, output: &str) -> Result<(), MetError> {
    grids.iter().for_each(|grid| {
        grid2diamond4(grid, output).unwrap();
    });
    Ok(())
}

/// output 是输出目录
pub fn grid2diamond4(grid: &SingleGrid, output: &str) -> Result<(), MetError> {
    let datastr = format!("{}{}", grid.data_date, grid.data_time);
    let dt = Utc.datetime_from_str(&datastr, "%Y%m%d%H%M%S")?;
    //todo
    let dst_file_name = if let Some(l) = &grid.level {
        if grid.station.is_none() {
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
                "{}/{}/{}/{}/{}/{}{:02}.{:03}",
                output,
                &grid.product,
                &grid.station.as_ref().unwrap(),
                &grid.element,
                l,
                &grid.data_date,
                &grid.data_time,
                &grid.forecast_time
            )
        }
    } else {
        if grid.station.is_none() {
            format!(
                "{}/{}/{}/{}{:02}.{:03}",
                output,
                &grid.product,
                &grid.element,
                &grid.data_date,
                &grid.data_time,
                &grid.forecast_time
            )
        } else {
            format!(
                "{}/{}/{}/{}/{}{:02}.{:03}",
                output,
                &grid.product,
                grid.station.as_ref().unwrap(),
                &grid.element,
                &grid.data_date,
                &grid.data_time,
                &grid.forecast_time
            )
        }
    };
    let path = Path::new(&dst_file_name);
    let parent = path.parent().unwrap();
    if !parent.exists() {
        create_dir_all(&parent)?;
    }
    let file = File::create(&dst_file_name)?;
    let mut buf = BufWriter::new(file);

    let data_des = if let Some(station) = &grid.station {
        format!(
            "{}{}{}{}",
            &grid.data_date, &grid.data_time, station, grid.product
        )
    } else {
        format!("{}{}{}", &grid.data_date, &grid.data_time, grid.product)
    };

    writeln!(buf, "diamond 4 {} ", data_des)?;
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
    )?;

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

    writeln!(buf, "{:.2} {:.2} {:.2} {} {} ", step, min, max, 0, 0)?;
    for (i, v) in grid.values.iter().enumerate() {
        // if (*v >100.0 || *v< -100.0) && *v!=crate::MISSING {
        //     println!("wield {} ",v );
        // }
        write!(buf, "{:.*} ", 2, v)?;
        if (i + 1).rem_euclid(10usize) == 0 {
            writeln!(buf)?;
        }
    }
    buf.flush()?;
    Ok(())
}
