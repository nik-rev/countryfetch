#![allow(clippy::cargo_common_metadata, reason = "idk")]
#![allow(clippy::multiple_crate_versions, reason = "idk")]

use std::env;

use clap::Parser as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let args = countryfetch::Args::parse();

    if args.no_color {
        // SAFETY: Caller ensures this runs in a single-threaded environment
        unsafe {
            env::set_var("NO_COLOR", "1");
        }
    }

    println!();

    args.print().await?;

    Ok(())
}
