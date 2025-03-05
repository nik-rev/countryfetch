use clap::Parser;
use countryfetch::args;

mod generated;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = countryfetch::Args::parse();

    // SAFETY: Runs in a single-threaded environment
    unsafe {
        args::print_args(args).await?;
    }

    Ok(())
}
