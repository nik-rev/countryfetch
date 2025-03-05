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
#[command(version, version = "Nik Revenco", about, long_about = None, styles=get_styles())]
pub struct Args {
    #[clap(hide_possible_values = true, ignore_case = true)]
    pub country: Option<Vec<generated::Country>>,
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
    /// Exclude top-level domain
    #[arg(long, help_heading = "Config")]
    pub no_tld: bool,
    /// Exclude languages
    #[arg(long, help_heading = "Config")]
    pub no_languages: bool,
    /// Exclude currency
    #[arg(long, help_heading = "Config")]
    pub no_currency: bool,
    /// Exclude neighbours
    #[arg(long, help_heading = "Config")]
    pub no_neighbours: bool,
    /// Exclude established date
    #[arg(long, help_heading = "Config")]
    pub no_established_date: bool,
    /// Exclude iso codes
    #[arg(long, help_heading = "Config")]
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
