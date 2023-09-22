use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use arrow::record_batch::RecordBatch;
use clustering::ClusterStatistics;
use datafusion::arrow::array::{ArrayRef, Float64Array, Int64Array};
use datafusion::arrow::datatypes::ArrowNativeType;
use datafusion::datasource::MemTable;
use parquet::arrow::ArrowWriter;
use parquet::file::writer::{FileWriter, FileWriterOptions};
use std::collections::HashMap;
use std::fs::File;
use std::io::Result;
use std::sync::Arc;

// Function to write ClusterStatistics to a Parquet file
pub fn write_cluster_statistics_to_parquet(
    cluster_statistics: Vec<ClusterStatistics>,
    output_file: &str,
) -> Result<()> {
    // Create a DataFusion Schema for the Parquet file
    let schema = create_parquet_schema();

    // Convert ClusterStatistics to RecordBatch
    let record_batch = create_record_batch(&cluster_statistics, &schema);

    // Create a MemTable from the RecordBatch
    let mem_table = MemTable::try_new(schema.clone(), vec![Arc::new(record_batch)])?;

    // Create Parquet writer options
    let options = FileWriterOptions {
        write_statistics: true,
        compression: parquet::basic::Compression::Snappy,
        ..Default::default()
    };

    // Create a Parquet writer
    let file = File::create(output_file)?;
    let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(options))?;

    // Write the MemTable to the Parquet file
    writer.write(&mem_table)?;

    // Finish writing and close the file
    writer.close()?;

    Ok(())
}

// Function to create a DataFusion Schema for the Parquet file
fn create_parquet_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("cluster_id", DataType::Int64, false),
        Field::new("centroid_lon", DataType::Float64, false),
        Field::new("centroid_lat", DataType::Float64, false),
        Field::new("depth_min", DataType::Float64, false),
        Field::new("depth_max", DataType::Float64, false),
        Field::new("depth_avg", DataType::Float64, false),
        Field::new("depth_count", DataType::Int64, false),
        Field::new("depth_std", DataType::Float64, false),
        Field::new("depth_skewness", DataType::Float64, false),
        Field::new("depth_kurtosis", DataType::Float64, false),
        Field::new(
            "depth_quantiles",
            DataType::Dictionary(
                Box::new(DataType::Float64),
                Box::new(DataType::Float64),
                false,
            ),
            false,
        ),
        Field::new("magnitude_min", DataType::Float64, false),
        Field::new("magnitude_max", DataType::Float64, false),
        Field::new("magnitude_avg", DataType::Float64, false),
        Field::new("magnitude_count", DataType::Int64, false),
        Field::new("magnitude_std", DataType::Float64, false),
        Field::new("magnitude_skewness", DataType::Float64, false),
        Field::new("magnitude_kurtosis", DataType::Float64, false),
        Field::new(
            "magnitude_quantiles",
            DataType::Dictionary(
                Box::new(DataType::Float64),
                Box::new(DataType::Float64),
                false,
            ),
            false,
        ),
        Field::new("duration", DataType::Int64, false), // Add the duration field
    ]))
}

fn create_record_batch(
    cluster_statistics: &Vec<ClusterStatistics>,
    schema: &SchemaRef,
) -> RecordBatch {
    // Create arrays for each field
    let cluster_id_array: ArrayRef = Arc::new(Int64Array::from(
        cluster_statistics
            .iter()
            .map(|stats| stats.cluster_id as i64)
            .collect::<Vec<_>>(),
    ));

    // ... (other fields)

    // Create an array for the duration field
    let duration_array: ArrayRef = Arc::new(Int64Array::from(
        cluster_statistics
            .iter()
            .map(|stats| {
                if let Some(duration) = &stats.duration {
                    duration.num_seconds() as i64
                } else {
                    0 // Default value for duration if None
                }
            })
            .collect::<Vec<_>>(),
    ));

    // Create a HashMap for the arrays
    let arrays: HashMap<&str, ArrayRef> = vec![
        ("cluster_id", cluster_id_array),
        ("centroid_lon", centroid_lon_array),
        ("centroid_lat", centroid_lat_array),
        ("depth_min", depth_min_array),
        ("depth_max", depth_max_array),
        ("depth_avg", depth_avg_array),
        ("depth_count", depth_count_array),
        ("depth_std", depth_std_array),
        ("depth_skewness", depth_skewness_array),
        ("depth_kurtosis", depth_kurtosis_array),
        ("depth_quantiles", depth_quantiles_array),
        ("magnitude_min", magnitude_min_array),
        ("magnitude_max", magnitude_max_array),
        ("magnitude_avg", magnitude_avg_array),
        ("magnitude_count", magnitude_count_array),
        ("magnitude_std", magnitude_std_array),
        ("magnitude_skewness", magnitude_skewness_array),
        ("magnitude_kurtosis", magnitude_kurtosis_array),
        ("magnitude_quantiles", magnitude_quantiles_array),
        ("duration", duration_array), // Add the duration field
    ]
    .into_iter()
    .collect();

    // Create a RecordBatch from the arrays
    RecordBatch::try_new(Arc::clone(schema), arrays).expect("Failed to create RecordBatch")
}
