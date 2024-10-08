use common::blocking::fetch::run_fetch;
use store_diesel::{convert_to_model, establish_connection, insert_earthquake_events};

fn main() -> anyhow::Result<()> {
    let start_time = "2014-01-01";
    let end_time = "2014-01-02";
    let min_magnitude = 3;
    let connection = &mut establish_connection();

    let eqs = run_fetch(start_time, end_time, min_magnitude)?;
    let eqs_model: Vec<store_diesel::models::EarthquakeEventModel> = convert_to_model(eqs);
    insert_earthquake_events(connection, eqs_model)?;

    Ok(())
}
