use image::{imageops, GenericImageView, ImageBuffer, RgbaImage};
use kdtree::distance::squared_euclidean;
use kdtree::ErrorKind;
use kdtree::KdTree;
use met_io_rs::Hdf5Reader;
use ndarray::prelude::*;
use palette::*;
use rayon::prelude::*;
use std::iter::FromIterator;
use std::path::Path;

fn point_value(lon: f32, lat: f32, lons: &[i16], lats: &[i16], values: &[i16]) -> f32 {
    let mut nominator = 0.0;
    let mut denominator = 0.0;
    // println!("begin  {} {}",lon,lat);
    for i in 0..values.len() {
        let dist1 = f32::abs(lon - lons[i] as f32 / 100.0);
        let dist2 = f32::abs(lat - lats[i] as f32 / 100.0);
        // println!("dist1 {}  dist2 {}",dist1,dist2);
        //         println!("dist1 {}  dist2 {}",dist1,dist2);
        // // println!("lon {}  loni {}",lon,lons[i] as f32/100.0);
        // // println!("lat {}  lati {}",lat,lats[i] as f32/100.0);
        if dist1 < 0.05 && dist2 < 0.05 {
            // println!("{}  {}  {} in",values[i] as f32/100.0,lon,lat);
            // let dist = f32::sqrt(
            //     (lon - lons[i] as f32) * (lon - lons[i] as f32)
            //         + (lat - lats[i] as f32) * (lat - lats[i] as f32),
            // );
            let dist = (lon - lons[i] as f32 / 100.0) * (lon - lons[i] as f32 / 100.0)
                + (lat - lats[i] as f32 / 100.0) * (lat - lats[i] as f32 / 100.0);

            // if dist < 0.5 {
            if dist < 0.0000000001 {
                return dist;
            }
            nominator = nominator + (values[i] as f32 / dist / 100.0);
            denominator = denominator + (1.0 / dist);
            // }
        }
    }
    if denominator > 0.0 {
        nominator / denominator
    } else {
        -9999.0
    }
}

pub fn main() {
    let fname = r##"H:\data\FY2E\FY2E_2018_11_01_00_31.HDF"##;
    // let p = Path::new("palette/V-01_x.xml");
    let p = Path::new("palette/I-01.xml");
    let pal = Palette::new_with_file(&p).unwrap();
    let reader = Hdf5Reader::new(fname);
    let attrs = reader.as_ref().unwrap().member_names();
    dbg!(attrs);
    let dataset = reader.as_ref().unwrap().dataset("Latitude");
    let data = dataset.as_ref();
    let lat = data.unwrap().read_2d::<i16>().unwrap();
    let lat_iter = lat.iter();

    let dataset = reader.as_ref().unwrap().dataset("Longitude");
    let data = dataset.as_ref();
    let lon = data.unwrap().read_2d::<i16>().unwrap();
    let lon_iter = lon.iter();

    let dataset = reader.as_ref().unwrap().dataset("ChannelIR1");
    let data = dataset.as_ref();
    let ir1 = data.unwrap().read_2d::<i16>().unwrap();
    let ir1_iter = ir1.iter();

    dbg!(ir1.len());
    dbg!(lat_iter.len());
    dbg!(lon_iter.len());

    let iter = lat_iter.zip(lon_iter).zip(ir1_iter);
    let mut lat_vec = Vec::new();
    let mut lon_vec = Vec::new();
    // let mut ir1_vec = Vec::new();
    let mut kdtree = KdTree::new(2);
    for (latlon, ir1) in iter {
        let lat = latlon.0;
        let lon = latlon.1;
        if *lat != 32765 && *lon != 32765 {
            lat_vec.push(*lat);
            lon_vec.push(*lon);
            // ir1_vec.push(*ir1);
            kdtree
                .add(
                    [*lon as f32 / 100.0, *lat as f32 / 100.0],
                    *ir1 as f32 / 100.0,
                )
                .unwrap();
            // println!("{}  {}  {}",*lon as f32/100.0, *lat as f32/100.0,*ir1 as f32/100.0);
        }
    }

    println!("kdtree created");
    let ret = kdtree
        .nearest(&[113.0, 0.0], 4, &squared_euclidean)
        .unwrap();
    let mut nom = 0.0;
    let mut denom = 0.0;
    for (dist, value) in ret.iter() {
        nom += *value / dist;
        denom += 1.0 / dist;
    }
    let v = nom / denom;

    println!(" {} ret {:?} ", v, ret);

    let lat_min = lat_vec.iter().min().unwrap();
    let lat_max = lat_vec.iter().max().unwrap();
    let lat_min = *lat_min as f32 / 100.0;
    let lat_max = *lat_max as f32 / 100.0;

    let lat_min = 0.0;
    let lat_max = 60.0;

    let step = 0.05;
    let rows = ((lat_max - lat_min) / step) as usize + 1;

    let lon_min = lon_vec.iter().min().unwrap();
    let lon_max = lon_vec.iter().max().unwrap();
    let lon_min = *lon_min as f32 / 100.0;
    let lon_max = *lon_max as f32 / 100.0;
    let lon_min = 70.0;
    let lon_max = 140.0;

    let cols = ((lon_max - lon_min) / step) as usize + 1;

    println!("{:?}  {:?}", lat_min, lat_max);
    println!("{:?}  {:?}", lon_min, lon_max);
    println!("{:?}  {:?}", rows, cols);

    let mut grid_lats = Vec::with_capacity(rows);
    for i in 0..rows {
        grid_lats.push(lat_max - i as f32 * step);
    }

    let mut grid_lons = Vec::with_capacity(cols);
    for i in 0..cols {
        grid_lons.push(lon_min + i as f32 * step);
    }
    let num = rows * cols;
    let mut grid_value = vec![0.0; num];
    grid_value.par_iter_mut().enumerate().for_each(|(i, d)| {
        let ii = i / cols;
        let jj = i % cols;
        // let v = point_value(grid_lons[jj], grid_lats[ii], &lon_vec, &lat_vec, &ir1_vec);
        let lon = grid_lons[jj];
        let lat = grid_lats[ii];
        let ret = kdtree.nearest(&[lon, lat], 4, &squared_euclidean).unwrap();
        let mut nom = 0.0;
        let mut denom = 0.0;
        for (dist, value) in ret.iter() {
            nom += *value / dist;
            denom += 1.0 / dist;
        }
        let v = nom / denom;
        // println!(" {} ret {:?} ",v, ret);
        *d = v;
    });

    // (0..rows*cols).into_par_iter().for_each(|i| {
    //             let v = point_value(
    //             grid_lons[i],
    //             grid_lats[i],
    //             &lon_vec,
    //             &lat_vec,
    //             &ir1_vec,
    //         );
    //         grid_value[i] = v;
    // })

    // for i in 0..rows {
    //     for j in 0..cols {
    //         let v = point_value(
    //             grid_lons[j],
    //             grid_lats[i],
    //             &lon_vec,
    //             &lat_vec,
    //             &ir1_vec,
    //         );
    //         grid_value.push(v);
    //         // println!("{}",v);

    //         println!("{}  {}  {}",grid_lons[j], grid_lats[i],v);
    //     }
    // }
    // let v = point_value(110.0, 40.0, &lon_vec, &lat_vec,&ir1_vec);
    // println!("{}",v);

    // println!("{:?} ",grid_value);

    // println!("gridvalue len {}",grid_value.len());

    // dbg!(dataset);
    // let shape = reader.as_ref().unwrap().shape("Latitude");
    // dbg!(shape);
    // let w = 2291;
    // let h = 2291;

    // let mut values = Vec::new();
    // data.mapv(|d| {
    //     values.push(d)
    // });

    // let ret:Vec<_> = values.iter().filter(|d| **d!=32765).collect();
    // // let ret:Vec<_> = values.iter().filter(|d| **d!=10745).collect();
    // // let mut iter = ret.iter();

    // println!("{}  {:?}  {:?}",ret.len(),ret.iter().min(),ret.iter().max());

    // let d = data.slice(s![0..2206,60..2231]);

    // println!("{:?}",d);
    // let mut value = Vec::new();
    // d.mapv(|aa|{
    //     println!("{}",aa);
    //     value.push(aa);
    // });
    // println!("{:?}",value.iter().min());
    let mut imgbuf = ImageBuffer::new(cols as u32, rows as u32);
    // // // // let dd = &data/100;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = y * cols as u32 + x;
        // let v = point_value(
        //     grid_lons[x as usize],
        //     grid_lats[y as usize],
        //     &lon_vec,
        //     &lat_vec,
        //     &ir1_vec,
        // );
        let v = grid_value[index as usize];
        let c = pal.get_color(v as f64).unwrap();
        *pixel = image::Rgba([c.r, c.g, c.b, c.a]);
    }
    imgbuf.save("IR1_1.png").unwrap();
}
