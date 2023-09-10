use common::earthquake_event::*;

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
