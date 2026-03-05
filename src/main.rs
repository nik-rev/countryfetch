use std::io::Write;

use anstream::eprintln;
use anstream::println;
use anstyle::AnsiColor;
use anstyle::Effects;
use clap::Parser;
use eyre::Result;

use crate::countries::COUNTRIES_DATA;
use crate::countries::Country;

mod countries;
mod country_display;
mod extra_country_data;
mod gen_countries;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = <Cli as Parser>::parse();

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

use simply_colored::*;

fn rgb(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{r};{g};{b}m")
}

fn bolden((r, g, b): (u8, u8, u8), word: &str, alias: &str) -> String {
    let mut alias_chars = alias.chars().peekable();

    let mut output = String::new();

    for word_char in word.chars() {
        if alias_chars
            .next_if(|alias_char| alias_char.eq_ignore_ascii_case(&word_char))
            .is_some()
        {
            let color = rgb(r, g, b);
            output.push_str(&format!("{BOLD}{color}{word_char}{RESET}"));
        } else {
            output.push(word_char);
        }
    }

    if alias_chars.peek().is_some() {
        // The country's name does not contain every character in the alias,
        // so the alias would not be valid to highlight. Highlight nothing at all.
        word.to_string()
    } else {
        output
    }
}

fn countries() -> String {
    let mut output: Vec<String> = Vec::new();

    for country in COUNTRIES_DATA.0.iter().map(|country| {
        format!(
            "{}{}",
            country
                .flag
                .as_ref()
                .map(|flag| format!("{flag} "))
                .unwrap_or_default(),
            bolden(
                country_display::brightest_color(country),
                &country.name.common,
                &country.cca2
            )
        )
    }) {
        if let Some(last) = output.last_mut()
            && anstream::adapter::strip_str(last).to_string().len() <= 140
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

#[derive(Parser, Debug)]
#[command(version, about, styles = STYLES, arg_required_else_help = true)]
pub struct Cli {
    /// Country to print. (case-insensitive)
    #[clap(ignore_case = true, hide_possible_values = true, help = format!("Country to show data for. {BOLD}Choose any of:{NO_BOLD}\n\n{}", countries()))]
    pub country: Option<gen_countries::CountryKind>,
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
