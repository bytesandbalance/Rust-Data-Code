User
I am making a video about basics in Rust. I have my code that calls USGS API and use serde to fetch earthquake data between start_time and end_time. What I want to do is typing the code and talk over it but we need to design the tutorial together which part should start with and create a script for the tutorial. I am going to copz different part of the code first then we will design and write the script for the video.This is a tutorial for Rust-data engineering. How does it sound, arer you ready to see the code?

The structure of the ode
common/src
    blocking
        - earthequake_events.rs
        - fetch.rs
        - mod.rs
    lib.rs
    utils.rs

fetch/src
    - fetch.rs
    - main.rs


Now each script one by one:

common/src
    blocking
        - earthequake_events.rs
```Rust
// Define a trait for earthquake data sources
use serde::{Deserialize, Serialize}; // Import serde traits for serialization/deserialization

pub trait EarthquakeDataSource {
    type Error;

    fn fetch_earthquake_data(
        &self,
        format: &str,
        start_time: &str,
        end_time: &str,
        min_magnitude: &str,
    ) -> Result<Vec<EarthquakeEvent>, Self::Error>;
}

impl EarthquakeDataSource for UsgsDataSource {
    type Error = Errors;

    fn fetch_earthquake_data(
        &self,
        format: &str,
        start_time: &str,
        end_time: &str,
        min_magnitude: &str,
    ) -> Result<Vec<EarthquakeEvent>, Errors> {
        // Construct the URL for the USGS API with the provided parameters
        let base_url = "https://earthquake.usgs.gov/fdsnws/event/1/query";
        let url = format!(
            "{}?format={}&starttime={}&endtime={}&minmagnitude={}",
            base_url, format, start_time, end_time, min_magnitude
        );
        println!("{:?}", url);

        // Make the HTTP request to the USGS API
        let response = reqwest::blocking::get(&url)?; // todo: async features: #[cfg(feature = "async")]

        // Check if the response was successful
        match response.status() {
            reqwest::StatusCode::OK => {
                // Parse the GeoJSON response into EarthquakeEvent objects
                let earthquake_data: GeoJsonData = response.json()?;
                let earthquake_events: Vec<EarthquakeEvent> = earthquake_data
                    .features
                    .into_iter()
                    .map(|feature| EarthquakeEvent {
                        mag: feature.properties.mag,
                        place: feature.properties.place,
                        time: feature.properties.time,
                        updated: feature.properties.updated,
                        tsunami: feature.properties.tsunami,
                        coordinates: feature.geometry.coordinates,
                        mag_type: feature.properties.mag_type,
                        event_type: feature.properties.event_type,
                    })
                    .collect();
                Ok(earthquake_events)
            }
            status => Err(Errors::UnexpectedStatusCode(status.to_string())),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Errors {
    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(String),

    #[error("request error")]
    OtherError(#[from] reqwest::Error),
}

// Implement the trait for the USGS data source
pub struct UsgsDataSource;

// Data structure to hold earthquake event information
#[derive(Debug, Serialize, Deserialize)]
pub struct EarthquakeEvent {
    pub mag: f64,
    pub place: String,
    pub time: i64,
    pub updated: i64,
    pub tsunami: i32,
    pub coordinates: Vec<f64>,
    pub mag_type: String,
    pub event_type: String,
}

// GeoJSON data structure to deserialize the response
#[derive(Debug, Serialize, Deserialize)]
pub struct GeoJsonData {
    features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    properties: Properties,
    geometry: Geometry, // Add geometry field
    id: String,
}

// Define the Properties struct with all the fields
#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    mag: f64,
    place: String,
    time: i64,
    updated: i64,
    tz: Option<String>,
    url: String,
    detail: String,
    felt: Option<i32>,
    cdi: Option<f64>,
    mmi: Option<f64>,
    alert: Option<String>,
    status: String,
    tsunami: i32,
    sig: i32,
    net: String,
    code: String,
    ids: String,
    sources: String,
    types: String,
    nst: Option<i32>,
    dmin: Option<f64>,
    rms: Option<f64>,
    gap: Option<f64>,
    #[serde(rename = "magType")]
    mag_type: String,
    #[serde(rename = "type")]
    event_type: String,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    type_: String,
    coordinates: Vec<f64>,
}
```
common/src
    blocking
        - fetch.rs

```Rust
use super::earthquake_event::*;

pub fn run_fetch(
    start_time: &str,
    end_time: &str,
    min_magnitude: i32,
) -> Result<Vec<EarthquakeEvent>, Errors> {
    let usgs_data_source = UsgsDataSource;
    let format = "geojson";

    let usgs_earthquake_data = usgs_data_source.fetch_earthquake_data(
        format,
        start_time,
        end_time,
        &min_magnitude.to_string(),
    );

    match usgs_earthquake_data {
        Ok(earthquake_events) => Ok(earthquake_events),
        Err(e) => Err(e),
    }
}

```

common/src
    blocking
        - mod.rs

```Rust
pub mod earthquake_event;
pub mod fetch;
```

common/src
    - lib.rs

```Rust
#[cfg(feature = "blocking")]
pub mod blocking;
pub mod earthquake_event;
pub mod fetch;
pub mod utils;

```

common/src
    - utils.rs

use chrono::Utc;

pub fn format_time(time: &chrono::DateTime<Utc>) -> String {
    time.format("%Y-%m-%dT%H:%M:%S").to_string()
}


fetch/src
    - fetch.rs

```Rust
use common::blocking::earthquake_event::*;

pub fn run_fetch(
    start_time: &str,
    end_time: &str,
    min_magnitude: i32,
) -> Result<Vec<EarthquakeEvent>, Errors> {
    let usgs_data_source = UsgsDataSource;
    let format = "geojson";

    let usgs_earthquake_data: Result<Vec<EarthquakeEvent>, Errors> = usgs_data_source.fetch_earthquake_data(
        format,
        start_time,
        end_time,
        &min_magnitude.to_string(),
    );

    match usgs_earthquake_data {
        Ok(earthquake_events) => Ok(earthquake_events),
        Err(e) => Err(e),
    }
}

```

- fetch/src
    - main.rs

pub mod fetch;
use fetch::run_fetch; // Import the function

fn main() -> anyhow::Result<()> {
    let start_time = "2014-01-01";
    let end_time = "2014-02-01";
    let min_magnitude = 3;

    let earthquake_events = run_fetch(start_time, end_time, min_magnitude)?;
    for event in earthquake_events {
        println!("{:?}", event);
    }

    Ok(())
}



Now we want to design the tutorial for fetching but bear in mind this will be a series of tutorials and common forlder wil be used by other chapters too.


