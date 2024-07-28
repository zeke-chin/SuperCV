use chrono::{DateTime, Local, TimeZone, Utc};
use chrono::Offset;
use chrono::offset::FixedOffset;

// 获取当前时间戳
pub fn get_current_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn get_current_date_time() -> DateTime<FixedOffset> {
    let local_timezone: String = Local::now().offset().fix().to_string();
    let current_timestamp = get_current_timestamp();
    timestamp_to_datetime(current_timestamp, &local_timezone)
}

// 将时间戳转换为字符串
pub fn timestamp_to_datetime(timestamp: i64, timezone: &str) -> DateTime<FixedOffset> {
    let tz: FixedOffset = timezone.parse().expect("Invalid timezone format");
    match Utc.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(datetime) => {
            let datetime_with_timezone = datetime.with_timezone(&tz);
            datetime_with_timezone
        }
        chrono::LocalResult::Ambiguous(_, _) => {
            panic!("Ambiguous timestamp");
        }
        chrono::LocalResult::None => {
            panic!("Invalid timestamp");
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use super::*;

    #[test]
    fn test_get_current_timestamp() {
        let current_timestamp = get_current_timestamp();
        println!("当前时间戳: {}", current_timestamp);
        assert!(current_timestamp > 0);
    }

    #[test]
    fn test_timestamp_to_datetime() {
        let current_timestamp = get_current_timestamp();
        let local_timezone: String = Local::now().offset().fix().to_string();
        let datetime = timestamp_to_datetime(current_timestamp, &local_timezone);
        println!(
            "当前是{}年{}月{}日{}时{}分{}秒",
            datetime.year(),
            datetime.month(),
            datetime.day(),
            datetime.hour(),
            datetime.minute(),
            datetime.second()
        );
        assert!(current_timestamp > 0);
    }
}
