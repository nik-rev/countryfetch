//! The command-line interface

use clap::ValueEnum;
use eyre::Result;
use strum::VariantArray;

use crate::country_format::format_country;
use crate::gen_countries;
use anstyle::AnsiColor;
use anstyle::Effects;
use clap::Parser;

/// Styles for the CLI
const STYLES: clap::builder::Styles = clap::builder::Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::BrightCyan.on_default())
    .error(AnsiColor::BrightRed.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::BrightYellow.on_default().effects(Effects::BOLD));

#[derive(clap::ValueEnum, Clone, Debug, strum::VariantArray, strum::Display)]
#[strum(serialize_all = "kebab-case")]
//
pub enum DataPiece {
    Area,
    Flag,
    Emoji,
    Continent,
    Population,
    Tlds,
    Languages,
    Currencies,
    Neighbours,
    EstablishmentDate,
    IsoCode,
    DrivingSide,
    Capital,
    DialingCode,
    Palette,
    Color,
}

/// Countryfetch's arguments
#[derive(Parser, Debug)]
#[command(version, author = "Nik Revenco", about, long_about = None, styles = STYLES)]
pub struct Cli {
    /// A list of countries to generate output for. If it's empty, detect the
    /// country
    #[clap(hide_possible_values = true, ignore_case = true)]
    pub country: Option<Vec<gen_countries::CountryKind>>,
    /// Print information about all countries
    #[arg(long)]
    pub all_countries: bool,
    /// Print a list of all countries that can be passed
    #[arg(long)]
    pub list_countries: bool,
    /// Select which fields to show
    #[arg(long)]
    pub select: Vec<DataPiece>,
}

impl Cli {
    /// # Panics
    ///
    /// - Stored invalid 2 letter country code in the cache file
    pub fn print(self) -> Result<()> {
        println!();

        if self.list_countries {
            println!(
                "`countryfetch` accepts any of the below values as an input.
You can either use the country name, or the 2-letter country code. Case-insensitive."
            );
            for country in gen_countries::CountryKind::VARIANTS {
                if let Some(value) = country.to_possible_value() {
                    let aliases = value
                        .get_name_and_aliases()
                        .collect::<Vec<&str>>()
                        .join(", ");
                    if let Some(emoji) = &country.data().flag {
                        println!("{emoji} {aliases}");
                    } else {
                        println!("{aliases}");
                    }
                }
            }
            return Ok(());
        } else if self.all_countries {
            for country in gen_countries::all_countries() {
                let out = format_country(country, &self);
                println!("{out}");
            }
        } else if let Some(countries) = &self
            .country
            .as_ref()
            .and_then(|v: &Vec<gen_countries::CountryKind>| (!v.is_empty()).then_some(v))
        {
            for country in *countries {
                let out = format_country(country.data(), &self);
                println!("{out}");
            }
        } else {
            panic!("argument is required");
        }

        Ok(())
    }
}
