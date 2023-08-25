use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::earthquake_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EarthquakeEventModel {
    pub mag: f64,
    pub place: String,
    pub time: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
    pub tsunami: i32,
    pub lon: f64,
    pub lat: f64,
    pub mag_type: String,
    pub event_type: String,
}
