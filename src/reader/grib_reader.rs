use crate::{MetError, SingleGrid, ToGrids};
use eccodes_rs;
use std::path::Path;
pub struct GribReader(String);

impl GribReader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        Ok(Self(String::from(fname)))
    }
}

impl ToGrids for GribReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        let fname = &self.0;
        let mut reader = eccodes_rs::GribReader::new(fname);
        let path = Path::new(fname);
        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_str().unwrap();
        let product;
        if file_name.starts_with("KT") {
            product = "T511";
        } else if file_name.starts_with("KW") {
            product = "WRF";
        } else {
            product = "GRIB";
        }
        let mut grids = Vec::new();
        reader
            .read_element_data_by("", |data, _| {
                for d in data.iter() {
                    let ni = d.ni;
                    let nj = d.nj;
                    let lat_gap = d.lat_gap;
                    let lng_gap = d.lng_gap;
                    let start_lat = d.start_lat;
                    let end_lat = d.end_lat;
                    let start_lng = d.start_lng;
                    let end_lng = d.end_lng;
                    let data_date = d.data_date.clone();
                    let data_time = format!("{:02}0000", d.data_time);
                    let forecast_time = d.forecast_time;
                    let element = d.element.clone();
                    let level = d.level;
                    let values = d.values.iter().map(|v| *v as f32).collect::<Vec<f32>>();

                    let data_des = format!("{}{}{}{}", data_date, data_time, element, level);
                    let sgrid = SingleGrid {
                        ni,
                        nj,
                        lat_gap,
                        lng_gap,
                        start_lat,
                        start_lng,
                        end_lat,
                        end_lng,
                        level: Some(level as f32),
                        element,
                        values,
                        data_date,
                        data_time,     //时次
                        forecast_time, //时效
                        center: String::from(""),
                        product: String::from(product),
                        data_des,
                    };
                    grids.push(sgrid);
                }
                Ok(())
            })
            .unwrap();

        Some(grids)
    }
}
