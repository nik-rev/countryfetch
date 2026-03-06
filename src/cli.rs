//! Command-line interface for `countryfetch`

use anstyle::AnsiColor;
use anstyle::Effects;
use clap::Parser;
use simply_colored::*;

use crate::countries::COUNTRIES_DATA;
use crate::country_display;
use crate::country_display::AnsiRgb;

#[derive(Parser)]
#[command(version, about, styles = STYLES)]
pub struct Cli {
    /// Country to print. (case-insensitive)
    #[clap(ignore_case = true, hide_possible_values = true, help = format!("Country to show data for. {BOLD}Choose any of:{NO_BOLD}\n\n{}", display_possible_countries()))]
    pub country: Option<crate::gen_countries::CountryKind>,
    /// Print all countries
    #[arg(short, long)]
    pub all: bool,
    /// Print in JSON format
    #[arg(short, long)]
    pub json: bool,
}

/// Styles for the CLI
const STYLES: clap::builder::Styles = clap::builder::Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::BrightCyan.on_default())
    .error(AnsiColor::BrightRed.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::BrightYellow.on_default().effects(Effects::BOLD));

/// Shows the list of countries that the user can use, displayed on --help
fn display_possible_countries() -> String {
    let mut output: Vec<String> = Vec::new();

    for country in COUNTRIES_DATA.0.iter() {
        let flag = country
            .flag
            .as_ref()
            .map(|flag| format!("{flag} "))
            .unwrap_or_default();
        let country_name = highlight_country_name(
            &country.name.common,
            &country.cca2,
            country_display::brightest_color(country),
        );
        let country = format!("{flag}{country_name}");

        const MAX_WIDTH: usize = 140;

        if let Some(last) = output.last_mut()
            && anstream::adapter::strip_str(last).to_string().len() <= MAX_WIDTH
        {
            last.push(' ');
            last.push(' ');
            last.push_str(&country);
        } else {
            output.push(country);
        }
    }

    output.join("\n")
}

/// Given a `country_name` such as "United Kingdom" and an `alias` such as "UK", highlights
/// the first matching letters of the country: "𝗨nited 𝗞ingdom"
fn highlight_country_name(country_name: &str, alias: &str, highlight_color: AnsiRgb) -> String {
    let mut alias_chars = alias.chars().peekable();

    let mut output = String::new();

    for word_char in country_name.chars() {
        if alias_chars
            .next_if(|alias_char| alias_char.eq_ignore_ascii_case(&word_char))
            .is_some()
        {
            output.push_str(&format!("{BOLD}{highlight_color}{word_char}{RESET}"));
        } else {
            output.push(word_char);
        }
    }

    if alias_chars.peek().is_some() {
        // The country's name does not contain every character in the alias,
        // so the alias would not be valid to highlight. Highlight nothing at all.
        country_name.to_string()
    } else {
        output
    }
}
