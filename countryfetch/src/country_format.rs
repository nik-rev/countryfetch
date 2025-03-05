//! This module is responsible for taking the data that we have in our app, and converting that into
//! a String ready to be printed to the terminal.

use colored::Colorize;
use core::fmt;
use countryfetch::{Country, Location};
use separator::Separatable;
use std::env;

use crate::generated::{self, CurrencyPosition};

struct CountryOutput<'a> {
    flag: &'a str,
    flag_emoji: &'a str,
    country_name: &'a str,
    area_km: f64,
    area_mi: f64,
    population: u64,
    continent: &'a Vec<String>,
    continent_code: Option<String>,
    top_level_domain: &'a Vec<String>,
    languages: Vec<String>,
    currency: (generated::CurrencyPosition, Vec<(String, String, String)>),
    neighbours: &'a Vec<String>,
    established_date: &'static str,
    dialing_code: String,
    capital: &'a Vec<String>,
    driving_side: &'a str,
    iso_codes: (String, String),
    palette: &'static [(u8, u8, u8)],
    brightest_color: (u8, u8, u8),
}

impl CountryOutput<'_> {
    /// Applies the country's brightest color to the given text
    fn colored(&self, s: &str) -> colored::ColoredString {
        s.truecolor(
            self.brightest_color.0,
            self.brightest_color.1,
            self.brightest_color.2,
        )
    }

    /// Formats the area information
    fn format_area(&self) -> String {
        let km = self.area_km.separated_string();
        let mi = self.area_mi.separated_string();
        format!("{}: {km} km ({mi} mi)", self.colored("Area"))
    }

    /// Formats the population information
    fn format_population(&self) -> String {
        format!(
            "{}: {} People",
            self.colored("Population"),
            self.population.separated_string()
        )
    }

    /// Formats the capital information
    fn format_capital(&self) -> String {
        format!(
            "{}: {}",
            self.colored(&format!(
                "Capital{s}",
                s = if self.capital.len() == 1 {
                    ""
                } else {
                    " Cities"
                }
            )),
            self.capital.join(", ")
        )
    }

    /// Formats the dialing code
    fn format_dialing_code(&self) -> String {
        format!("{}: {}", self.colored("Dialing code"), self.dialing_code)
    }

    /// Formats the ISO codes
    fn format_iso_codes(&self) -> String {
        format!(
            "{}: {} / {}",
            self.colored("ISO Codes"),
            self.iso_codes.0,
            self.iso_codes.1
        )
    }

    /// Formats the driving side
    fn format_driving_side(&self) -> String {
        format!("{}: {}", self.colored("Driving side"), self.driving_side)
    }

    /// Formats the currency information
    fn format_currency(&self) -> String {
        let (currency_position, currencies) = &self.currency;

        let currency_label = self.colored(&format!(
            "Currenc{y}",
            y = if currencies.len() == 1 { "y" } else { "ies" }
        ));

        match currency_position {
            CurrencyPosition::Left => {
                format!(
                    "{}: {}",
                    currency_label,
                    currencies
                        .iter()
                        .map(|(id, name, symbol)| format!("{symbol} {id} ({name})"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            CurrencyPosition::Right => {
                format!(
                    "{}: {}",
                    currency_label,
                    currencies
                        .iter()
                        .map(|(id, name, symbol)| format!("{id} {symbol} ({name})"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }

    /// Formats the color palette
    fn format_palette(&self) -> String {
        self.palette
            .iter()
            .map(|color| format!("{}", "███".truecolor(color.0, color.1, color.2)))
            .collect::<Vec<_>>()
            .join("")
    }

    /// Formats the neighbours information
    fn format_neighbours(&self) -> String {
        let neigh = self
            .neighbours
            .iter()
            .flat_map(|cc3| generated::Country::from_country_code(cc3).map(|a| a.country_name()))
            .collect::<Vec<_>>()
            .join(", ");

        let neigh_text = if neigh.is_empty() {
            "No neighbours"
        } else {
            &neigh
        };

        format!(
            "{}: {}",
            self.colored(&format!(
                "Neighbour{s}",
                s = if self.neighbours.len() == 1 { "" } else { "s" }
            )),
            neigh_text
        )
    }

    /// Formats the continent information
    fn format_continent(&self) -> String {
        format!(
            "{}: {}{}",
            self.colored(&format!(
                "Continent{s}",
                s = if self.continent.len() == 1 { "" } else { "s" }
            )),
            self.continent.join(", "),
            self.continent_code
                .clone()
                .map(|c| format!(" ({c})"))
                .unwrap_or_default()
        )
    }

    /// Formats the establishment date
    fn format_established(&self) -> String {
        format!("{}: {}", self.colored("Established"), self.established_date)
    }

    /// Formats the top-level domain information
    fn format_top_level_domain(&self) -> String {
        format!(
            "{}: {}",
            self.colored(&format!(
                "Top Level Domain{s}",
                s = if self.top_level_domain.len() == 1 {
                    ""
                } else {
                    "s"
                }
            )),
            self.top_level_domain.join(", ")
        )
    }

    /// Formats the languages
    fn format_languages(&self) -> String {
        format!(
            "{}: {}",
            self.colored(&format!(
                "Language{s}",
                s = if self.languages.len() == 1 { "" } else { "s" }
            )),
            self.languages.join(", ")
        )
    }

    /// Generates the complete country information text
    fn generate_information(&self) -> String {
        let country_name = self.country_name;
        let flag_emoji = self.flag_emoji;

        format!(
            "\
{country_name} {flag_emoji}
-------
{area}
{continent}
{population}
{neighbours}
{capital}
{iso_codes}
{driving_side}
{dialing_code}
{language}
{established}
{currency}
{top_level_domain}

{palette}",
            area = self.format_area(),
            continent = self.format_continent(),
            population = self.format_population(),
            neighbours = self.format_neighbours(),
            capital = self.format_capital(),
            iso_codes = self.format_iso_codes(),
            driving_side = self.format_driving_side(),
            dialing_code = self.format_dialing_code(),
            language = self.format_languages(),
            established = self.format_established(),
            currency = self.format_currency(),
            top_level_domain = self.format_top_level_domain(),
            palette = self.format_palette()
        )
    }
}

impl fmt::Display for CountryOutput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let information = self.generate_information();

        let lines = self
            .flag
            .lines()
            .map(|line| format!("  {line}"))
            .zip(information.lines().chain(std::iter::repeat("")));

        for (flag_part, info_part) in lines {
            let extra_space = if info_part.is_empty() { "" } else { "  " };
            writeln!(f, "{flag_part}{extra_space}{info_part}")?;
        }

        Ok(())
    }
}

pub fn format_country(
    gen_country: generated::Country,
    country: Option<&Country>,
    location: Option<&Location>,
) -> String {
    let area_km = country.map(|c| c.area_km).unwrap_or(gen_country.area_km());

    // TODO: We don't need to clone and to_string everything, CountryOutput should be able to just be a struct with no owned values.
    CountryOutput {
        flag: if env::var_os("NO_COLOR").is_some() {
            gen_country.flag_nocolor()
        } else {
            gen_country.flag()
        },
        flag_emoji: &country
            .map(|c| c.emoji.clone())
            .unwrap_or(gen_country.emoji().to_string()),
        area_km: country.map(|c| c.area_km).unwrap_or(gen_country.area_km()),
        // rounds to the nearest 100
        area_mi: (area_km * 0.62137 * 0.01).round() / 0.01,
        country_name: country
            .map(|c| c.country_name())
            .unwrap_or(gen_country.country_name()),
        continent: &country.map(|c| c.continents.clone()).unwrap_or(
            gen_country
                .continents()
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ),
        continent_code: location.map(|l| l.continent_code.clone()),
        population: country
            .map(|c| c.population)
            .unwrap_or(gen_country.population()),
        top_level_domain: &country.map(|c| c.top_level_domain.clone()).unwrap_or(
            gen_country
                .top_level_domain()
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>(),
        ),
        languages: country
            .map(|c| c.languages.clone().into_values().collect())
            .unwrap_or(
                gen_country
                    .languages()
                    .iter()
                    .map(|(_, lang)| lang.to_string())
                    .collect(),
            ),
        currency: (
            generated::currency_position(gen_country),
            country
                .map(|c| {
                    c.currencies
                        .iter()
                        .map(|(currency_id, currency)| {
                            (
                                currency_id.to_string(),
                                currency.name.clone(),
                                currency.symbol.clone(),
                            )
                        })
                        .collect()
                })
                .unwrap_or(
                    gen_country
                        .currencies()
                        .iter()
                        .map(|c| (c.0.to_string(), c.1.to_string(), c.2.to_string()))
                        .collect(),
                ),
        ),
        neighbours: &country.map(|c| c.neighbours.clone()).unwrap_or(
            gen_country
                .neighbours()
                .iter()
                .map(|n| n.to_string())
                .collect(),
        ),
        established_date: generated::established_date(gen_country),
        iso_codes: country
            .map(|c| (c.country_code2.clone(), c.country_code3.clone()))
            .unwrap_or((
                gen_country.country_code2().to_string(),
                gen_country.country_code3().to_string(),
            )),
        driving_side: country
            .map(|c| c.driving_side())
            .unwrap_or(gen_country.driving_side()),
        capital: &country.map(|c| c.capital.clone()).unwrap_or(
            gen_country
                .capital()
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ),
        dialing_code: country
            .map(|c| c.dialing_code())
            .unwrap_or(gen_country.dialing_code().to_string()),
        palette: gen_country.palette(),
        brightest_color: gen_country.brightest_color(),
    }
    .to_string()
}
