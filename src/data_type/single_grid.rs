pub struct SingleGrid {
    pub ni: i64,
    pub nj: i64,
    pub lat_gap: f64,
    pub lng_gap: f64,
    pub start_lat: f64,
    pub start_lng: f64,
    pub end_lat: f64,
    pub end_lng: f64,
    pub level: i64,
    pub element: String,
    pub values: Vec<f32>,
    // pub data_date: String,
    // pub data_time: i64,     //时次
    // pub forecast_time: i64, //时效
    // pub center: String,
}
