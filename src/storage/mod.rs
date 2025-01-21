use crate::data::{Kline, Trade};
use anyhow::Result;
use chrono::{DateTime, Utc};

/// 数据存储接口
pub trait DataStorage {
    /// 存储K线数据
    async fn save_klines(&self, symbol: &str, klines: Vec<Kline>) -> Result<()>;
    
    /// 存储交易数据
    async fn save_trades(&self, symbol: &str, trades: Vec<Trade>) -> Result<()>;
    
    /// 查询K线数据
    async fn query_klines(
        &self,
        symbol: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>
    ) -> Result<Vec<Kline>>;
    
    /// 查询交易数据
    async fn query_trades(
        &self,
        symbol: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>
    ) -> Result<Vec<Trade>>;
}

/// 内存数据存储实现
pub struct MemoryStorage {
    klines: std::collections::HashMap<String, Vec<Kline>>,
    trades: std::collections::HashMap<String, Vec<Trade>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            klines: std::collections::HashMap::new(),
            trades: std::collections::HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl DataStorage for MemoryStorage {
    async fn save_klines(&self, symbol: &str, klines: Vec<Kline>) -> Result<()> {
        let mut klines_map = self.klines.write().unwrap();
        klines_map.entry(symbol.to_string())
            .or_insert_with(Vec::new)
            .extend(klines);
        Ok(())
    }
    
    async fn save_trades(&self, symbol: &str, trades: Vec<Trade>) -> Result<()> {
        let mut trades_map = self.trades.write().unwrap();
        trades_map.entry(symbol.to_string())
            .or_insert_with(Vec::new)
            .extend(trades);
        Ok(())
    }
    
    async fn query_klines(
        &self,
        symbol: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>
    ) -> Result<Vec<Kline>> {
        let klines_map = self.klines.read().unwrap();
        Ok(klines_map.get(symbol)
            .map(|klines| klines.iter()
                .filter(|k| k.timestamp >= start_time && k.timestamp <= end_time)
                .cloned()
                .collect())
            .unwrap_or_default())
    }
    
    async fn query_trades(
        &self,
        symbol: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>
    ) -> Result<Vec<Trade>> {
        let trades_map = self.trades.read().unwrap();
        Ok(trades_map.get(symbol)
            .map(|trades| trades.iter()
                .filter(|t| t.timestamp >= start_time && t.timestamp <= end_time)
                .cloned()
                .collect())
            .unwrap_or_default())
    }
}