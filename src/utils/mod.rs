use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use anyhow::Result;

/// 将时间戳转换为 UTC DateTime
pub fn timestamp_to_datetime(timestamp: i64) -> Result<DateTime<Utc>> {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .ok_or_else(|| anyhow::anyhow!("Invalid timestamp"))?;
    Ok(Utc.from_utc_datetime(&naive))
}

/// 将 UTC DateTime 转换为时间戳
pub fn datetime_to_timestamp(dt: DateTime<Utc>) -> i64 {
    dt.timestamp()
}

/// 解析时间间隔字符串
pub fn parse_interval(interval: &str) -> Result<chrono::Duration> {
    let (amount, unit) = interval.split_at(interval.len() - 1);
    let amount: i64 = amount.parse()?;
    
    match unit {
        "m" => Ok(chrono::Duration::minutes(amount)),
        "h" => Ok(chrono::Duration::hours(amount)),
        "d" => Ok(chrono::Duration::days(amount)),
        _ => Err(anyhow::anyhow!("Invalid interval format"))
    }
}

/// 将 f64 转换为指定精度的字符串
pub fn format_float(value: f64, precision: usize) -> String {
    format!("{:.1$}", value, precision)
}