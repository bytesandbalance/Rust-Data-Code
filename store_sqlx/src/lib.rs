use chrono::prelude::*;
use sqlx::error::Error as SqlxError;
use sqlx::postgres::PgPool;
use sqlx::query;
pub mod models;
pub use self::models::EarthquakeEventModel;
use common::earthquake_event::EarthquakeEvent;

pub async fn insert_earthquake_events(
    pool: &PgPool,
    events: Vec<EarthquakeEventModel>,
) -> Result<(), SqlxError> {
    // Insert the earthquake events into the PostgreSQL table
    for event in &events {
        query!(
            "INSERT INTO earthquake_events (mag, place, time, updated, tsunami, lon, lat, mag_type, event_type) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            event.mag,
            &event.place,
            event.time,
            event.updated,
            event.tsunami,
            event.lon,
            event.lat,
            &event.mag_type,
            &event.event_type,
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub fn convert_to_model(events: Vec<EarthquakeEvent>) -> Vec<EarthquakeEventModel> {
    events
        .into_iter()
        .map(|event| {
            let time =
                NaiveDateTime::from_timestamp_opt(event.time / 1000, 0).expect("Invalid timestamp");
            let time_utc: DateTime<Utc> = DateTime::from_utc(time, Utc);
            println!("{:?}", time_utc);
            let updated = NaiveDateTime::from_timestamp_opt(event.updated / 1000, 0)
                .expect("Invalid timestamp");
            let updated_utc: DateTime<Utc> = DateTime::from_utc(updated, Utc);

            EarthquakeEventModel {
                mag: event.mag,
                place: event.place.clone(),
                tsunami: event.tsunami,
                time: Some(time_utc),
                updated: Some(updated_utc),
                lon: event.coordinates[0],
                lat: event.coordinates[1],
                mag_type: event.mag_type.clone(),
                event_type: event.event_type.clone(),
            }
        })
        .collect()
}
