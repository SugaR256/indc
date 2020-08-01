use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StockProfile {
    pub symbol: String,
    pub price: f32,
    pub changes_percentage: f32,
    pub change: f32,
    pub day_low: f32,
    pub day_high: f32,
    pub year_high: f32,
    pub year_low: f32,
    pub market_cap: f32,
    pub price_avg50: f32,
    pub price_avg200: f32,
    pub volume: i32,
    pub avg_volume: i128,
    pub exchange: String,
}