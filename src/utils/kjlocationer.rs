#![allow(non_snake_case)]
pub struct KJLocationer {
    pub GWT_LAT0: f32,
    pub GWT_LON0: f32,
    pub GWT_NXL: i32,
    pub GWT_NYD: i32,
    pub GWT_XGD: f32,
    pub GWT_YGD: f32,
}

impl KJLocationer {
    pub fn new(type1: i32) -> Option<Self> {
        let GWT_LAT0;
        let GWT_LON0;
        let GWT_NXL;
        let GWT_NYD;
        let GWT_XGD;
        let GWT_YGD;
        if type1 == 1 {
            //总参大气所兰勃托投影参数 (大小范围：1280X1024   57N-15N  60E-155E)
            GWT_LAT0 = 34.0;
            GWT_LON0 = 105.0;
            GWT_NXL = 1280 / 2;
            GWT_NYD = 1024 / 2;
            GWT_XGD = 4.5;
            GWT_YGD = 4.5;
            Some({
                KJLocationer {
                    GWT_LAT0,
                    GWT_LON0,
                    GWT_NXL,
                    GWT_NYD,
                    GWT_XGD,
                    GWT_YGD,
                }
            })
        } else if type1 == 2 {
            //总参大气所兰勃托投影参数 (大小范围：1000X700   54N-15N  70E-150E)
            GWT_LAT0 = 34.0;
            GWT_LON0 = 110.0;
            GWT_NXL = 1000 / 2;
            GWT_NYD = 700 / 2;
            GWT_XGD = 6.2; //12.71 ;
            GWT_YGD = 6.2; //12.55 ;
            Some({
                KJLocationer {
                    GWT_LAT0,
                    GWT_LON0,
                    GWT_NXL,
                    GWT_NYD,
                    GWT_XGD,
                    GWT_YGD,
                }
            })
        } else if type1 == 3 {
            //  国家卫星气象中心AWX格式兰勃托投影参数
            GWT_LAT0 = 31.0;
            GWT_LON0 = 105.0;
            GWT_NXL = 768 / 2;
            GWT_NYD = 575 / 2;
            GWT_XGD = 13.42; //12.71 ;
            GWT_YGD = 13.11; //12.55 ;
            Some({
                KJLocationer {
                    GWT_LAT0,
                    GWT_LON0,
                    GWT_NXL,
                    GWT_NYD,
                    GWT_XGD,
                    GWT_YGD,
                }
            })
        } else {
            None
        }
    }
    pub fn lbt_lat_lon_to_xy_coord_proc(&self, lat: f32, lon: f32) -> (f64, f64) {
        let x1 = std::f32::consts::PI / 6.0;
        let x2 = std::f32::consts::PI / 3.0;
        let lon0 = self.GWT_LON0.to_radians();
        let lat0 = self.GWT_LAT0.to_radians();
        let b1 = f32::log10(f32::tan((std::f32::consts::PI / 2.0 - x1) / 2.0))
            - f32::log10(f32::tan((std::f32::consts::PI / 2.0 - x2) / 2.0));
        let gwt_en = f32::log10(f32::cos(x1)) / b1 - f32::log10(f32::cos(x2)) / b1;
        let gwt_ea = (6.371e3 / gwt_en * f32::cos(x2))
            / f32::powf(f32::tan(0.5 * (std::f32::consts::PI / 2.0 - x2)), gwt_en);
        let gwt_ej =
            -gwt_ea * f32::powf(f32::tan(0.5 * (std::f32::consts::PI / 2.0 - lat0)), gwt_en);
        let x0 = self.GWT_NXL as f64 * self.GWT_XGD as f64;
        let y0 = self.GWT_NYD as f64 * self.GWT_YGD as f64;
        let b1 = lon.to_radians();
        let a1 = lat.to_radians();
        let b0 = lon0;
        let a0 = f32::tan(0.5 * (std::f32::consts::PI / 2.0 - a1));
        let aa = f32::powf(a0, gwt_en);
        let bb = b1 - b0;
        let x1 = gwt_ea * aa * f32::sin(gwt_en * bb);
        let y1 = gwt_ea * aa * f32::cos(gwt_en * bb) + gwt_ej;
        let x = (x0 + x1 as f64) / self.GWT_XGD as f64;
        let y = (y0 + y1 as f64) / self.GWT_YGD as f64;

        (x as f64, y as f64)
    }

    pub fn lbt_grid_ij_to_lat_and_longitude_proc(&self, ix: f32, iy: f32) -> (f32, f32) {
        let x1 = std::f32::consts::PI / 6.0;
        let x2 = std::f32::consts::PI / 3.0;
        let lon0 = self.GWT_LON0.to_radians();
        let lat0 = self.GWT_LAT0.to_radians();
        let b1 = f32::log10(f32::tan((std::f32::consts::PI / 2.0 - x1) / 2.0))
            - f32::log10(f32::tan((std::f32::consts::PI / 2.0 - x2) / 2.0));
        let b = f32::log10(f32::cos(x1)) / b1 - f32::log10(f32::cos(x2)) / b1;
        let a = ((6371.0 / b) * f32::cos(x2))
            / (f32::powf(f32::tan(0.5 * (std::f32::consts::PI / 2.0 - x2)), b));
        let j0 = -(a
            * f32::cos(b * 0.0)
            * f32::powf(f32::tan(0.5 * (std::f32::consts::PI / 2.0 - lat0)), b))
            / self.GWT_YGD;
        let xx = (lon0
            + 1.0 / b
                * f64::atan(
                    (ix as f64 - self.GWT_NXL as f64) * self.GWT_XGD as f64
                        / ((-j0 as f64 + iy as f64 - self.GWT_NYD as f64) * self.GWT_YGD as f64),
                ) as f32)
            .to_degrees();
        let yy = (std::f32::consts::PI / 2.0
            - 2.0
                * f32::atan(f64::powf(
                    ((-j0 as f64 + iy as f64 - self.GWT_NYD as f64) * self.GWT_YGD as f64)
                        / (a as f64 * f64::cos(b as f64 * (xx.to_radians() - lon0) as f64)),
                    1.0 / b as f64,
                ) as f32))
        .to_degrees();
        (xx, yy)
    }
}
