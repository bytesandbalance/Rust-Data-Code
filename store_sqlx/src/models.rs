use chrono::{DateTime, Utc};

pub struct EarthquakeEventModel {
    pub mag: f64,
    pub place: String,
    pub time: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub tsunami: i32,
    pub lon: f64,
    pub lat: f64,
    pub mag_type: String,
    pub event_type: String,
}
