use chrono::{DateTime, Datelike, Duration, Local, NaiveDateTime, TimeZone, Timelike, Utc};
use std::str::FromStr;
use unitools_core::error::ToolError;

/// 获取当前本地时间
pub fn current_local_time() -> DateTime<Local> {
    Local::now()
}

/// 获取当前UTC时间
pub fn current_utc_time() -> DateTime<Utc> {
    Utc::now()
}

/// 格式化DateTime为字符串
pub fn format_datetime<Tz: TimeZone>(dt: &DateTime<Tz>, fmt: &str) -> String
where
    Tz::Offset: std::fmt::Display,
{
    dt.format(fmt).to_string()
}

/// 解析字符串为DateTime
pub fn parse_datetime(s: &str, fmt: &str) -> Result<DateTime<Utc>, ToolError> {
    let naive = NaiveDateTime::parse_from_str(s, fmt)
        .map_err(|e| ToolError::ParseError(format!("解析日期时间错误: {}", e)))?;

    let dt = DateTime::<Utc>::from_utc(naive, Utc);
    Ok(dt)
}

/// 解析ISO 8601格式的时间字符串
pub fn parse_iso8601(s: &str) -> Result<DateTime<Utc>, ToolError> {
    DateTime::parse_from_str(s, "%+")
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|_| DateTime::from_str(s).map(|dt| dt))
        .map_err(|e| ToolError::ParseError(format!("解析ISO8601格式错误: {}", e)))
}

/// 转换时间戳（秒）为DateTime<Utc>
pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap_or_default();
    DateTime::<Utc>::from_utc(naive, Utc)
}

/// 转换DateTime<Utc>为时间戳（秒）
pub fn datetime_to_timestamp<Tz: TimeZone>(dt: &DateTime<Tz>) -> i64
where
    Tz::Offset: std::fmt::Display,
{
    dt.timestamp()
}

/// 计算两个日期之间的天数差
pub fn days_between<Tz1, Tz2>(dt1: &DateTime<Tz1>, dt2: &DateTime<Tz2>) -> i64
where
    Tz1: TimeZone,
    Tz2: TimeZone,
    Tz1::Offset: std::fmt::Display,
    Tz2::Offset: std::fmt::Display,
{
    let utc1 = dt1.with_timezone(&Utc);
    let utc2 = dt2.with_timezone(&Utc);

    let duration = if utc1 > utc2 {
        utc1 - utc2
    } else {
        utc2 - utc1
    };

    duration.num_days()
}

/// 增加/减少天数
pub fn add_days<Tz: TimeZone>(dt: &DateTime<Tz>, days: i64) -> DateTime<Tz>
where
    Tz::Offset: std::fmt::Display,
{
    *dt + Duration::days(days)
}

/// 增加/减少小时
pub fn add_hours<Tz: TimeZone>(dt: &DateTime<Tz>, hours: i64) -> DateTime<Tz>
where
    Tz::Offset: std::fmt::Display,
{
    *dt + Duration::hours(hours)
}

/// 获取日期的年、月、日
pub fn get_date_components<Tz: TimeZone>(dt: &DateTime<Tz>) -> (i32, u32, u32)
where
    Tz::Offset: std::fmt::Display,
{
    (dt.year(), dt.month(), dt.day())
}

/// 获取时间的时、分、秒
pub fn get_time_components<Tz: TimeZone>(dt: &DateTime<Tz>) -> (u32, u32, u32)
where
    Tz::Offset: std::fmt::Display,
{
    (dt.hour(), dt.minute(), dt.second())
}
