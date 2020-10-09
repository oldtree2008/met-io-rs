use std::f64::consts::PI;

pub struct MercatorTrans {
    pub east: f32,
    pub south: f32,
    pub west: f32,
    pub north: f32,
    pub res: f64,
    pub width: usize,
    pub height: usize,
}

const R: f64 = 6378.137f64;
const HALF_L: f64 = PI * R;

impl MercatorTrans {
    pub fn new(e: f32, s: f32, w: f32, n: f32, res: f64, width: usize, height: usize) -> Self {
        Self {
            east: e,
            south: s,
            west: w,
            north: n,
            res,
            width,
            height,
        }
    }

    fn latlon_to_meters(&self, lat: f64, lon: f64) -> (f64, f64) {
        let mx = lon * HALF_L / 180.0;
        let my = ((90.0 + lat) * PI / 360.0).tan().ln() / (PI / 180.0);
        let my = my * HALF_L / 180.0;
        (mx, my)
    }
    fn meters_to_latlon(&self, mx: f64, my: f64) -> (f64, f64) {
        let lon = (mx / HALF_L) * 180.0;
        let lat = (my / HALF_L) * 180.0;
        let lat = 180.0 / PI * (2.0 * (lat * PI / 180.0).exp().atan() - PI / 2.0);
        (lat, lon)
    }
    fn pixels_to_meters(&self, px: i32, py: i32, res: f64) -> (f64, f64) {
        let mx = px as f64 * res - HALF_L;
        let my = py as f64 * res - HALF_L;
        (mx, my)
    }
    fn meters_to_pixels(&self, mx: f64, my: f64, res: f64) -> (i32, i32) {
        let px = ((mx as f64 + HALF_L) / res) as i32;
        let py = ((my as f64 + HALF_L) / res) as i32;
        (px, py)
    }

    pub fn latlon_to_pixels(&self, lat: f64, lon: f64, res: f64) -> (i32, i32) {
        let (x, y) = self.latlon_to_meters(lat, lon);
        let (x, y) = self.meters_to_pixels(x, y, res);
        (x, y)
    }

    pub fn latlon2xycoords(&self, lat: f64, lon: f64) -> (i32, i32) {
        if lat >= self.south as f64
            && lat <= self.north as f64
            && lon >= self.west as f64
            && lon < self.east as f64
        {
            let (w, s) = self.latlon_to_meters(self.south as f64, self.west as f64);
            //west south 像素点坐标
            let (w, s) = self.meters_to_pixels(w, s, self.res as f64);

            let (e, n) = self.latlon_to_meters(self.north as f64, self.east as f64);
            //east north 像素点坐标
            let (e, n) = self.meters_to_pixels(e, n, self.res as f64);

            let (x, y) = self.latlon_to_meters(lat, lon);
            //lat lon 像素点坐标
            let (x, y) = self.meters_to_pixels(x, y, self.res as f64);

            //todo
            // dbg!(y, s, y - s, self.height);
            let (ix, iy) = ((x - w), self.height as i32 * 3 - (y - s) - 1);
            (ix, iy)
        } else {
            (-1, -1)
        }
    }
}
