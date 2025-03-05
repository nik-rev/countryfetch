//! This module is responsible for taking the data that we have in our app, and converting that into
//! a String ready to be printed to the terminal.

use colored::Colorize;
use core::fmt;
use countryfetch::{Country, Location};
use separator::Separatable;
use std::env;

use crate::generated::{self, CurrencyPosition};

type Currency = Option<(generated::CurrencyPosition, Vec<(String, String, String)>)>;

struct CountryOutput<'a> {
    flag: Option<&'a str>,
    flag_emoji: Option<&'a str>,
    country_name: &'a str,
    area_km: Option<f64>,
    area_mi: Option<f64>,
    population: Option<u64>,
    continent: Option<&'a Vec<String>>,
    continent_code: Option<&'a str>,
    top_level_domain: Option<&'a Vec<String>>,
    languages: Option<Vec<String>>,
    currency: Currency,
    neighbours: Option<&'a Vec<String>>,
    established_date: Option<&'static str>,
    dialing_code: Option<String>,
    capital: Option<&'a Vec<String>>,
    driving_side: Option<&'a str>,
    iso_codes: Option<(String, String)>,
    palette: Option<&'static [(u8, u8, u8)]>,
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

    fn format_area(&self) -> String {
        if let (Some(area_km), Some(area_mi)) = (self.area_km, self.area_mi) {
            let km = area_km.separated_string();
            let mi = area_mi.separated_string();
            format!("{}: {km} km ({mi} miles)\n", self.colored("Area"))
        } else {
            "".to_owned()
        }
    }

    fn format_population(&self) -> String {
        if let Some(population) = self.population {
            format!(
                "{}: {} People\n",
                self.colored("Population"),
                population.separated_string()
            )
        } else {
            "".to_owned()
        }
    }

    fn format_capital(&self) -> String {
        if let Some(capital) = self.capital {
            format!(
                "{}: {}\n",
                self.colored(&format!(
                    "Capital{s}",
                    s = if capital.len() == 1 { "" } else { " Cities" }
                )),
                capital.join(", ")
            )
        } else {
            "".to_owned()
        }
    }

    fn format_dialing_code(&self) -> String {
        if let Some(dialing_code) = &self.dialing_code {
            format!("{}: {}\n", self.colored("Dialing code"), dialing_code)
        } else {
            "".to_owned()
        }
    }

    fn format_iso_codes(&self) -> String {
        if let Some(iso_codes) = &self.iso_codes {
            format!(
                "{}: {} / {}\n",
                self.colored("ISO Codes"),
                iso_codes.0,
                iso_codes.1
            )
        } else {
            "".to_owned()
        }
    }

    fn format_driving_side(&self) -> String {
        if let Some(driving_side) = self.driving_side {
            format!("{}: {}\n", self.colored("Driving side"), driving_side)
        } else {
            "".to_owned()
        }
    }

    fn format_currency(&self) -> String {
        if let Some((currency_position, currencies)) = &self.currency {
            let currency_label = self.colored(&format!(
                "Currenc{y}",
                y = if currencies.len() == 1 { "y" } else { "ies" }
            ));

            match currency_position {
                CurrencyPosition::Left => {
                    format!(
                        "{}: {}\n",
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
                        "{}: {}\n",
                        currency_label,
                        currencies
                            .iter()
                            .map(|(id, name, symbol)| format!("{id} {symbol} ({name})"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
        } else {
            "".to_owned()
        }
    }

    fn format_palette(&self) -> String {
        if let Some(palette) = self.palette {
            format!(
                "{}\n",
                palette
                    .iter()
                    .map(|color| format!("{}", "███".truecolor(color.0, color.1, color.2)))
                    .collect::<Vec<_>>()
                    .join("")
            )
        } else {
            "".to_owned()
        }
    }

    fn format_neighbours(&self) -> String {
        if let Some(neighbours) = self.neighbours {
            let neigh = neighbours
                .iter()
                .flat_map(|cc3| {
                    generated::Country::from_country_code(cc3).map(|a| a.country_name())
                })
                .collect::<Vec<_>>()
                .join(", ");

            let neigh_text = if neigh.is_empty() {
                "No neighbours"
            } else {
                &neigh
            };

            format!(
                "{}: {}\n",
                self.colored(&format!(
                    "Neighbour{s}",
                    s = if neighbours.len() == 1 { "" } else { "s" }
                )),
                neigh_text
            )
        } else {
            "".to_owned()
        }
    }

    fn format_continent(&self) -> String {
        if let (Some(continent), continent_code) = (self.continent, self.continent_code) {
            format!(
                "{}: {}{}\n",
                self.colored(&format!(
                    "Continent{s}",
                    s = if continent.len() == 1 { "" } else { "s" }
                )),
                continent.join(", "),
                continent_code
                    .map(|c| format!(" ({c})"))
                    .unwrap_or_default()
            )
        } else {
            "".to_owned()
        }
    }

    fn format_established(&self) -> String {
        if let Some(established_date) = self.established_date {
            format!("{}: {}\n", self.colored("Established"), established_date)
        } else {
            "".to_owned()
        }
    }

    fn format_top_level_domain(&self) -> String {
        if let Some(top_level_domain) = self.top_level_domain {
            format!(
                "{}: {}\n",
                self.colored(&format!(
                    "Top Level Domain{s}",
                    s = if top_level_domain.len() == 1 { "" } else { "s" }
                )),
                top_level_domain.join(", ")
            )
        } else {
            "".to_owned()
        }
    }

    fn format_languages(&self) -> String {
        if let Some(languages) = &self.languages {
            format!(
                "{}: {}\n",
                self.colored(&format!(
                    "Language{s}",
                    s = if languages.len() == 1 { "" } else { "s" }
                )),
                languages.join(", ")
            )
        } else {
            "".to_owned()
        }
    }

    /// Generates the complete country information text
    fn generate_information(&self) -> String {
        let country_name = self.country_name;
        let flag_emoji = self
            .flag_emoji
            .map(|flag| format!(" {flag}"))
            .unwrap_or_default();

        let mut output = format!("{country_name}{flag_emoji}\n-------\n");

        output.push_str(&self.format_area());
        output.push_str(&self.format_area());
        output.push_str(&self.format_continent());
        output.push_str(&self.format_population());
        output.push_str(&self.format_neighbours());
        output.push_str(&self.format_capital());
        output.push_str(&self.format_iso_codes());
        output.push_str(&self.format_driving_side());
        output.push_str(&self.format_dialing_code());
        output.push_str(&self.format_languages());
        output.push_str(&self.format_established());
        output.push_str(&self.format_currency());
        output.push_str(&self.format_top_level_domain());
        output.push('\n');
        output.push_str(&self.format_palette());

        output
    }
}

impl fmt::Display for CountryOutput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let information = self.generate_information();

        if let Some(flag) = self.flag {
            let lines = flag
                .lines()
                .map(|line| format!("  {line}"))
                .zip(information.lines().chain(std::iter::repeat("")));

            for (flag_part, info_part) in lines {
                let extra_space = if info_part.is_empty() { "" } else { "  " };
                writeln!(f, "{flag_part}{extra_space}{info_part}")?;
            }
        } else {
            writeln!(f, "{information}")?;
        }

        Ok(())
    }
}

/// Passing gen_country is required, passing other fields is optional and will further refine the output.
pub fn format_country(
    gen_country: generated::Country,
    country: Option<&Country>,
    location: Option<&Location>,
) -> String {
    let area_km = country.map(|c| c.area_km).unwrap_or(gen_country.area_km());

    // TODO: We don't need to clone and to_string everything, CountryOutput should be able to just be a struct with no owned values.
    CountryOutput {
        flag: true.then_some(if env::var_os("NO_COLOR").is_some() {
            gen_country.flag_nocolor()
        } else {
            gen_country.flag()
        }),
        flag_emoji: true.then_some(
            country
                .map(|c| c.emoji.as_str())
                .unwrap_or(gen_country.emoji()),
        ),
        area_km: true.then_some(country.map(|c| c.area_km).unwrap_or(gen_country.area_km())),
        // rounds to the nearest 100
        area_mi: true.then_some((area_km * 0.62137 * 0.01).round() / 0.01),
        country_name: country
            .map(|c| c.country_name())
            .unwrap_or(gen_country.country_name()),

        continent: true.then_some(
            &country.map(|c| c.continents.clone()).unwrap_or(
                gen_country
                    .continents()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
        ),
        continent_code: location.map(|l| l.continent_code.as_str()).filter(|_| true),
        population: true.then_some(
            country
                .map(|c| c.population)
                .unwrap_or(gen_country.population()),
        ),
        top_level_domain: true.then_some(
            &country.map(|c| c.top_level_domain.clone()).unwrap_or(
                gen_country
                    .top_level_domain()
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>(),
            ),
        ),
        languages: true.then_some(
            country
                .map(|c| c.languages.clone().into_values().collect())
                .unwrap_or(
                    gen_country
                        .languages()
                        .iter()
                        .map(|(_, lang)| lang.to_string())
                        .collect(),
                ),
        ),
        currency: true.then_some((
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
        )),
        neighbours: true.then_some(
            &country.map(|c| c.neighbours.clone()).unwrap_or(
                gen_country
                    .neighbours()
                    .iter()
                    .map(|n| n.to_string())
                    .collect(),
            ),
        ),
        established_date: true.then_some(generated::established_date(gen_country)),
        iso_codes: true.then_some(
            country
                .map(|c| (c.country_code2.clone(), c.country_code3.clone()))
                .unwrap_or((
                    gen_country.country_code2().to_string(),
                    gen_country.country_code3().to_string(),
                )),
        ),
        driving_side: true.then_some(
            country
                .map(|c| c.driving_side())
                .unwrap_or(gen_country.driving_side()),
        ),
        capital: true.then_some(
            &country.map(|c| c.capital.clone()).unwrap_or(
                gen_country
                    .capital()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
        ),
        dialing_code: true.then_some(
            country
                .map(|c| c.dialing_code())
                .unwrap_or(gen_country.dialing_code().to_owned()),
        ),
        palette: true.then_some(gen_country.palette()),
        brightest_color: gen_country.brightest_color(),
    }
    .to_string()
}
