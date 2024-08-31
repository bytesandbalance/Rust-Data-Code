use polars::prelude::*;
use common::earthquake_event::EarthquakeEvent;

pub fn events_to_dataframe(events: Vec<EarthquakeEvent>) -> Result<DataFrame, PolarsError> {
    let timestamps: Vec<i64> = events.iter().map(|e| e.time).collect();
    let magnitudes: Vec<f64> = events.iter().map(|e| e.mag).collect();
    let latitudes: Vec<f64> = events.iter().map(|e| e.coordinates.lat).collect();
    let longitudes: Vec<f64> = events.iter().map(|e| e.coordinates.lon).collect();

    let df = DataFrame::new(vec![
        Series::new("timestamp", timestamps),
        Series::new("magnitude", magnitudes),
        Series::new("latitude", latitudes),
        Series::new("longitude", longitudes),
    ])?;

    Ok(df)
}

pub async fn temporal_analysis(df: &DataFrame) -> Result<DataFrame, PolarsError> {
    let df = df.clone()
        .lazy()
        .with_columns([
            // Convert the "timestamp" column from milliseconds to datetime
            col("timestamp")
                .cast(DataType::Datetime(TimeUnit::Milliseconds, None))
                .alias("datetime"),
        ])
        .with_columns([
            // Extract the month from the "datetime" column
            col("datetime")
                .dt()
                .month()
                .alias("month"),
            // Extract the year from the "datetime" column
            col("datetime")
                .dt()
                .year()
                .alias("year"),
        ])
        .group_by([col("year"), col("month")])
        .agg([
            // Count the number of "magnitude" entries per group
            col("magnitude").count().alias("count"),
        ])
        .collect()?;

    Ok(df)
}
