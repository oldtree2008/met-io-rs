use crate::NomGrid;
use image::ImageBuffer;
use palette::*;
use std::path::Path;

pub fn nom2img(grid: &NomGrid, palfile: &str, output: &str) {
    // let p = Path::new("palette/xradar.xml");
    let res = grid.res;
    let bounds = &grid.bounds;
    let _southWest = &bounds._southWest;
    let _northEast = &bounds._northEast;
    let south = _southWest.lat;
    let north = _northEast.lat;

    let west = _southWest.lng;
    let east = _northEast.lng;

    let ni = f32::round((east - south) / res);
    let w = ni as u32;

    let nj = f32::round((north - south) / res);
    let h = nj as u32;

    let p = Path::new(palfile);
    let pal = Palette::new_with_file(&p).unwrap();
    // let w = &data.0;
    // let h = &data.1;
    // let w = *w as u32;
    // let h = *h as u32;
    let mut data = Vec::new();

    for d in &grid.data {
        for dd in d {
            data.push(*dd);
        }
    }
    let w = grid.data[0].len() as u32;
    let h = grid.data.len() as u32;

    let count = w * h;
    println!(
        "{}    {}   {}  {}",
        grid.data.len() * grid.data[0].len(),
        data.len(),
        w,
        h
    );

    // let grid_value = &grid.data;

    let mut imgbuf = ImageBuffer::new(w, h);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = y * w + x;
        // if index < count {
        let v = data[index as usize];
        let c = pal.get_color(v as f64).unwrap();
        *pixel = image::Rgba([c.r, c.g, c.b, c.a]);
        // }
    }
    imgbuf.save(output).unwrap();
}
