use crate::{MetError, RadarData, STRadialData, ToGrids};
use contour::*;
use geojson::Value;
use plotters::prelude::*;
use std::fs::File;
use std::io::Read;
mod cc_reader;
mod sab_reader;
mod sc_reader;
mod wsr98d_reader;

use common_data::SingleGrid;
use sab_reader::SABReader;
use sc_reader::SCReader;
use wsr98d_reader::WSR98DReader;

pub enum CinRadarReader {
    WSR98D(STRadialData),
    // SAB(SABReader),
    // SC(SCReader),
}

impl CinRadarReader {
    pub fn new(fname: &str) -> Result<Self, MetError> {
        let mut buf = Vec::new();
        let mut f = File::open(fname)?;
        f.read_to_end(&mut buf)?;
        // dbg!(buf.len() % 3132);
        let flag = &buf[0..28];
        let flag1 = &flag[0..4];
        //标准格式
        if flag1 == b"RSTM" {
            println!("WSR98D");
            let reader = WSR98DReader::new(&buf).unwrap();
            // let r = reader.el2idx(0.48339844);
            // println!("{:?}",r);
            // let r = reader.el2idx(19.511719);
            // println!("{:?}",r);
            // let r = reader.get_nearest_4v("dBT", 0.48339844, 0.1, 1000.0); //11, 19.511719

            //lon 114.243904 lat 24.871916   lon0 118.7025 lat0 29.085278
            let r = reader.ppi_to_grid_lonlat(19.511719, "dBT").unwrap();
            crate::grid2diamond4(&r, "/mnt/d/temp/demo5");

            // let r = reader.get_vcs_data("dBT",&(118.7025,29.085278),&(114.243904,24.871916)).unwrap();
            // let r = reader
            //     .get_vcs_data("dBZ", &(118.7025,29.085278), &(119.209, 29.141))
            //     .unwrap();
            // let r = reader
            //     .get_vcs_data("dBZ", &(118.7025, 29.085278), &(118.763, 29.116))
            //     .unwrap();
            let r = reader
                .get_vcs_data("dBZ", &(118.491, 29.203), &(118.655, 29.163))
                .unwrap();
            // println!("{:?} {:?}, {:#?}",r.len(),r[0].len(),r);
            // println!("{:#?} ",r);
            let x_dim = r[0].len();
            let y_dim = r.len();
            let data = r
                .into_iter()
                .flatten()
                // .map(|d| if d == crate::MISSING { -100.0 } else { d })
                .collect::<Vec<f32>>();
            // let data = data.iter().map(|d| if *d==crate::MISSING {f32::NAN}else {*d});

            // let mut dat = Vec::new();

            // for (yidx,y) in r.iter().enumerate() {
            //     for (xidx,x) in y.iter().enumerate() {
            //         if *x != crate::MISSING {
            //             // println!("xidx {} yidx {}",xidx,yidx);
            //             dat.push((xidx,yidx));
            //         }
            //     }
            // }

            // println!("{:?}",dat);

            // let root_area = SVGBackend::new("vcs.svg", (600, 400))
            // .into_drawing_area();
            // root_area.fill(&BLACK).unwrap();

            // let mut ctx = ChartBuilder::on(&root_area)
            //     // .set_label_area_size(LabelAreaPosition::Left, 40)
            //     // .set_label_area_size(LabelAreaPosition::Bottom, 40)
            //     // .caption("Demo", ("sans-serif", 40))
            //     .build_cartesian_2d(0..x_dim, 0..y_dim)
            //     .unwrap();

            // ctx.configure_mesh().draw().unwrap();

            // ctx.draw_series(dat.iter().map(|point| Circle::new(*point, 1, &RED)))
            // .unwrap();

            let c = ContourBuilder::new(x_dim as u32, y_dim as u32, true);
            let data = data.iter().map(|d| *d as f64).collect::<Vec<f64>>();
            let res = c.contours(data.as_slice(), &[10.0]).unwrap();
            // let res = c.contours(data.as_slice(), &[10.0]);
            // println!("{:#?}",res);

            // // println!("{:?}",line_verts);
            let root_area = BitMapBackend::new("vcs.png", (600, 400)).into_drawing_area();
            root_area.fill(&BLACK).unwrap();

            let mut ctx = ChartBuilder::on(&root_area)
                // .set_label_area_size(LabelAreaPosition::Left, 40)
                // .set_label_area_size(LabelAreaPosition::Bottom, 40)
                // .caption("Demo", ("sans-serif", 40))
                // .build_cartesian_2d(0..x_dim, 0..y_dim)
                .build_cartesian_2d(0.0..66.0, 0.0..8.0)
                .unwrap();
            ctx.configure_mesh().draw().unwrap();

            for f in res.iter() {
                if let Some(g) = &f.geometry {
                    if let &Value::MultiPolygon(ref mpoly) = &g.value {
                        for poly in mpoly.iter() {
                            for (i, line) in poly.iter().enumerate() {
                                let mut line_verts = Vec::new();
                                for ps in line.iter() {
                                    line_verts.push((ps[0], ps[1]));
                                }
                                println!("{} ", i);
                                println!("{:?}", line_verts);
                                if i == 0 {
                                    ctx.draw_series(std::iter::once(PathElement::new(
                                        line_verts, &RED,
                                    )))
                                    .unwrap();
                                } else {
                                    ctx.draw_series(std::iter::once(PathElement::new(
                                        line_verts, &BLUE,
                                    )))
                                    .unwrap();
                                }
                            }
                        }
                    }
                }

                if let Some(props) = &f.properties {
                    let value = props.get("value");
                    // println!("{:?}",value);
                    if let Some(n) = value {
                        println!("{:?}", n.as_f64().unwrap());
                    }
                }
            }

            // ctx.draw_series(dat.iter().map(|point| Circle::new(*point, 1, &RED)))
            // .unwrap();

            // let r = reader.get_vol_data("dBZ").unwrap();
            // println!("{:#?}", r);
            // crate::grid2diamond4(&r, "/mnt/d/temp/demo4");
            // println!("r {:?}", r);
            // let r = reader.get_elevate_element();
            // println!("r {:?}", r);
            return Ok(Self::WSR98D(reader));
        } else {
            // if &flag[14..16] == b"\x01\x00" {
            //     println!("SAB");
            //     let reader = SABReader::new(&buf)?;
            //     return Ok(Self::SAB(reader));
            // }

            // // dbg!(flag1);

            // let sc_flag = &buf[100..109];
            // if sc_flag == b"CINRAD/SC" || sc_flag == b"CINRAD/CD" {
            //     println!("SC");
            //     let reader = SCReader::new(&buf)?;
            //     return Ok(Self::SC(reader));
            // }
            // dbg!(sc_flag);

            // let cc_flag = &buf[116..125];
            // if cc_flag == b"CINRAD/CC" {
            //     println!("CC")
            // }
        }
        // dbg!(cc_flag);

        // Ok(CinRadarReader)
        Err(MetError::UnknowCinRadError)
    }
}

impl ToGrids for CinRadarReader {
    fn to_grids(&self) -> Option<Vec<SingleGrid>> {
        match self {
            Self::WSR98D(std) => std.to_grids(),
            _ => None,
        }
    }
}
