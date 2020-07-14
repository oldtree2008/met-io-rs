use image::{imageops, GenericImageView, ImageBuffer, RgbaImage};
use palette::*;
use std::path::Path;
pub fn grid2img(data: &(usize, usize, Vec<f32>), palfile: &str, output: &str) {
    // let p = Path::new("palette/xradar.xml");
    let p = Path::new(palfile);
    let pal = Palette::new_with_file(&p).unwrap();
    let w = &data.0;
    let h = &data.1;
    let w = *w as u32;
    let h = *h as u32;
    let grid_value = &data.2;
    let mut imgbuf = ImageBuffer::new(w, h);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = y * w + x;
        let v = grid_value[index as usize];
        let c = pal.get_color(v as f64).unwrap();
        *pixel = image::Rgba([c.r, c.g, c.b, c.a]);
    }
    imgbuf.save(output).unwrap();
}
