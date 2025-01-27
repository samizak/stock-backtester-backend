use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Prices {
    pub ticker: String,
    pub prices: Vec<PriceData>,
}

#[derive(Serialize)]
pub struct PriceData {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub ticker: String,
}
