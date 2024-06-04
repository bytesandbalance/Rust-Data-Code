use crate::clustering::EarthquakeCluster;
use chrono::{Duration, NaiveDateTime};
use std::error::Error;
use std::fmt;

// Function to calculate time since the last earthquake with magnitude greater than 5 for an individual cluster
pub async fn calculate_time_since_last_significant_earthquake(
    cluster: &EarthquakeCluster,
) -> Option<Duration> {
    // Find the most recent significant earthquake for the cluster
    let mut most_recent_timestamp: Option<NaiveDateTime> = None;

    for earthquake in &cluster.events {
        let timestamp_secs = earthquake.time / 1000;
        let earthquake_datetime = NaiveDateTime::from_timestamp_opt(timestamp_secs, 0);

        if earthquake.mag > 5.0 {
            if most_recent_timestamp.is_none() || earthquake_datetime > most_recent_timestamp {
                most_recent_timestamp = earthquake_datetime;
            }
        }
    }

    // Calculate time since the last significant earthquake for the cluster
    if let Some(timestamp) = most_recent_timestamp {
        let current_time = chrono::Utc::now().naive_utc();
        Some(current_time - timestamp)
    } else {
        None // No significant earthquake found
    }
}

// Define a struct to hold statistics for a single metric (depth, magnitude, or energy)
#[derive(Clone, Debug)]
pub struct MetricStatistics {
    pub min: f64,
    pub max: f64,
    pub avg: f64
}

// Function to calculate statistics for a single metric using DataFusion
async fn calculate_metric_statistics_async(metric_data: &Vec<f64>) -> MetricStatistics {
    // Filter out infinite values before calculating min and max
    let finite_data: Vec<f64> = metric_data.iter().cloned().filter(|&x| x.is_finite()).collect();

    // Calculate min and max for finite data
    let min = finite_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = finite_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // Calculate the average value
    let sum: f64 = finite_data.iter().sum();
    let avg = sum / finite_data.len() as f64;

    MetricStatistics {
        min,
        max,
        avg,
    }
}
// Function to calculate statistics for a cluster
pub async fn calculate_cluster_statistics_async(
    cluster: &EarthquakeCluster,
) -> (MetricStatistics, MetricStatistics) {
    // Extract depth and magnitude data from the cluster's EarthquakeEvent instances
    let depth_data: Vec<f64> = cluster
        .events
        .iter()
        .map(|event| event.coordinates.depth)
        .collect::<Vec<f64>>()
        .into();

    let magnitude_data: Vec<f64> = cluster
        .events
        .iter()
        .map(|event| event.mag)
        .collect::<Vec<f64>>()
        .into();

    let (depth_stats, magnitude_stats) = tokio::join!(
        calculate_metric_statistics_async(&depth_data),
        calculate_metric_statistics_async(&magnitude_data),
    );

    (depth_stats, magnitude_stats)
}


pub async fn calculate_all_cluster_statistics_async(
    clusters: Vec<EarthquakeCluster>,
) -> Result<Vec<ClusterStatistics>, Box<dyn Error>> {
    let tasks = clusters.into_iter().map(|cluster| {
        let centroid = cluster.centroid.clone(); // Clone centroid
        let cluster_clone = cluster.clone(); // Clone cluster for async block

        async move {
            let duration_task = calculate_time_since_last_significant_earthquake(&cluster_clone);

            let (depth_stats, magnitude_stats) = calculate_cluster_statistics_async(&cluster_clone).await;
            let duration = duration_task.await;

            Ok(ClusterStatistics {
                centroid,
                depth_stats,
                magnitude_stats,
                duration,
            })
        }
    });

    // Collect and await all the async tasks
    let results: Result<Vec<_>, Box<dyn Error>> = futures::future::try_join_all(tasks).await;

    // Handle errors or extract successful results
    match results {
        Ok(statistics) => Ok(statistics),
        Err(err) => Err(err),
    }
}


// Define the ClusterStatistics struct to hold statistics for each cluster
pub struct ClusterStatistics {
    // pub cluster_id: usize,
    pub centroid: (f64, f64),
    pub depth_stats: MetricStatistics,
    pub magnitude_stats: MetricStatistics,
    pub duration: Option<Duration>, // Time since last significant earthquake
}

impl fmt::Display for ClusterStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cluster Statistics:\n\
            Centroid: {:?}\n\
            Depth Stats: {:?}\n\
            Magnitude Stats: {:?}\n\
            Duration since last significant earthquake: {:?}\n",
            self.centroid, self.depth_stats, self.magnitude_stats, self.duration
        )
    }
}
