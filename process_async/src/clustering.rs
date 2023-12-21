use common::earthquake_event::EarthquakeEvent;
use linfa::traits::{Fit, Predict};
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use ndarray::Array2;
use ndarray_rand::rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Clustering Failed")]
    ClusteringFailed,
}
 
// Define a struct to represent a cluster of earthquake events
#[derive(Debug, Clone)]
pub struct EarthquakeCluster {
    pub events: Vec<EarthquakeEvent>,
    pub centroid: (f64, f64), // (lon, lat) coordinates of the cluster centroid
}

// Function to convert EarthquakeEvents to Linfa observations
pub fn convert_to_linfa_observations(events: &[EarthquakeEvent]) -> Vec<[f64; 2]> {
    events
        .iter()
        .map(|event| [event.coordinates.lon, event.coordinates.lat])
        .collect()
}

// Function to perform K-means clustering on earthquake events
pub fn cluster_earthquake_events(
    events: Vec<EarthquakeEvent>,
    n_clusters: usize, // Number of clusters
) -> Result<Vec<EarthquakeCluster>, Error> {
    // Convert EarthquakeEvents to Linfa observations
    let observations: Vec<[f64; 2]> = convert_to_linfa_observations(&events);

    // Convert Vec<[f64; 2]> to ndarray::Array2<f64>
    let mut data_array = Array2::zeros((observations.len(), 2));
    for (i, point) in observations.iter().enumerate() {
        data_array[[i, 0]] = point[0];
        data_array[[i, 1]] = point[1];
    }
    let observations = DatasetBase::from(data_array);

    // Our random number generator, seeded for reproducibility
    let seed = 42;
    let mut rng = Xoshiro256Plus::seed_from_u64(seed);

    // Configure and run the K-means algorithm
    let model = KMeans::params_with_rng(n_clusters, rng.clone())
        .tolerance(1e-2)
        .fit(&observations)
        .expect("KMeans fitted");

    // Get cluster assignments for each data point
    let cluster_assignments = model
        .predict(observations)
        .targets()
        .iter()
        .map(|&cluster_idx| cluster_idx as usize)
        .collect::<Vec<usize>>();

    // Initialize EarthquakeCluster instances
    let mut clusters: Vec<EarthquakeCluster> = vec![
        EarthquakeCluster {
            events: Vec::new(),
            centroid: (0.0, 0.0),
        };
        n_clusters
    ];

    // Assign earthquake events to their respective clusters
    for (event, cluster_idx) in events.into_iter().zip(cluster_assignments) {
        clusters[cluster_idx].events.push(event);
    }

    // Calculate centroids for each cluster
    for cluster in &mut clusters {
        let sum_lon: f64 = cluster
            .events
            .iter()
            .map(|event| event.coordinates.lon)
            .sum();
        let sum_lat: f64 = cluster
            .events
            .iter()
            .map(|event| event.coordinates.lat)
            .sum();
        let count = cluster.events.len() as f64;

        if count > 0.0 {
            cluster.centroid = (sum_lon / count, sum_lat / count);
        }
    }

    Ok(clusters)
}

// Function to calculate the centroid of a cluster
fn calculate_centroid(events: &[EarthquakeEvent]) -> (f64, f64) {
    let total_lat: f64 = events.iter().map(|event| event.coordinates.lat).sum();
    let total_lon: f64 = events.iter().map(|event| event.coordinates.lon).sum();
    let count = events.len() as f64;

    (total_lon / count, total_lat / count)
}
