use async_trait::async_trait;
use serde::{Deserialize, Serialize}; // Import serde traits for serialization/deserialization

#[async_trait]
pub trait EarthquakeDataSource {
    type Error;

    async fn fetch_earthquake_data(
        &self,
        format: &str,
        start_time: &str,
        end_time: &str,
        min_magnitude: &str,
    ) -> Result<Vec<EarthquakeEvent>, Self::Error>;
}

#[async_trait]
impl EarthquakeDataSource for UsgsDataSource {
    type Error = Errors;

    async fn fetch_earthquake_data(
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
        let response = reqwest::get(&url).await?; // todo: async features: #[cfg(feature = "async")]

        // Check if the response was successful
        match response.status() {
            reqwest::StatusCode::OK => {
                // Parse the GeoJSON response into EarthquakeEvent objects
                let earthquake_data: GeoJsonData = response.json().await?;
                let earthquake_events: Vec<EarthquakeEvent> = earthquake_data
                    .features
                    .into_iter()
                    .map(|feature: Feature| EarthquakeEvent {
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EarthquakeEvent {
    pub mag: f64,
    pub place: Option<String>,
    pub time: i64,
    pub updated: i64,
    pub tsunami: i32,
    pub coordinates: Coordinates<f64>,
    pub mag_type: String,
    pub event_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinates<T> {
    pub lat: T,
    pub lon: T,
    pub depth: T,
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
#[serde(rename_all = "camelCase")]
pub struct Properties {
    mag: f64,
    place: Option<String>,
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
    mag_type: String,
    #[serde(alias = "type")]
    event_type: String,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(alias = "type")]
    geometry_type: String,
    coordinates: Coordinates<f64>,
}
