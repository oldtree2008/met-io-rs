pub fn interp_ppi(
    az: f32,
    r: f32,
    az_0: f32,
    az_1: f32,
    r_0: f32,
    r_1: f32,
    mat_00: f32,
    mat_01: f32,
    mat_10: f32,
    mat_11: f32,
) -> f32 {
    // 利用雷达扫描的周围四个点插值中间的点(az, r)
    // interp radar ppi scan data
    // az : target azimuth, units:degree
    // r : target range, units:meters
    // az_0 : grid start azimuth, units:degree
    // az_1 : grid end azimuth, units:degree
    // r_0 : grid start range , units : meters
    // r_1 : grid end range, units: meters
    // mat_00: data for [az_0, r_0]
    // mat_01: data for [az_0, r_1]
    // mat_10: data for [az_1, r_0]
    // mat_11: data for [az_1, r_1]
    // fillvalue: fillvalue for mat
    // return target value interped, units: like mat
    let interped;
    let fillvalue = crate::MISSING;
    if ((mat_00 != fillvalue) && (mat_01 != fillvalue))
        && ((mat_10 != fillvalue) && (mat_11 != fillvalue))
    {
        interped = (mat_00 * (az_1 - az) * (r_1 - r)
            + mat_10 * (az - az_0) * (r_1 - r)
            + mat_01 * (az_1 - az) * (r - r_0)
            + mat_11 * (az - az_0) * (r - r_0))
            / (r_1 - r_0)
            / (az_1 - az_0);
    } else if (mat_00 != fillvalue) && (mat_01 != fillvalue) {
        interped = (mat_00 * (r_1 - r) + mat_01 * (r - r_0)) / (r_1 - r_0);
    } else if (mat_10 != fillvalue) && (mat_11 != fillvalue) {
        interped = (mat_10 * (r_1 - r) + mat_11 * (r - r_0)) / (r_1 - r_0);
    } else if (mat_00 != fillvalue) && (mat_10 != fillvalue) {
        interped = (mat_00 * (az_1 - az) + mat_10 * (az - az_0)) / (az_1 - az_0);
    } else if (mat_01 != fillvalue) && (mat_11 != fillvalue) {
        interped = (mat_01 * (az_1 - az) + mat_11 * (az - az_0)) / (az_1 - az_0);
    } else {
        interped = fillvalue;
    }
    interped
}
//反距离插值
pub fn interp_azimuth(az: f32, az_0: f32, az_1: f32, dat_0: f32, dat_1: f32) -> f32 {
    let fillvalue = crate::MISSING;
    if (dat_0 - fillvalue).abs() > f32::EPSILON && (dat_1 - fillvalue).abs() > f32::EPSILON {
        return ((az_1 - az) * dat_0 + (az - az_0) * dat_1) / (az_1 - az_0);
    } else if dat_0 == fillvalue {
        return dat_1;
    } else {
        return dat_0;
    }
}
