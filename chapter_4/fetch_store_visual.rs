use chrono::Utc;
use std::time::Duration;
use store::insert_earthquake_events;
use tokio::time;
// use visual::visualize_earthquake_events;

// Other imports and dependencies

#[tokio::main]
async fn main() {
    let usgs_data_source = UsgsDataSource; // Instantiate your data source
    let format = "geojson";
    let polling_interval_secs = 60;

    loop {
        let current_time = Utc::now();
        let end_time = format_time(&current_time);
        let start_time =
            format_time(&(current_time - chrono::Duration::seconds(polling_interval_secs as i64)));
        let min_magnitude = "3";

        let earthquake_events = usgs_data_source
            .fetch_earthquake_data(format, &start_time, &end_time, min_magnitude)
            .await;

        match earthquake_events {
            Ok(events) => {
                // Create a database connection
                let connection = "host=localhost user=postgres dbname=USGS";

                // Asynchronously store earthquake events in the database
                tokio::spawn(async move {
                    insert_earthquake_events(&connection, &events).await;
                });

                // Asynchronously visualize earthquake events on the map
                // tokio::spawn(async move {
                //     visualize_earthquake_events(&events).await;
                // });
            }
            Err(e) => eprintln!("{e:?}"),
        }

        time::delay_for(Duration::from_secs(polling_interval_secs)).await;
    }
}
