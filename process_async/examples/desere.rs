use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Deserialize;
use tokio::task::spawn_blocking;

// Data structure to hold earthquake event information
// Make these the names you want to use in code.
// Then use serde field macros to map them to the names if they can't automatically (eg: snake case or camel case)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarthquakeEvent {
    #[serde(alias = "mag")]
    pub magnitude: f64,
    pub place: String,
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub updated: DateTime<Utc>,
    pub tsunami: i32,
    pub mag_type: String,
    // #[serde(alias = "type")]
    // pub event_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates<T> {
    pub lat: T,
    pub lon: T,
    pub depth: T,
}

// GeoJSON data structure to deserialize the response
#[derive(Debug, Deserialize)]
pub struct UsgsEarthquakeEvent {
    features: Vec<Feature>,
}

#[derive(Debug, Deserialize)]
pub struct Feature {
    // #[serde(flatten)]
    properties: EarthquakeEvent,
    geometry: Geometry, // Add geometry field
    id: String,
}

#[derive(Debug, Deserialize)]
pub struct Geometry {
    // type: String,
    // #[serde(flatten)]
    coordinates: Coordinates<f64>,
}

#[derive(thiserror::Error, Debug)]
pub enum Errors {
    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(String),

    #[error("request error")]
    OtherError(#[from] reqwest::Error),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("working");
    let format = "geojson";
    let start_time = "2014-01-01";
    let end_time = "2014-02-01";
    let min_magnitude = 3;

    let base_url = "https://earthquake.usgs.gov/fdsnws/event/1/query";
    let url = format!(
        "{}?format={}&starttime={}&endtime={}&minmagnitude={}",
        base_url, format, start_time, end_time, min_magnitude
    );
    println!("url: {url:?}");

    // // Make the HTTP request to the USGS API
    // let response = reqwest::get(&url).await?; // todo: async features: #[cfg(feature = "async")]

    // // Check if the response was successful
    // let events = match response.status() {
    //     reqwest::StatusCode::OK => {
    //         // Parse the GeoJSON response into EarthquakeEvent objects
    //         let earthquake_data: UsgsEarthquakeEvent = response.json().await?;
    //         let earthquake_events: Vec<EarthquakeEvent> = earthquake_data
    //             .features
    //             .into_iter()
    //             .map(|feature| feature.properties)
    //             .collect();
    //         Ok(earthquake_events)
    //     }
    //     status => Err(Errors::UnexpectedStatusCode(status.to_string())),
    // }?;

    // just to save making repeated calls, just read in from a file
    // serde_json::from_reader is difficult in async, because tokio's fs::File doesn't impl the right thing
    let earthquake_data = spawn_blocking(move || {
        let file = std::fs::File::open("sample.json")?;
        let json: UsgsEarthquakeEvent = serde_json::from_reader(file)?;
        let result: Result<UsgsEarthquakeEvent, anyhow::Error> = Ok(json);
        result
    })
    .await??;

    // let earthquake_events: Vec<EarthquakeEvent> = earthquake_data
    //     .features
    //     .into_iter()
    //     .map(|feature| feature.properties)
    //     .collect();

    println!("{earthquake_data:#?}");

    Ok(())
}
