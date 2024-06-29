pub mod clustering;
pub mod statistics;

use std::error::Error;

use opentelemetry::{global::shutdown_tracer_provider, KeyValue};
use opentelemetry_sdk::{trace, Resource};
use tracing::{instrument, level_filters::LevelFilter, Instrument, Span};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer, Registry};

use clustering::cluster_earthquake_events;
use common::fetch::run_fetch;
use statistics::calculate_all_cluster_statistics_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let console_env_filter = EnvFilter::builder()
        .with_env_var("RUST_LOG")
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let console_logger = tracing_subscriber::fmt::layer().with_filter(console_env_filter);

    // Change the endpoint via OTEL_EXPORTER_OTLP_ENDPOINT. Default is http://localhost:4317/
    let otlp_exporter = opentelemetry_otlp::new_exporter().tonic();

    let otlp_tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "earthquake_tracing_app",
            )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    let tracing_env_filter = EnvFilter::builder()
        // .with_env_var("RUST_LOG")
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let telemetry = tracing_opentelemetry::layer()
        .with_tracer(otlp_tracer)
        .with_filter(tracing_env_filter);
    let subscriber = Registry::default().with(telemetry).with(console_logger);
    tracing::subscriber::set_global_default(subscriber)?;

    process(chrono::Utc::now(), chrono::Duration::days(5 /* * 365 */)).await?;

    tracing::info!("sleeping before shutting down trace providers");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    shutdown_tracer_provider();

    // If the program reaches this point, everything executed successfully
    Ok(())
}

// Manually record the fields, else we get ugly debug output
#[instrument(skip_all, fields(end_date, back_until_seconds))]
async fn process(
    end_date: chrono::DateTime<chrono::Utc>,
    back_until: chrono::Duration,
) -> Result<(), Box<dyn Error>> {
    // Define the start date as ten years ago from today
    let end_date = end_date.date_naive();
    let start_date = end_date - back_until;

    Span::current()
        .record("end_date", format!("{end_date}"))
        .record("back_until_seconds", back_until.num_seconds() as i64);

    // Create a date iterator to fetch data monthly
    let mut current_date = start_date;
    let mut tasks = Vec::new(); // Vector to hold asynchronous tasks

    while current_date < end_date {
        let next_2day = current_date + chrono::Duration::days(2);
        let start_time = current_date.to_string();
        let end_time = next_2day.to_string();
        let min_magnitude = 3; // Set your desired minimum magnitude here

        tracing::info!("Fetching data");

        // We need get the current span to instrument the async call with.
        // Otherwise, the spawned task ends up in a separate trace tree.
        let current_span = Span::current();

        // Spawn asynchronous task for fetching data and push it to the vector
        tasks.push(tokio::spawn(async move {
            run_fetch(&start_time, &end_time, min_magnitude)
                .instrument(current_span)
                .await
        }));

        // Move to the next month
        current_date = next_2day;
    }

    let mut all_earthquake_events = Vec::new();
    for task in tasks {
        let earthquake_events = task.await??; // Unwrap the Result twice
        all_earthquake_events.extend(earthquake_events);
    }

    // Set the number of clusters for k-means clustering
    let k = 20; // Adjust as needed

    // Cluster the earthquake events
    let clusters = cluster_earthquake_events(all_earthquake_events, k)?;

    // Calculate statistics for each cluster asynchronously
    let clusters_statistics = calculate_all_cluster_statistics_async(clusters).await?;

    // Print each ClusterStatistics
    for cluster_statistic in clusters_statistics {
        tracing::info!(?cluster_statistic, "cluster statistic");
    }

    Ok(())
}
