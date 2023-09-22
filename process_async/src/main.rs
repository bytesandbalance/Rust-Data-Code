pub mod clustering;
pub mod statistics;
pub mod write;

use clustering::cluster_earthquake_events;
use common::earthquake_event::EarthquakeEvent;
use common::fetch::run_fetch;
use statistics::{calculate_all_cluster_statistics_async, ClusterStatistics};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Define the start date as ten years ago from today
    let end_date = chrono::Utc::now().naive_utc();
    let start_date = end_date - chrono::Duration::days(365 * 10);

    // Create a date iterator to fetch data monthly
    let mut current_date = start_date.clone();
    let mut all_earthquake_events: Vec<EarthquakeEvent> = Vec::new();

    while current_date <= end_date {
        let next_month = current_date + chrono::Duration::days(30); // Assuming 30 days in a month for simplicity
        let start_time = current_date.to_string();
        let end_time = next_month.to_string();
        let min_magnitude = 3; // Set your desired minimum magnitude here

        // Log the start and end times being fetched
        println!("Fetching data for Start: {} End: {}", start_time, end_time);

        // Fetch data asynchronously for the current month
        let earthquake_events = run_fetch(&start_time, &end_time, min_magnitude).await?; // because of ? here, what was a &'static str didn't line up with what is the Err variant type main's return type

        // Append the fetched earthquake events to the list
        all_earthquake_events.extend(earthquake_events);

        // Move to the next month
        current_date = next_month;
    }

    // Set the number of clusters for k-means clustering
    let k = 10; // Adjust as needed

    // Cluster the earthquake events
    let clusters = cluster_earthquake_events(all_earthquake_events, k)?;

    // Calculate statistics for each cluster asynchronously
    let cluster_statistics: Vec<ClusterStatistics> =
        calculate_all_cluster_statistics_async(clusters).await;

    write::write_cluster_statistics_to_parquet(cluster_statistics, "output.parquet")?;

    Ok(())
}
