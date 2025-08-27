//! The command-line interface

use core::error;

use anstyle::AnsiColor;
use anstyle::Effects;
use clap::Parser;
use clap::ValueEnum as _;

use crate::Country;
use crate::Location;
use crate::cache::Cache;
use crate::country_format::format_country;
use crate::generated_country_data;

/// Styles for the CLI
const STYLES: clap::builder::Styles = clap::builder::Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::BrightCyan.on_default())
    .error(AnsiColor::BrightRed.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::BrightYellow.on_default().effects(Effects::BOLD));

/// Countryfetch's arguments
#[derive(Parser, Debug)]
#[command(version, author = "Nik Revenco", about, long_about = None, styles = STYLES)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Clap is expected to have many command line arguments"
)]
pub struct Args {
    /// A list of countries to generate output for. If it's empty, detect the
    /// country
    #[clap(hide_possible_values = true, ignore_case = true)]
    pub country: Option<Vec<generated_country_data::Country>>,
    /// Print information about all countries
    #[arg(long)]
    pub all_countries: bool,
    /// Print a list of all countries that can be passed
    #[arg(long)]
    pub list_countries: bool,
    /// Exclude information about area of country
    #[arg(long, help_heading = "Config")]
    pub no_area: bool,
    /// Exclude ASCII flag art
    #[arg(long, help_heading = "Config")]
    pub no_flag: bool,
    /// Exclude country emoji
    #[arg(long, help_heading = "Config")]
    pub no_emoji: bool,
    /// Exclude continent
    #[arg(long, help_heading = "Config")]
    pub no_continent: bool,
    /// Exclude population
    #[arg(long, help_heading = "Config")]
    pub no_population: bool,
    /// Exclude top-level domains
    #[arg(long, help_heading = "Config", alias = "no-tld")]
    pub no_tlds: bool,
    /// Exclude languages
    #[arg(long, help_heading = "Config", alias = "no-language")]
    pub no_languages: bool,
    /// Exclude currencies
    #[arg(long, help_heading = "Config", alias = "no-currency")]
    pub no_currencies: bool,
    /// Exclude neighbours
    #[arg(long, help_heading = "Config", aliases = ["no-neighbour", "no-neighbors", "no-neighbor"])]
    pub no_neighbours: bool,
    /// Exclude established date
    #[arg(long, help_heading = "Config")]
    pub no_established_date: bool,
    /// Exclude iso codes
    #[arg(long, help_heading = "Config", alias = "no-iso-code")]
    pub no_iso_codes: bool,
    /// Exclude driving side
    #[arg(long, help_heading = "Config")]
    pub no_driving_side: bool,
    /// Exclude capital
    #[arg(long, help_heading = "Config")]
    pub no_capital: bool,
    /// Exclude dialing code
    #[arg(long, help_heading = "Config")]
    pub no_dialing_code: bool,
    /// Exclude the country flag's palette
    #[arg(long, help_heading = "Config")]
    pub no_palette: bool,
    /// No colored output
    #[arg(long, help_heading = "Config")]
    pub no_color: bool,
}

impl Args {
    /// # Panics
    ///
    /// - Stored invalid 2 letter country code in the cache file
    #[expect(clippy::print_stdout, reason = "this is where we print the country")]
    pub async fn print(self) -> Result<(), Box<dyn error::Error>> {
        if self.list_countries {
            println!(
                "`countryfetch` accepts any of the below values as an input.
You can either use the country name, or the 2-letter country code. Case-insensitive."
            );
            for country in generated_country_data::Country::ALL_COUNTRIES {
                if let Some(value) = country.to_possible_value() {
                    let aliases = value
                        .get_name_and_aliases()
                        .collect::<Vec<&str>>()
                        .join(", ");
                    println!("{} {aliases}", country.emoji());
                }
            }
            return Ok(());
        } else if self.all_countries {
            for country in generated_country_data::Country::ALL_COUNTRIES {
                let out = format_country(*country, None, None, &self);
                println!("{out}");
            }
        } else if let Some(countries) = &self
            .country
            .as_ref()
            .and_then(|v| (!v.is_empty()).then_some(v))
        {
            for country in *countries {
                let out = format_country(*country, None, None, &self);
                println!("{out}");
            }
        } else if let Some(cache) = Cache::read() {
            let gen_country = generated_country_data::Country::country_code3_from_country_code2(
                &cache.country_code,
            )
            .and_then(generated_country_data::Country::from_country_code)
            .expect("Stored a valid 2 letter country code in cache");

            println!("{}", format_country(gen_country, None, None, &self));
        } else {
            let ip = public_ip::addr()
                .await
                .ok_or("Error: Unable to retrieve your public IP.")?;

            let location = Location::from_ip(ip).await?;
            let country = Country::from_cc2(&location.country_code).await.ok();
            let gen_country = generated_country_data::Country::from_country_code(
                generated_country_data::Country::country_code3_from_country_code2(
                    &location.country_code,
                )
                .expect(
                    "Location's country_code will always be valid 2 letter countrycode that can \
                     be converted into a 3 letter country code",
                ),
            )
            .expect("Generated country code must exist");

            let _ = Cache::write(location.country_code.clone());

            println!(
                "{}",
                format_country(gen_country, country.as_ref(), Some(&location), &self)
            );
        }

        Ok(())
    }
}
