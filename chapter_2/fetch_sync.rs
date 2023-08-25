use crate::earthquake_event::*;
use crate::utils::format_time;
use chrono::Utc;
use std::thread;
use std::time::Duration;

fn fetch_earthquake_data_real_time(
    source: &dyn EarthquakeDataSource<Error = Errors>,
    format: &str,
    polling_interval_secs: u64,
) {
    loop {
        // Calculate start and end times dynamically
        let current_time = Utc::now();
        let end_time = format_time(&current_time);
        let start_time =
            format_time(&(current_time - chrono::Duration::seconds(polling_interval_secs as i64)));

        let min_magnitude = "3";

        // Fetch earthquake data synchronously
        let usgs_earthquake_data =
            source.fetch_earthquake_data(format, &start_time, &end_time, min_magnitude);

        match usgs_earthquake_data {
            Ok(earthquake_events) => {
                for event in earthquake_events {
                    println!("{event:?}");
                }
            }
            Err(e) => eprintln!("{e:?}"),
        }

        // Pause for the polling interval before the next fetch
        thread::sleep(Duration::from_secs(polling_interval_secs));
    }
}

// Example usage
pub fn run_fetch_sync() {
    let usgs_data_source = UsgsDataSource;
    let format = "geojson";
    let polling_interval_secs = 60; // Fetch every 1 minute

    fetch_earthquake_data_real_time(&usgs_data_source, format, polling_interval_secs);
}
