use tracing::instrument;

use super::earthquake_event::*;

#[instrument]
pub async fn run_fetch(
    start_time: &str,
    end_time: &str,
    min_magnitude: i32,
) -> Result<Vec<EarthquakeEvent>, Errors> {
    let usgs_data_source = UsgsDataSource;
    let format = "geojson";

    usgs_data_source
        .fetch_earthquake_data(format, start_time, end_time, &min_magnitude.to_string())
        .await
}
