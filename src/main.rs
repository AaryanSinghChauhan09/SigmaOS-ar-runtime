//! SigmaOS-ar-runtime - SigmaOS Component

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting SigmaOS-ar-runtime");
    println!("SigmaOS SigmaOS-ar-runtime v0.1.0");
    Ok(())
}
