use crate::interplate;
use crate::SingleGrid;
use rayon::prelude::*;

/// 将lon和lat 间隔不同的格点，转换为格距相同的格点。
/// 小徐的格式要求
pub fn normalize_grid(d: &SingleGrid) -> SingleGrid {
    let mut new_grid = d.clone();
    // let gap = (d.lat_gap + d.lng_gap) / 2.0;
    let gap = f64::min(d.lat_gap, d.lng_gap);
    let nj = (d.end_lat - d.start_lat) / gap;
    let nj = nj as i64 + 1;
    let ni = (d.end_lng - d.start_lng) / gap;
    let ni = ni as i64 + 1;

    let end_lat = d.start_lat + (nj - 1) as f64 * gap;
    let end_lng = d.start_lng + (ni - 1) as f64 * gap;

    let mut values = vec![crate::MISSING; (ni * nj) as usize];
    let data_len = d.values.len();
    // dbg!(&values[0]);
    // d.values.iter().for_each(|d| {
    //     if d.is_nan() {
    //         println!("IS nan");
    //     }
    // });

    values.iter_mut().enumerate().for_each(|(i, vv)| {
        let r = i / ni as usize;
        let c = i % ni as usize;
        let vlat = r as f64 * gap;
        let vlon = c as f64 * gap;
        let oldr = vlat / d.lat_gap;
        let oldc = vlon / d.lng_gap;
        let oldr0 = oldr.floor();
        let oldr1 = oldr0 + 1.0;
        let oldc0 = oldc.floor();
        let oldc1 = oldc0 + 1.0;

        let idx00 = oldr0 * d.ni as f64 + oldc0;
        let idx00 = idx00 as usize;

        let idx01 = oldr0 * d.ni as f64 + oldc1;
        let idx01 = idx01 as usize;

        let idx10 = oldr1 * d.ni as f64 + oldc0;
        let idx10 = idx10 as usize;

        let idx11 = oldr1 * d.ni as f64 + oldc1;
        let idx11 = idx11 as usize;

        // println!("{} {} {} {} {} {}  {}",idx00,idx01,idx10,idx11,idx10-idx00,idx11-idx01,ni);
        // let indx = oldr * d.ni as f64 + oldc;
        // let indx  = indx as usize;
        // if indx < data_len {
        //     *vv = d.values[indx];
        // }
        if idx00 < data_len && idx01 < data_len && idx10 < data_len && idx11 < data_len {
            let data00 = d.values[idx00];
            let data01 = d.values[idx01];
            let data10 = d.values[idx10];
            let data11 = d.values[idx11];

            let mut v = interplate::interp_ppi(
                oldr as f32,
                oldc as f32,
                oldr0 as f32,
                oldr1 as f32,
                oldc0 as f32,
                oldc1 as f32,
                data00,
                data01,
                data10,
                data11,
            );
            if v.is_nan() {
                v = crate::MISSING;
                println!(
                    "MISSING {} {} {} {}  {} {} {} {}",
                    data00, data01, data10, data11, oldr0, oldr1, oldc0, oldc1
                );
            }
            *vv = v;
        }
    });
    // dbg!(&values[0]);
    new_grid.ni = ni;
    new_grid.nj = nj;
    new_grid.end_lat = end_lat;
    new_grid.end_lng = end_lng;
    new_grid.values = values;
    new_grid.lat_gap = gap;
    new_grid.lng_gap = gap;

    new_grid
}
