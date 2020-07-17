use crate::SingleGrid;
use crate::Hdf5Reader;
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use rayon::prelude::*;

pub struct KJH5SatReader;
impl KJH5SatReader {
    pub fn read(fname:&str)->Option<Vec<SingleGrid>> {
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
    
        let dataset = reader.as_ref().unwrap().dataset("ChannelIR2");
        let data = dataset.as_ref();
        let ir2 = data.unwrap().read_2d::<i16>().unwrap();
        let ir2_iter = ir2.iter();
    
        let dataset = reader.as_ref().unwrap().dataset("ChannelIR3");
        let data = dataset.as_ref();
        let ir3 = data.unwrap().read_2d::<i16>().unwrap();
        let ir3_iter = ir3.iter();
    
        let dataset = reader.as_ref().unwrap().dataset("ChannelIR4");
        let data = dataset.as_ref();
        let ir4 = data.unwrap().read_2d::<i16>().unwrap();
        let ir4_iter = ir4.iter();
    
        let dataset = reader.as_ref().unwrap().dataset("ChannelVIS");
        let data = dataset.as_ref();
        let vis = data.unwrap().read_2d::<i16>().unwrap();
        let vis_iter = vis.iter();
        let iter = lat_iter
            .zip(lon_iter)
            .zip(ir1_iter)
            .zip(ir2_iter)
            .zip(ir3_iter)
            .zip(ir4_iter)
            .zip(vis_iter);
        let mut kdtree = KdTree::new(2);
    
        let mut min_lat = f32::MAX;
        let mut max_lat = f32::MIN;
        let mut min_lon = f32::MAX;
        let mut max_lon = f32::MIN;
        let start_lat = 0.0;
        let end_lat = 60.0;
        let start_lng = 70.0;
        let end_lng = 140.0;
    
        iter.for_each(|((((((lat, lon), ir1), ir2), ir3), ir4), vis)| {
            if *lat != 32765 && *lon != 32765 {
                let vlat = *lat as f32 / 100.0;
                let vlon = *lon as f32 / 100.0;
                // if *ir1 <0  {
                //     dbg!(ir1);
                // }
                if vlat < min_lat {
                    min_lat = vlat;
                }
                if vlat > max_lat {
                    max_lat = vlat;
                }
                if vlon < min_lon {
                    min_lon = vlon;
                }
                if vlon > max_lon {
                    max_lon = vlon;
                }
    
                if vlat >= start_lat && vlat <= end_lat && vlon >= start_lng && vlon <= end_lng {
                    kdtree
                        .add(
                            [vlon, vlat],
                            [
                                *ir1 as f32 / 100.0,
                                *ir2 as f32 / 100.0,
                                *ir3 as f32 / 100.0,
                                *ir4 as f32 / 100.0,
                                *vis as f32 / 100.0,
                            ],
                        )
                        .unwrap();
                }
            }
        });
        println!("kdtree created");
        dbg!(min_lat, max_lat, min_lon, max_lon);
    
        let step = 0.05;
        let rows = (end_lat - start_lat) / 0.05;
        let rows = rows as usize + 1;
        let cols = (end_lng - start_lng) / 0.05;
        let cols = cols as usize + 1;
    
        let mut grid_values = vec![vec![0.0f32; 5]; rows * cols];
    
        grid_values.par_iter_mut().enumerate().for_each(|(i, d)| {
            let r = i / cols;
            let c = i % cols;
            let lat = start_lat + r as f32 * step;
            let lon = start_lng + c as f32 * step;
            let ret = kdtree.nearest(&[lon, lat], 4, &squared_euclidean).unwrap();
            let mut nom = vec![0.0; 5];
            let mut denom = vec![0.0; 5];
            for (dist, value) in ret.iter() {
                for i in 0..5 {
                    // if value[i] <0.0 {
                    //     dbg!(value[i]);
                    // }
                    if *dist == 0.0 {
                        nom[i] += value[i] / f32::MIN;
                        denom[i] += 1.0 / f32::MIN;
                    } else {
                        nom[i] += value[i] / dist;
                        denom[i] += 1.0 / dist;
                    }
                }
            }
            let mut v = vec![0.0; 5];
            for i in 0..5 {
                v[i] = nom[i] / denom[i];
            }
            *d = v;
        });
    
        let mut last_values = vec![vec![0f32; rows * cols]; 5];
        last_values.par_iter_mut().enumerate().for_each(|(i, d)| {
            *d = grid_values.iter().map(|gv| gv[i]).collect::<Vec<_>>();
        });
    
        let elements = vec!["IR1", "IR2", "IR3", "IR4", "VIS"];
    
        let mut sgrids = Vec::new();
        elements.iter().enumerate().for_each(|(i, e)| {
            let sgrid = SingleGrid::<_, f32> {
                ni: cols as i64,
                nj: rows as i64,
                start_lat: start_lat as f64,
                start_lng: start_lng as f64,
                end_lat: end_lat as f64,
                end_lng: end_lng as f64,
                lat_gap: step as f64,
                lng_gap: step as f64,
                level: None,
                element: String::from(*e),
                center: String::new(),
                product: String::from("FY2E"),
                data_des: String::from("FY2E_2018_11_01_00_31"),
                values: last_values[i].clone(),
                data_date: String::from("20181101"),
                data_time: String::from("010031"),
                forecast_time: 0,
            };
            sgrids.push(sgrid);
        });
        Some(sgrids)
    }    
}