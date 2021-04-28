use common_data::SingleGrid;
use contour::ContourBuilder;
use geojson::Feature;
use rayon::collections::vec_deque;

pub fn contour(data: &SingleGrid, levels: &Vec<f64>) -> Vec<Feature> {
    dbg!("contour");
    let dx = data.ni;
    let dy = data.nj;
    let datas = &data.values;
    let west = data.start_lng;
    let east = data.end_lng;
    let north = data.end_lat;
    let south = data.start_lat;
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    let datas = datas
        .iter()
        .map(|&d| {
            if d == 9999.0 {
                f64::NAN
            } else {
                let d = d as f64;
                if d > max {
                    max = d;
                }
                if d < min {
                    min = d;
                }
                d
            }
        })
        .collect::<Vec<f64>>();
    let builder = ContourBuilder::new(dx as u32, dy as u32, true);
    println!("col {}  row {}   min {}  max {}", dx, dy, min, max);
    let step = (max - min) / 10.0;
    let mut levels = vec![0.0; 10];
    levels.iter_mut().enumerate().for_each(|(idx, d)| {
        *d = min + idx as f64 * step;
    });
    dbg!(&levels);
    let mut res = builder.contours(&datas, &levels);
    res.unwrap()
}
