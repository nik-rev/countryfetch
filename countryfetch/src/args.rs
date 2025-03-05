use clap::Parser;

use crate::generated;

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles=get_styles())]
pub struct Args {
    /// Print information about a specific country
    #[arg(long)]
    pub country: generated::Country,
    /// Print information about all countries
    #[arg(long)]
    pub all_countries: bool,
    /// Exclude information about area of country
    #[arg(long)]
    pub no_area: bool,
    /// Exclude ASCII flag art
    #[arg(long)]
    pub no_flag: bool,
    /// Exclude country emoji
    #[arg(long)]
    pub no_emoji: bool,
    /// Exclude continent
    #[arg(long)]
    pub no_continent: bool,
    /// Exclude population
    #[arg(long)]
    pub no_population: bool,
    /// Exclude top-level domain
    #[arg(long)]
    pub no_tld: bool,
    /// Exclude languages
    #[arg(long)]
    pub no_languages: bool,
    /// Exclude currency
    #[arg(long)]
    pub no_currency: bool,
    /// Exclude neighbours
    #[arg(long)]
    pub no_neighbours: bool,
    /// Exclude established date
    #[arg(long)]
    pub no_established_date: bool,
    /// Exclude iso codes
    #[arg(long)]
    pub no_iso_codes: bool,
    /// Exclude driving side
    #[arg(long)]
    pub no_driving_side: bool,
    /// Exclude capital
    #[arg(long)]
    pub no_capital: bool,
    /// Exclude dialing code
    #[arg(long)]
    pub no_dialing_code: bool,
    /// Exclude the country flag's palette
    #[arg(long)]
    pub no_palette: bool,
    /// No colored output
    #[arg(long)]
    pub no_color: bool,
}
