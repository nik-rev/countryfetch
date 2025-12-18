//! Countryfetch

use eyre::Result;
use std::env;
use strum::VariantArray;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut args = <countryfetch::Cli as clap::Parser>::parse();

    if args.select.is_empty() {
        args.select = countryfetch::cli::DataPiece::VARIANTS.to_vec();
    }

    if args.no_color {
        // SAFETY: This runs in a single-threaded environment
        unsafe {
            env::set_var("NO_COLOR", "1");
        }
    }
    args.print()?;

    Ok(())
}
