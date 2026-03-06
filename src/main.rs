//! Things left to do
//!
//! - Vertically align the country flag in the middle
//! - Fancy display for the "Neighbours", so it shows the full country name
//! - Fix numeric value of the "Area" field
//! - Round population to the nearest million, e.g "13.4 million" or "63.1 thousand" instead of exact count

use std::io::Write;

use anstream::println;
use clap::Parser;
use eyre::ContextCompat;
use eyre::Result;
use serde::Deserialize;

use crate::countries::COUNTRIES_DATA;
use crate::countries::Country;

mod cli;
mod countries;
mod country_display;
mod extra_country_data;
mod gen_countries;

fn main() -> Result<()> {
    let cli = <cli::Cli as Parser>::parse();

    match cli.all {
        false => {
            let country = if let Some(country) = cli.country {
                country.data()
            } else {
                // No specific country provided, so we'll detect it

                #[derive(Deserialize)]
                struct Response {
                    /// 2-letter country code (cca2)
                    country: String,
                }

                // get current country
                let cca2 = ureq::get("https://api.country.is")
                    .header("User-Agent", "countryfetch")
                    .call()?
                    .body_mut()
                    .read_json::<Response>()?
                    .country;

                COUNTRIES_DATA
                    .0
                    .iter()
                    .find(|country| country.cca2 == cca2)
                    .wrap_err_with(|| format!("no country with cca2 {cca2} found"))?
            };

            if cli.json {
                println!("{}", colored_json::to_colored_json_auto(country)?)
            } else {
                println!("{}", country)
            }
        }
        true => {
            let countries: &[&Country] = &gen_countries::all_countries();

            if cli.json {
                println!("{}", colored_json::to_colored_json_auto(&countries)?);
            } else {
                let mut stdout = anstream::stdout().lock();

                for country in countries {
                    stdout.write_all(country.to_string().as_bytes())?;
                    stdout.write_all(b"\n")?;
                }
            }
        }
    }

    Ok(())
}
