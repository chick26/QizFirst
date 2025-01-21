use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kline {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub timestamp: DateTime<Utc>,
    pub price: f64,
    pub amount: f64,
    pub side: TradeSide,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TradeSide {
    Buy,
    Sell,
}

/// 市场数据获取接口
pub trait MarketDataProvider {
    async fn get_klines(&self, symbol: &str, interval: &str, limit: u32) -> anyhow::Result<Vec<Kline>>;
    async fn get_trades(&self, symbol: &str, limit: u32) -> anyhow::Result<Vec<Trade>>;
}