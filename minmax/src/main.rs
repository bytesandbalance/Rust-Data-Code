use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    // Sample data in a Series
    let s = Series::new("", &[1, 2, 3, 4, 5, 23]);

    // Calculate skewness
    let skew = s.skew(false)?;

    // Print the skewness value or "N/A" if it's None
    match skew {
        Some(value) => println!("Skewness: {}", value),
        None => println!("Skewness: N/A"),
    }

    Ok(())
}
