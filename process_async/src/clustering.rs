use common::earthquake_event::EarthquakeEvent;
use ndarray::{Array, Array2};
use rusty_machine::linalg::Matrix;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Clustering Failed")]
    ClusteringFailed,
}

// Import your EarthquakeEvent struct and other necessary modules here

// Define a struct to represent a cluster of earthquake events
#[derive(Debug)]
pub struct EarthquakeCluster {
    pub events: Vec<EarthquakeEvent>,
    pub centroid: (f64, f64), // (lon, lat) coordinates of the cluster centroid
}

// Function to perform K-means clustering on earthquake events
pub fn cluster_earthquake_events(
    events: Vec<EarthquakeEvent>,
    k: usize, // Number of clusters
) -> Result<Vec<EarthquakeCluster>, Error> {
    // Extract coordinates (lon, lat) from the earthquake events
    let coordinates: Vec<(f64, f64)> = events
        .iter()
        .map(|event| (event.coordinates.lon, event.coordinates.lat))
        .collect();

    // Create a matrix from the coordinates
    let data = Array::from(coordinates);
    let matrix = Matrix::new(k, data, events);

    // Perform K-means clustering
    let kmeans = KMeans::new(&matrix, k);
    let results = kmeans.train(&matrix);

    match results {
        Ok(model) => {
            let labels = model.labels();

            // Organize clustered events
            let mut clusters: HashMap<usize, Vec<EarthquakeEvent>> = HashMap::new();

            for (event_idx, cluster_idx) in labels.iter().enumerate() {
                let cluster = clusters.entry(*cluster_idx).or_insert(Vec::new());
                cluster.push(events[event_idx].clone());
            }

            // Calculate centroids
            let mut earthquake_clusters: Vec<EarthquakeCluster> = Vec::new();

            for (_, &cluster_events) in clusters.iter() {
                let centroid = calculate_centroid(cluster_events.as_ref());
                earthquake_clusters.push(EarthquakeCluster {
                    events: cluster_events,
                    centroid,
                });
            }

            Ok(earthquake_clusters)
        }
        Err(_) => Err(Error::ClusteringFailed),
    }
}

// Function to calculate the centroid of a cluster
fn calculate_centroid(events: &[EarthquakeEvent]) -> (f64, f64) {
    let total_lat: f64 = events.iter().map(|event| event.coordinates.lat).sum();
    let total_lon: f64 = events.iter().map(|event| event.coordinates.lon).sum();
    let count = events.len() as f64;

    (total_lon / count, total_lat / count)
}
