pub fn antenna_to_cartesian(ranges: f32, azimuths: f32, elevations: f32) -> (f32, f32, f32) {
    let theta_e = elevations * std::f32::consts::PI / 180.0; //# elevation angle in radians.
    let theta_a = azimuths * std::f32::consts::PI / 180.0; //     # azimuth angle in radians.
    let R: f32 = 6371.0 * 1000.0 * 4.0 / 3.0; //     # effective radius of earth in meters.
    let r = ranges * 1.0; //  # distances to gates in meters.
                          // let z = (r ** 2.0 + R ** 2.0 + 2.0 * r * R * f32::sin(theta_e)) ** 0.5 - R;
    let z = (r.powf(2.0) + R.powf(2.0) + 2.0 * r * R * f32::sin(theta_e)).powf(0.5) - R;
    let s = R * f32::asin(r * f32::cos(theta_e) / (R + z)); //  # arc length in m.
    let x = s * f32::sin(theta_a);
    let y = s * f32::cos(theta_a);

    (x, y, z)
}
pub fn antenna_to_cartesian_cwr(
    ranges: f32,
    azimuths: f32,
    elevations: f32,
    h: f32,
) -> (f32, f32, f32) {
    let theta_e = elevations.to_radians(); //# elevation angle in radians.
    let theta_a = azimuths.to_radians(); //     # azimuth angle in radians.
    let R: f32 = 6371.0 * 1000.0 * 4.0 / 3.0; //     # effective radius of earth in meters.
    let r = ranges * 1.0; //  # distances to gates in meters.
                          // z = ((r * np.cos(theta_e)) ** 2 + (R + h + r * np.sin(theta_e)) ** 2) ** 0.5 - R
    let z = ((r * f32::cos(theta_e)).powf(2.0) + (R + h + r * f32::sin(theta_e)).powf(2.0))
        .powf(0.5)
        - R;
    let s = R * f32::asin(r * f32::cos(theta_e) / (R + z)); //  # arc length in m.

    // println!("s {}", s);
    let x = s * f32::sin(theta_a);
    let y = s * f32::cos(theta_a);

    (x, y, z)
}

pub fn cartesian_xyz_to_antenna(x: f32, y: f32, z: f32, h: f32) -> (f32, f32, f32) {
    // """
    // 根据采样点距离雷达的x,y的水平距离,以及高度z, 以及雷达高度h
    // x, units:meters
    // y, units:meters
    // z, units:meters
    // h, units:meters
    // return ranges, azimuth, elevation
    // """
    let R = 8494666.6666666661; // #等效地球半径
                                //((R + h) ** 2 + (R + z) ** 2 - 2 * (R + h) * (R + z) * np.cos((x ** 2 + y ** 2) ** 0.5 / R)) ** 0.5
                                // let ranges = ((R + h).powf(2.0) + (R + z).powf(2.0)
                                //     - 2.0 * (R + h) * (R + z) * f32::cos((x.powf(2.0) + y.powf(2.0)).powf(0.5) / R))
                                // .powf(0.5);
    let h = h as f64;
    let R = R as f64;
    let z = z as f64;
    let x = x as f64;
    let y = y as f64;

    let v1: f64 = ((R + h) as f64).powf(2.0);
    let v2: f64 = ((R + z) as f64).powf(2.0);
    let v3: f64 = 2.0 * (R + h) * (R + z) * f64::cos((x.powf(2.0) + y.powf(2.0)).powf(0.5) / R);

    let vv = v1 + v2 - v3 as f64;

    // let vv = vv.powf(0.5);
    let ranges = f64::sqrt(vv);
    // println!("range {} {} {} {} ", v1, v2, v3, vv);

    let elevation: f64 =
        f64::acos((R + z) * f64::sin((x.powf(2.0) + y.powf(2.0)).powf(0.5) / R) / ranges) * 180.
            / std::f64::consts::PI;
    let azimuth = _azimuth(x as f32, y as f32);
    let azimuth = azimuth.to_degrees();
    (azimuth, ranges as f32, elevation as f32)
}
fn _azimuth(x: f32, y: f32) -> f32 {
    // '''根据某一点距离雷达x方向，y方向的距离，计算方位角，单位：弧度'''
    let mut az = std::f32::consts::PI / 2.0 - y.atan2(x); //np.angle(x + y * 1j)
    if az < 0.0 {
        az += 2.0 * std::f32::consts::PI;
    }
    az
}

pub fn cartesian_to_antenna_cwr(x: f32, y: f32, elevation: f32, h: f32) -> (f32, f32, f32) {
    // """根据采样点距离雷达的x,y的水平距离,以及雷达仰角
    // return x, y, z
    // 和高度,计算该点雷达的斜距
    // ..math::
    //     s = sqrt(x^2 + y^2)
    //     r = sin(s/R)*(R+h)/cos(elevation)
    //     R为地球半径m,h为雷达高度m,elevation为仰角degree
    // """
    let R = 6371.0 * 1000.0 * 4.0 / 3.0; //# effective radius of earth in meters.
    let s = f32::sqrt(x * x + y * y);
    let El = elevation.to_radians();
    let ranges = f32::tan(s / R) * (R + h) / f32::cosh(El);
    let z = (R + h) / f32::cos(El + s / R) * f32::cos(El) - R; //##计算高度
    let az = _azimuth(x, y); //##计算方位角
    (az, ranges, z)
}

pub fn geographic_to_cartesian_aeqd(lon: f32, lat: f32, lon_0: f32, lat_0: f32) -> (f32, f32) {
    let R = 6370997.0;
    let lon_rad = f32::to_radians(lon);
    let lat_rad = f32::to_radians(lat);
    let lat_0_rad = f32::to_radians(lat_0);
    let lon_0_rad = f32::to_radians(lon_0);
    let lon_diff_rad = lon_rad - lon_0_rad;
    let mut arg_arccos =
        lat_0_rad.sin() * lat_rad.sin() + lat_0_rad.cos() * lat_rad.cos() * lon_diff_rad.cos();
    if arg_arccos > 1.0f32 {
        arg_arccos = 1.0;
    }
    if arg_arccos < -1.0f32 {
        arg_arccos = -1.0;
    }
    let c = arg_arccos.acos();
    let k;
    if c == 0.0 {
        k = 1.0;
    } else {
        k = c / c.sin();
    }
    let x = R * k * lat_rad.cos() * lon_diff_rad.sin();
    let y = R
        * k
        * (lat_0_rad.cos() * lat_rad.sin() - lat_0_rad.sin() * lat_rad.cos() * lon_diff_rad.cos());
    (x, y)
}

pub fn cartesian_to_geographic_aeqd(x: f32, y: f32, lon_0: f32, lat_0: f32) -> (f32, f32) {
    let R = 6370997.0;
    let lat_0_rad = f32::to_radians(lat_0);
    let lon_0_rad = f32::to_radians(lon_0);
    let rho = f32::sqrt(x * x + y * y);
    let c = rho / R;
    let lat_deg;
    if rho != 0.0 {
        let lat_rad = f32::asin(c.cos() * lat_0_rad.sin() + y * c.sin() * lat_0_rad.cos() / rho);
        lat_deg = lat_rad.to_degrees();
    } else {
        lat_deg = lat_0;
    }
    let x1 = x * c.sin();
    let x2 = rho * f32::cos(lat_0_rad) * f32::cos(c) - y * f32::sin(lat_0_rad) * f32::sin(c);
    let lon_rad = lon_0_rad + f32::atan2(x1, x2);
    let mut lon_deg = f32::to_degrees(lon_rad);
    if lon_deg > 180.0 {
        lon_deg -= 360.0
    }
    if lon_deg < -180.0 {
        lon_deg += 360.0;
    }
    (lon_deg, lat_deg)
}
