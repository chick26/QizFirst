use pyo3::prelude::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use polars::prelude::*;

// 定义模块
mod data;
mod storage;
mod utils;

#[pyclass]
struct MarketAnalyzer {
    storage: storage::MemoryStorage,
}

#[pymethods]
impl MarketAnalyzer {
    #[new]
    fn new() -> Self {
        Self {
            storage: storage::MemoryStorage::new(),
        }
    }

    fn save_klines(&self, symbol: String, klines_data: Vec<(i64, f64, f64, f64, f64, f64)>) -> PyResult<()> {
        let klines = klines_data.into_iter().map(|(ts, open, high, low, close, volume)| {
            data::Kline {
                timestamp: utils::timestamp_to_datetime(ts).unwrap(),
                open,
                high,
                low,
                close,
                volume,
            }
        }).collect();

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.storage.save_klines(&symbol, klines))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn query_klines(&self, symbol: String, start_ts: i64, end_ts: i64) -> PyResult<Vec<(i64, f64, f64, f64, f64, f64)>> {
        let start_time = utils::timestamp_to_datetime(start_ts).unwrap();
        let end_time = utils::timestamp_to_datetime(end_ts).unwrap();

        let klines = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.storage.query_klines(&symbol, start_time, end_time))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(klines.into_iter().map(|k| (
            utils::datetime_to_timestamp(k.timestamp),
            k.open,
            k.high,
            k.low,
            k.close,
            k.volume
        )).collect())
    }

    fn calculate_sma(&self, symbol: String, period: usize, start_ts: i64, end_ts: i64) -> PyResult<Vec<(i64, f64)>> {
        let klines = self.query_klines(symbol, start_ts, end_ts)?;
        
        if klines.is_empty() {
            return Ok(vec![]);
        }

        let closes: Vec<f64> = klines.iter().map(|k| k.4).collect();
        let timestamps: Vec<i64> = klines.iter().map(|k| k.0).collect();
        
        let series = Series::new("close", closes);
        let sma = series.rolling_mean(period).unwrap();
        
        Ok(timestamps.into_iter().zip(sma.f64().unwrap().into_iter())
            .filter_map(|(ts, maybe_val)| maybe_val.map(|v| (ts, v)))
            .collect())
    }
}

/// Python 模块初始化函数
#[pymodule]
fn qiz_first(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MarketAnalyzer>()?;
    Ok(())
}

/// 初始化函数
pub fn initialize() -> Result<()> {
    Ok(())
}