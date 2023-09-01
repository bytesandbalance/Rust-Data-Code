pub mod fetch_sync;
use fetch_sync::run_fetch_sync; // Import the function

fn main() -> anyhow::Result<()> {
    run_fetch_sync();

    Ok(())
}
