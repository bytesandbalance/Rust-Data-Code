pub mod fetch;
use dotenvy::dotenv;
use fetch::run_fetch;
use sqlx::postgres::PgPoolOptions;
use std::env;
use store_sqlx::{convert_to_model, insert_earthquake_events, EarthquakeEventModel};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let start_time = "2015-01-01";
    let end_time = "2015-01-02";
    let min_magnitude = 3;

    let eqs = run_fetch(start_time, end_time, min_magnitude).await?;
    let earthquake_models: Vec<EarthquakeEventModel> = convert_to_model(eqs);
    insert_earthquake_events(&pool, earthquake_models).await?;

    Ok(())
}
