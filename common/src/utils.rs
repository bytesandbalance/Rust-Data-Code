use chrono::Utc;

pub fn format_time(time: &chrono::DateTime<Utc>) -> String {
    time.format("%Y-%m-%dT%H:%M:%S").to_string()
}
