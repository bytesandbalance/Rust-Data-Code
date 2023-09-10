pub mod models;
pub mod schema;

use self::models::EarthquakeEventModel;
use chrono::NaiveDateTime;
use common::blocking::earthquake_event::EarthquakeEvent;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_earthquake_events(
    conn: &mut PgConnection,
    events: Vec<EarthquakeEventModel>,
) -> Result<(), diesel::result::Error> {
    use schema::earthquake_events;

    diesel::insert_into(earthquake_events::table)
        .values(events)
        .execute(conn)?;

    Ok(())
}

pub fn convert_to_model(events: Vec<EarthquakeEvent>) -> Vec<EarthquakeEventModel> {
    events
        .into_iter()
        .map(|event| {
            let time = NaiveDateTime::from_timestamp_opt(event.time / 1000, 0);
            let updated = NaiveDateTime::from_timestamp_opt(event.updated / 1000, 0);

            EarthquakeEventModel {
                mag: event.mag,
                place: event.place.clone(),
                time: time,
                updated: updated,
                tsunami: event.tsunami,
                lon: event.coordinates[0],
                lat: event.coordinates[1],
                mag_type: event.mag_type.clone(),
                event_type: event.event_type.clone(),
            }
        })
        .collect()
}
