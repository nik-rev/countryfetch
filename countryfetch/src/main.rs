use std::env;

use clap::Parser;

mod generated;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lol = std::sync::Arc::new(4);

    let args = countryfetch::Args::parse();

    if args.no_color {
        // SAFETY: Caller ensures this runs in a single-threaded environment
        unsafe {
            env::set_var("NO_COLOR", "1");
        }
    };

    println!();

    args.print().await?;

    Ok(())
}
