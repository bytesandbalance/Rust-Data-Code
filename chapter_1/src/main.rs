// File: src/main.rs or src/lib.rs

pub mod fetch;
use fetch::run_fetch; // Import the function

fn main() -> anyhow::Result<()> {
    let start_time = "2014-01-01";
    let end_time = "2014-01-02";
    let min_magnitude = 3;

    let earthquake_events = run_fetch(start_time, end_time, min_magnitude)?;
    for event in earthquake_events {
        println!("{:?}", event);
    }

    Ok(())
}
