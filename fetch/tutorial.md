// Introduction:

Kia ora everyone! Welcome to my Rust data engineering series. Today, We'll be leveraging Rust to access the USGS API and obtain seismic activity data.

We'll kick things off by setting up our project structure and getting familiar with its organization. Then, we'll dive into the `EarthquakeEvent` struct, which forms the backbone of our earthquake data representation. After that, we'll roll up our sleeves and implement the `EarthquakeDataSource` trait, which will serve as a common interface for fetching earthquake data from various sources.

Let's get started!

// Setting Up the Project:

Let's first organize our project structure. Our project is arranged into several folders, with the crux of our functionality  within the `common` folder.

Within the `common` folder resides a module named `blocking`. This module specializes in synchronous fetching of earthquake data, housing the essential functionality required to communicate with the USGS API and retrieve earthquake data.

Let's quickly glance at the directory structure:

common/
│
├── blocking/
│   ├── earthquake_event.rs
│   ├── fetch.rs
│   └── mod.rs
│
├── lib.rs
└── utils.rs

Within the `blocking` module:
- `earthquake_event.rs`: Defines the structure for earthquake events.
- `fetch.rs`: Implements synchronous fetching of earthquake data.
- `mod.rs`: Acts as a module file, ensuring efficient organization of our code.

Outside the `blocking` module:
- `lib.rs`: Serves as the entry point for our library, defining public APIs and module structures.
- `utils.rs`: Contains utility functions that can be shared across different modules.

With our project structure neatly laid out, we're all set to dive into coding our earthquake data fetching functionality.

// Introducing the EarthquakeEvent Struct:

Now, let's zoom in on the `EarthquakeEvent` struct, the cornerstone of our earthquake data representation.

```rust
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

The EarthquakeEvent struct encapsulates crucial information about each seismic event:

mag: Magnitude of the earthquake.
place: Location of the earthquake.
time: Timestamp of the earthquake.
updated: Timestamp of when the earthquake data was last updated.
tsunami: Indicator for whether the earthquake caused a tsunami.
coordinates: Geographic coordinates of the earthquake epicenter.
mag_type: Type of magnitude measurement.
event_type: Type of earthquake event.
We've utilized serde's Serialize and Deserialize traits to facilitate seamless conversion of our Rust structs to and from JSON format, enabling easy interaction with APIs like the one provided by USGS.

// Implementing the EarthquakeDataSource Trait:

Now, let's roll up our sleeves and implement the EarthquakeDataSource trait. This trait provides a common interface for fetching earthquake data, abstracting away the intricacies of data retrieval.

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

The EarthquakeDataSource trait defines a method fetch_earthquake_data responsible for fetching earthquake data. It accepts parameters such as format, start time, end time, and minimum magnitude, returning a Result containing a vector of EarthquakeEvent objects or an error.

Now, let's explore the implementation for the USGS data source:

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

        // Make the HTTP request to the USGS API
        let response = reqwest::blocking::get(&url)?;

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

In this implementation, we construct the API URL using provided parameters and

// Explanation of deserializing GeoJSON response and parsing into EarthquakeEvent objects:
Here, we deserialize the JSON response from the USGS API into a GeoJsonData object.
GeoJsonData  represents data in the GeoJSON format, commonly used for geographic data representation.
The `response.json()?` line attempts deserialization, using the `?` operator for error handling.
If deserialization succeeds, the code extracts earthquake event information by iterating over the features of the GeoJsonData object. For each feature, a new EarthquakeEvent object is created, initializing it with relevant properties such as magnitude, place, time, etc. This is achieved using the `map` function in conjunction with `into_iter()`, applying the mapping operation in a concise and efficient functional style. Finally, the mapped EarthquakeEvent objects are collected into a vector using the `collect` function, which is then returned as part of the `Ok` variant of the `Result`.

// Where UnexpectedStatusCode comes from:
`UnexpectedStatusCode` is an error variant defined in the `Errors` enum.

```Rust
#[derive(thiserror::Error, Debug)]
pub enum Errors {
    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(String),

    #[error("request error")]
    OtherError(#[from] reqwest::Error),
}
```
It signifies scenarios where the HTTP response status code from the USGS API is unexpected or indicates an error condition.
In the `match` block, if the HTTP response status is not `OK` (200), indicating a successful response, an `UnexpectedStatusCode` error is constructed. The status code is converted to a string and returned as part of the `Err` variant of the `Result`.


4. Fetching Earthquake Data:

Now that we have our fetching functionality implemented, let's explore how we can use it to obtain earthquake data.

Introducing the `run_fetch` Function:

In the `fetch.rs` file, we have the `run_fetch` function, which serves as the entry point for fetching earthquake data.

```rust
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

The run_fetch function accepts parameters such as start time, end time, and minimum magnitude, and returns a Result containing a vector of EarthquakeEvent objects or an error of type Errors.

Using the run_fetch Function:
Now, let's see how we can use the run_fetch function to fetch earthquake data.

In the main.rs file:

pub mod fetch;
use fetch::run_fetch;

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


Here, we import the run_fetch function and call it with the specified parameters. If the function succeeds, we print each earthquake event to the console.

Running the Application:
To run the application, execute the following command in the terminal:

cargo run

Conclusion:
We've covered the fundamentals of fetching earthquake data using Rust. We began by setting up our project structure, defining essential data structures, and implementing functionality to fetch earthquake data from the USGS API.

Throughout the process, we emphasized error handling, modular design, and the use of traits for abstraction, showcasing Rust's robustness and expressiveness for data engineering tasks.

As you continue your Rust journey, remember to explore additional features, such as asynchronous fetching, error recovery strategies, and integrating with external libraries for data analysis and visualization.

Thank you for joining us on this Rust data engineering adventure! See you in the next tutorial.
