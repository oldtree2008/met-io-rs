use crate::error::MetError;
use crate::Hdf5Reader;
use crate::{SingleGrid, ToGrids};
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use ndarray::Ix3;
use rayon::prelude::*;
use std::result::Result;
//检测预警产品 颠簸、结冰、雷暴
pub struct WarnH5Reader(Hdf5Reader);
impl WarnH5Reader {
    pub fn new(fname: &str) -> Result<WarnH5Reader, MetError> {
        let reader = Hdf5Reader::new(fname)?;
        Ok(WarnH5Reader(reader))
    }
}

impl ToGrids for WarnH5Reader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let reader = &self.0;
        let vars = reader.member_names();
        dbg!(vars);
        let attrs = reader.attribute_names();
        dbg!(attrs);

        // let start_time = &begin_time[8..12];
        let dataset = reader.dataset("Product_Name").unwrap();
        let len = dataset.ndim();

        dbg!(dataset.shape());
        let product_name = if len == 1 {
            let p = dataset
                .read_1d::<hdf5::types::FixedAscii<[u8; 16]>>()
                .unwrap();
            p[0]
        } else {
            dataset
                .read_scalar::<hdf5::types::FixedAscii<[u8; 16]>>()
                .unwrap()
        };
        // let product_name = product_name[0];
        // dbg!( product_name);

        if product_name == "DB" {
            let dataset = reader.dataset("Begin_Time").unwrap();
            // dbg!(dataset.shape());
            let begin_time = dataset
                .read_1d::<hdf5::types::FixedAscii<[u8; 16]>>()
                .unwrap();
            let begin_time = begin_time[0].as_str();

            let dataset = reader.dataset("Sat_Name").unwrap();
            // dbg!(dataset.shape());
            let sat_name = dataset
                .read_1d::<hdf5::types::FixedAscii<[u8; 16]>>()
                .unwrap();
            // dbg!( sat_name);
            let sat_name = sat_name[0];

            let (start_date, start_time) = begin_time.split_at(8);
            dbg!(start_date, start_time);
            let dataset = reader.dataset("Latitude");
            let data = dataset.as_ref();
            let lat = data.unwrap().read_2d::<i16>().unwrap();
            let lat_iter = lat.iter();

            let dataset = reader.dataset("Longitude");
            let data = dataset.as_ref();
            let lon = data.unwrap().read_2d::<i16>().unwrap();
            let lon_iter = lon.iter();

            let dataset = reader.dataset("DB_JC");
            let data = dataset.as_ref();
            let db_jc = data.unwrap().read_2d::<i16>().unwrap();
            let db_jc_iter = db_jc.iter();
            let dataset = reader.dataset("DB_YJ");
            let data = dataset.as_ref();
            let db_yj = data.unwrap().read_2d::<i16>().unwrap();
            let db_yj_iter = db_yj.iter();

            let iter = lat_iter.zip(lon_iter).zip(db_jc_iter).zip(db_yj_iter);
            let mut kdtree = KdTree::new(2);

            let mut min_lat = f32::MAX;
            let mut max_lat = f32::MIN;
            let mut min_lon = f32::MAX;
            let mut max_lon = f32::MIN;
            let start_lat = 0.0;
            let end_lat = 60.0;
            let start_lng = 70.0;
            let end_lng = 140.0;

            iter.for_each(|(((lat, lon), db_jc), db_yj)| {
                if *lat != 327 && *lon != 327 {
                    // dbg!(*lat, *lon, *db_jc, *db_yj);
                    let vlat = *lat as f32;
                    let vlon = *lon as f32;
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

                    // if vlat >= start_lat && vlat <= end_lat && vlon >= start_lng && vlon <= end_lng
                    // {
                    kdtree
                        .add([vlon, vlat], [*db_jc as f32, *db_yj as f32])
                        .unwrap();
                    // }
                }
            });

            dbg!(min_lat, max_lat, min_lon, max_lon);
            let start_lat = min_lat;
            let end_lat = max_lat;
            let start_lng = min_lon;
            let end_lng = max_lon;

            println!("kdtree created");

            let step = 0.5;
            let rows = (end_lat - start_lat) / 0.5;
            let rows = rows as usize + 1;
            let cols = (end_lng - start_lng) / 0.5;
            let cols = cols as usize + 1;

            let mut grid_values = vec![vec![crate::MISSING; 2]; rows * cols];

            grid_values.par_iter_mut().enumerate().for_each(|(i, d)| {
                let r = i / cols;
                let c = i % cols;
                let lat = start_lat + r as f32 * step;
                let lon = start_lng + c as f32 * step;
                let ret = kdtree.nearest(&[lon, lat], 4, &squared_euclidean).unwrap();
                let mut nom = vec![0.0; 2];
                let mut denom = vec![0.0; 2];
                for (dist, value) in ret.iter() {
                    for i in 0..2 {
                        // if value[i] <0.0 {
                        //     dbg!(value[i]);
                        // }
                        if *dist == 0.0 {
                            nom[i] += value[i] / f32::MIN;
                            denom[i] += 1.0 / f32::MIN;
                        } else {
                            // if *dist>1.0 {
                            //     println!("{}",*dist);
                            // }
                            nom[i] += value[i] / dist;
                            denom[i] += 1.0 / dist;
                        }
                    }
                }
                let mut v = vec![0.0; 2];
                for i in 0..2 {
                    v[i] = nom[i] / denom[i];
                }
                *d = v;
            });

            let mut last_values = vec![vec![crate::MISSING; rows * cols]; 2];
            last_values.par_iter_mut().enumerate().for_each(|(i, d)| {
                *d = grid_values.iter().map(|gv| gv[i]).collect::<Vec<_>>();
            });

            let elements = vec!["DB_JC", "DB_YJ"];

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
                    product: format!("{}/{}", product_name, sat_name),
                    station: None,
                    values: last_values[i].clone(),
                    data_date: format!("{}", start_date),
                    data_time: format!("{}00", start_time),
                    forecast_time: 0,
                };
                sgrids.push(sgrid);
            });
            Some(sgrids)
        } else if product_name == "LB" {
            let dataset = reader.dataset("Begin_Time").unwrap();
            // dbg!(dataset.shape());
            let begin_time = dataset
                .read_scalar::<hdf5::types::FixedAscii<[u8; 16]>>()
                .unwrap();

            let dataset = reader.dataset("Sat_Name").unwrap();
            // dbg!(dataset.shape());
            let sat_name = dataset
                .read_scalar::<hdf5::types::FixedAscii<[u8; 16]>>()
                .unwrap();
            dbg!(sat_name);

            let (start_date, start_time) = begin_time.split_at(8);
            dbg!(start_date, start_time);
            let dataset = reader.dataset("Lat");
            let data = dataset.as_ref();
            let lat = data.unwrap().read_2d::<i16>().unwrap();
            let lat_iter = lat.iter();

            let dataset = reader.dataset("Lon");
            let data = dataset.as_ref();
            let lon = data.unwrap().read_2d::<i16>().unwrap();
            let lon_iter = lon.iter();
            let dataset = reader.dataset("LB_JC");
            let data = dataset.as_ref();
            let iter1 = data.unwrap().read_2d::<i16>().unwrap();
            let iter1 = iter1.iter();
            let dataset = reader.dataset("LB_ZZ_DIR");
            let data = dataset.as_ref();
            let iter2 = data.unwrap().read_2d::<i16>().unwrap();
            let iter2 = iter2.iter();
            let dataset = reader.dataset("LB_ZZ_INS");
            let data = dataset.as_ref();
            let iter3 = data.unwrap().read_2d::<i16>().unwrap();
            let iter3 = iter3.iter();
            let dataset = reader.dataset("LB_ZZ_VEL");
            let data = dataset.as_ref();
            let iter4 = data.unwrap().read_2d::<i16>().unwrap();
            let iter4 = iter4.iter();

            let iter = lat_iter
                .zip(lon_iter)
                .zip(iter1)
                .zip(iter2)
                .zip(iter3)
                .zip(iter4);
            let mut kdtree = KdTree::new(2);

            let mut min_lat = f32::MAX;
            let mut max_lat = f32::MIN;
            let mut min_lon = f32::MAX;
            let mut max_lon = f32::MIN;
            let start_lat = 0.0;
            let end_lat = 60.0;
            let start_lng = 70.0;
            let end_lng = 140.0;

            iter.for_each(|(((((lat, lon), ir1), ir2), ir3), ir4)| {
                if *lat != 32765 && *lon != 32765 {
                    let vlat = *lat as f32;
                    let vlon = *lon as f32;
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

                    // if vlat >= start_lat && vlat <= end_lat && vlon >= start_lng && vlon <= end_lng
                    // {
                    kdtree
                        .add(
                            [vlon, vlat],
                            [*ir1 as f32, *ir2 as f32, *ir3 as f32, *ir4 as f32],
                        )
                        .unwrap();
                    // }
                }
            });
            println!("kdtree created");
            dbg!(min_lat, max_lat, min_lon, max_lon);
            let start_lat = min_lat;
            let end_lat = max_lat;
            let start_lng = min_lon;
            let end_lng = max_lon;

            let step = 0.5;
            let rows = (end_lat - start_lat) / 0.5;
            let rows = rows as usize + 1;
            let cols = (end_lng - start_lng) / 0.5;
            let cols = cols as usize + 1;

            let mut grid_values = vec![vec![0.0f32; 4]; rows * cols];

            grid_values.par_iter_mut().enumerate().for_each(|(i, d)| {
                let r = i / cols;
                let c = i % cols;
                let lat = start_lat + r as f32 * step;
                let lon = start_lng + c as f32 * step;
                let ret = kdtree.nearest(&[lon, lat], 4, &squared_euclidean).unwrap();
                let mut nom = vec![0.0; 4];
                let mut denom = vec![0.0; 4];
                for (dist, value) in ret.iter() {
                    for i in 0..4 {
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
                let mut v = vec![0.0; 4];
                for i in 0..4 {
                    v[i] = nom[i] / denom[i];
                }
                *d = v;
            });

            let mut last_values = vec![vec![0f32; rows * cols]; 4];
            last_values.par_iter_mut().enumerate().for_each(|(i, d)| {
                *d = grid_values.iter().map(|gv| gv[i]).collect::<Vec<_>>();
            });

            let elements = vec!["LB_JC", "LB_ZZ_DIR", "LB_ZZ_INS", "LB_ZZ_VEL"];

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
                    product: format!("{}/{}", product_name, sat_name),
                    station: None,
                    values: last_values[i].clone(),
                    data_date: format!("{}", start_date),
                    data_time: format!("{}00", start_time),
                    forecast_time: 0,
                };
                sgrids.push(sgrid);
            });
            Some(sgrids)
        } else if product_name == "JB" {
            let dataset = reader.dataset("Begin_Time").unwrap();
            // dbg!(dataset.shape());
            let begin_time = dataset
                .read_scalar::<hdf5::types::FixedAscii<[u8; 16]>>()
                .unwrap();

            let dataset = reader.dataset("Sat_Name").unwrap();
            // dbg!(dataset.shape());
            let sat_name = dataset
                .read_scalar::<hdf5::types::FixedAscii<[u8; 16]>>()
                .unwrap();
            dbg!(sat_name);
            let (start_date, start_time) = begin_time.split_at(8);
            dbg!(start_date, start_time);
            let dataset = reader.dataset("Latitude");
            let data = dataset.as_ref();
            let lat = data.unwrap().read_2d::<i16>().unwrap();

            let dataset = reader.dataset("Longitude");
            let data = dataset.as_ref();
            let lon = data.unwrap().read_2d::<i16>().unwrap();

            let dataset = reader.dataset("JB_JC");
            let data = dataset.as_ref();
            let out1 = data.unwrap().read::<i16, Ix3>().unwrap();
            let out1_iter = out1.outer_iter();
            let dataset = reader.dataset("JB_YJ");
            let data = dataset.as_ref();
            let out2 = data.unwrap().read::<i16, Ix3>().unwrap();
            let out2_iter = out2.outer_iter();
            let out_iter = out1_iter.zip(out2_iter);
            let mut count = 0;
            let mut sgrids = Vec::new();

            out_iter.for_each(|(out1, out2)| {
                let lat_iter = lat.iter();
                let lon_iter = lon.iter();
                let d1_iter = out1.iter();
                let d2_iter = out2.iter();
                let iter = lat_iter.zip(lon_iter).zip(d1_iter).zip(d2_iter);
                let mut kdtree = KdTree::new(2);

                let mut min_lat = f32::MAX;
                let mut max_lat = f32::MIN;
                let mut min_lon = f32::MAX;
                let mut max_lon = f32::MIN;
                // let start_lat = 0.0;
                // let end_lat = 60.0;
                // let start_lng = 70.0;
                // let end_lng = 140.0;

                iter.for_each(|(((lat, lon), ir1), ir2)| {
                    if *lat != 32765 && *lon != 32765 {
                        let vlat = *lat as f32;
                        let vlon = *lon as f32;
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

                        // if vlat >= start_lat && vlat <= end_lat && vlon >= start_lng && vlon <= end_lng
                        // {
                        kdtree
                            .add([vlon, vlat], [*ir1 as f32, *ir2 as f32])
                            .unwrap();
                        // }
                    }
                });
                println!("kdtree created");
                dbg!(min_lat, max_lat, min_lon, max_lon);

                let start_lat = min_lat;
                let end_lat = max_lat;
                let start_lng = min_lon;
                let end_lng = max_lon;

                let step = 0.5;
                let rows = (end_lat - start_lat) / 0.5;
                let rows = rows as usize + 1;
                let cols = (end_lng - start_lng) / 0.5;
                let cols = cols as usize + 1;

                let mut grid_values = vec![vec![0.0f32; 2]; rows * cols];

                grid_values.par_iter_mut().enumerate().for_each(|(i, d)| {
                    let r = i / cols;
                    let c = i % cols;
                    let lat = start_lat + r as f32 * step;
                    let lon = start_lng + c as f32 * step;
                    let ret = kdtree.nearest(&[lon, lat], 4, &squared_euclidean).unwrap();
                    let mut nom = vec![0.0; 2];
                    let mut denom = vec![0.0; 2];
                    for (dist, value) in ret.iter() {
                        for i in 0..2 {
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
                    let mut v = vec![0.0; 2];
                    for i in 0..2 {
                        v[i] = nom[i] / denom[i];
                    }
                    *d = v;
                });

                let mut last_values = vec![vec![0f32; rows * cols]; 2];
                last_values.par_iter_mut().enumerate().for_each(|(i, d)| {
                    *d = grid_values.iter().map(|gv| gv[i]).collect::<Vec<_>>();
                });

                let elements = vec!["JB_JC", "JB_YJ"];

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
                        level: Some((count + 1) as f32),
                        element: String::from(*e),
                        center: String::new(),
                        product: format!("{}/{}", product_name, sat_name),
                        station: None,
                        values: last_values[i].clone(),
                        data_date: format!("{}", start_date),
                        data_time: format!("{}00", start_time),
                        forecast_time: 0,
                    };
                    sgrids.push(sgrid);
                });

                // dbg!(out1,out2);
                count += 1;
            });
            // dbg!(count);
            Some(sgrids)
        } else {
            println!("error file");
            None
        }
    }
}
