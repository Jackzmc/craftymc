use chrono::{DateTime, Utc};

pub fn get_unix_timestamp_now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64
}

pub fn get_iso8601(mut timestamp: Option<i64>) -> String {
    if timestamp.is_none() {
        timestamp = Some(get_unix_timestamp_now());
    }
    DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(timestamp.unwrap(), 0), Utc).to_rfc3339()
}
