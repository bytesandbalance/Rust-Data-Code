// File: src/main.rs or src/lib.rs

// Import the function from the module
pub mod chapter_2 {
    pub mod fetch_sync; // Import the fetch_sync module
}

pub mod earthquake_event;
pub mod utils;

use chapter_2::fetch_sync::run_fetch_sync; // Import the function

fn main() -> anyhow::Result<()> {
    run_fetch_sync();

    Ok(())
}
