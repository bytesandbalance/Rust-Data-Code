use crate::clustering::EarthquakeCluster;
use arrow::array::Float64Array;
use chrono::{Duration, NaiveDateTime};
use std::collections::HashMap;
use tokio::task;

// Function to calculate time since the last earthquake with magnitude greater than 5 for an individual cluster
pub async fn calculate_time_since_last_significant_earthquake(
    cluster: &EarthquakeCluster,
) -> Option<Duration> {
    // Find the most recent significant earthquake for the cluster
    let mut most_recent_timestamp: Option<NaiveDateTime> = None;

    for earthquake in &cluster.events {
        if earthquake.magnitude > 5.0 {
            if most_recent_timestamp.is_none()
                || earthquake.timestamp > most_recent_timestamp.unwrap()
            {
                most_recent_timestamp = Some(earthquake.timestamp);
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
pub struct MetricStatistics {
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub count: usize,
    pub std: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub quantiles: HashMap<f64, f64>, // Quantile values and their corresponding quantile levels
}

// Function to calculate statistics for a single metric using DataFusion
async fn calculate_metric_statistics_async(metric_data: &Float64Array) -> MetricStatistics {
    // Dereference metric_data to access the underlying data
    let metric_data = *metric_data;

    let min = metric_data.min().unwrap_or(0.0);
    let max = metric_data.max().unwrap_or(0.0);
    let avg = metric_data.mean().unwrap_or(0.0);
    let count = metric_data.len();
    let std = metric_data.stddev().unwrap_or(0.0);
    let skewness = metric_data.skewness().unwrap_or(0.0);
    let kurtosis = metric_data.kurtosis().unwrap_or(0.0);

    // Calculate quantiles (25th, 50th, and 75th percentiles)
    let quantile_levels = vec![0.25, 0.5, 0.75];
    let quantile_values = quantile_levels
        .iter()
        .map(|&q| (q, metric_data.quantile(q).unwrap_or(0.0)))
        .collect::<HashMap<_, _>>();

    MetricStatistics {
        min,
        max,
        avg,
        count,
        std,
        skewness,
        kurtosis,
        quantiles: quantile_values,
    }
}

// Function to calculate statistics for depth and magnitude separately
pub async fn calculate_statistics_async(
    depth_data: &Float64Array,
    magnitude_data: &Float64Array,
) -> (MetricStatistics, MetricStatistics) {
    let depth_stats = calculate_metric_statistics_async(depth_data).await;
    let magnitude_stats = calculate_metric_statistics_async(magnitude_data).await;

    (depth_stats, magnitude_stats)
}

// Function to calculate statistics for a cluster
pub async fn calculate_cluster_statistics_async(
    cluster: &EarthquakeCluster,
) -> (MetricStatistics, MetricStatistics) {
    // Extract depth and magnitude data from the cluster's EarthquakeEvent instances
    let depth_data: Float64Array = cluster
        .events
        .iter()
        .map(|event| event.depth)
        .collect::<Vec<f64>>()
        .into();

    let magnitude_data: Float64Array = cluster
        .events
        .iter()
        .map(|event| event.magnitude)
        .collect::<Vec<f64>>()
        .into();

    let (depth_stats, magnitude_stats) = tokio::join!(
        calculate_metric_statistics_async(&depth_data),
        calculate_metric_statistics_async(&magnitude_data),
    );

    (depth_stats, magnitude_stats)
}

// Function to calculate statistics for all clusters asynchronously
pub async fn calculate_all_cluster_statistics_async(
    clusters: Vec<EarthquakeCluster>,
) -> Vec<ClusterStatistics> {
    let tasks = clusters.into_iter().map(|cluster| {
        let cluster_id = cluster.cluster_id;
        let centroid = cluster.centroid;

        // Calculate time since last significant earthquake for the cluster
        let duration_task = calculate_time_since_last_significant_earthquake(&cluster);

        task::spawn(async move {
            let (depth_stats, magnitude_stats) = calculate_cluster_statistics_async(&cluster).await;

            // Wait for the duration calculation to complete
            let duration = duration_task.await;

            ClusterStatistics {
                cluster_id,
                centroid,
                depth_stats,
                magnitude_stats,
                duration,
            }
        })
    });

    let results = tokio::join_all(tasks).await;
    results.into_iter().map(|res| res.unwrap()).collect()
}

// Define the ClusterStatistics struct to hold statistics for each cluster

pub struct ClusterStatistics {
    pub cluster_id: usize,
    pub centroid: (f64, f64),
    pub depth_stats: MetricStatistics,
    pub magnitude_stats: MetricStatistics,
    pub duration: Option<Duration>, // Time since last significant earthquake
}
