//! Countryfetch

use std::env;

use clap::Parser as _;

#[expect(clippy::allow_attributes, reason = "may or may not do IO")]
#[allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "print from `main` is ok"
)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let args = countryfetch::Args::parse();

    if args.no_color {
        // SAFETY: This runs in a single-threaded environment
        unsafe {
            env::set_var("NO_COLOR", "1");
        }
    }

    println!();

    args.print().await?;

    Ok(())
}
