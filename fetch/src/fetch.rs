use common::earthquake_event::*;

// Function to fetch earthquake data from a specific source
fn fetch_earthquake_data_from_source(
    source: &dyn EarthquakeDataSource<Error = Errors>,
    format: &str,
    start_time: &str,
    end_time: &str,
    min_magnitude: &str,
) -> Result<Vec<EarthquakeEvent>, Errors> {
    source.fetch_earthquake_data(format, start_time, end_time, min_magnitude)
}

// Example usage
pub fn run_fetch(
    start_time: &str,
    end_time: &str,
    min_magnitude: i32,
) -> Result<Vec<EarthquakeEvent>, Errors> {
    let usgs_data_source = UsgsDataSource;
    let format = "geojson";

    let usgs_earthquake_data = fetch_earthquake_data_from_source(
        &usgs_data_source,
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
