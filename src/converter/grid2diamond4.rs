
use crate::data_type::SingleGrid;
use std::io::*;
use std::fs::File;

pub fn grid2diamond4(grid:&SingleGrid,output:&str) {
    let  file = File::create(output).unwrap();
    let mut buf = BufWriter::new(file);
    writeln!(
        buf,
        "diamond 4 radardemo").unwrap();

    //20200704_164546
    writeln!(
        buf,
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} ",
        2020,
        07,
        04,
        16,
        45,
        0,
        grid.lng_gap,
        grid.lat_gap,
        grid.start_lng,
        grid.end_lng,
        grid.start_lat,
        grid.end_lat,
        grid.ni,
        grid.nj
    ).unwrap();
    writeln!(buf, "{:.2} {:.2} {:.2} {} {} ", 5.0, -5, 75, 0, 0).unwrap();
    for (i,v) in grid.values.iter().enumerate() {                  
        write!(buf, "{:.*} ", 2, v).unwrap();
        if i.rem_euclid(10usize) == 0 {
            writeln!(buf).unwrap();
        }
       
    }  
    buf.flush();
}