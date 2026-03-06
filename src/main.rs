use std::io::Write;

use anstream::eprintln;
use anstream::println;
use clap::Parser;
use eyre::Result;

use crate::countries::Country;

mod cli;
mod countries;
mod country_display;
mod extra_country_data;
mod gen_countries;

fn main() -> Result<()> {
    let cli = <cli::Cli as Parser>::parse();

    if let Some(country) = cli.country {
        if cli.all {
            eprintln!("you cannot specify both a country and --all countries");
            std::process::exit(1);
        }

        if cli.json {
            println!("{}", colored_json::to_colored_json_auto(country.data())?)
        } else {
            println!("{}", country.data())
        }
    } else if cli.all {
        let countries: &[&Country] = &gen_countries::all_countries();

        if cli.json {
            println!("{}", colored_json::to_colored_json_auto(&countries)?);
        } else {
            let mut stdout = std::io::stdout().lock();
            for country in countries {
                stdout.write_all(country.to_string().as_bytes())?;
                stdout.write_all(b"\n")?;
            }
        }
    } else {
        eprintln!("either specify a country, or --all countries");
        std::process::exit(1);
    }

    Ok(())
}
