extern crate chrono;

use std::ops::Add;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

fn main() {
    // 解析日期时间字符串
    let datetime_str = "2023-09-25 15:25:02.963";
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S%.3f")
        .unwrap()
        .and_utc() - chrono::Duration::hours(8);

    // 转换为时间戳
    let timestamp = datetime.timestamp_millis();

    println!("时间戳: {}", timestamp);
}
