#[derive(Clone,Debug)]
pub struct SingleGrid<T = f32, L = f32> {
    pub ni: i64, //列数，lon的个数
    pub nj: i64,
    pub lat_gap: f64,
    pub lng_gap: f64,
    pub start_lat: f64,
    pub start_lng: f64,
    pub end_lat: f64,
    pub end_lng: f64,
    pub level: Option<L>,
    pub element: String,
    pub values: Vec<T>,
    pub data_date: String,
    pub data_time: String,  //时次
    pub forecast_time: i64, //时效
    pub center: String,
    pub product: String,
    pub station: Option<String>,
}
