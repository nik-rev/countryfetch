//! This module is responsible for taking the data that we have in our app, and
//! converting that into a String ready to be printed to the terminal.
use core::fmt;
use core::fmt::Write as _;
use std::env;

use colored::Colorize as _;
use separator::Separatable as _;

use crate::extra_country_data::{self, CurrencyPosition};
use crate::{Args, generated_country_data};

type Currency = Option<(CurrencyPosition, Vec<(String, String, String)>)>;

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

    fn area(&self) -> String {
        if let (Some(area_km), Some(area_mi)) = (self.area_km, self.area_mi) {
            let km = area_km.separated_string();
            let mi = area_mi.separated_string();
            format!("{}: {km} km² ({mi} miles²)\n", self.colored("Area"))
        } else {
            String::new()
        }
    }

    fn population(&self) -> String {
        self.population.map_or_else(String::new, |population| {
            format!(
                "{}: {} People\n",
                self.colored("Population"),
                population.separated_string()
            )
        })
    }

    fn capital(&self) -> String {
        self.capital.map_or_else(String::new, |capital| {
            format!(
                "{}: {}\n",
                self.colored(&format!(
                    "Capital{s}",
                    s = if capital.len() == 1 { "" } else { " Cities" }
                )),
                capital.join(", ")
            )
        })
    }

    fn dialing_code(&self) -> String {
        self.dialing_code
            .as_ref()
            .map_or_else(String::new, |dialing_code| {
                format!("{}: {}\n", self.colored("Dialing code"), dialing_code)
            })
    }

    fn iso_codes(&self) -> String {
        self.iso_codes
            .as_ref()
            .map_or_else(String::new, |iso_codes| {
                format!(
                    "{}: {} / {}\n",
                    self.colored("ISO Codes"),
                    iso_codes.0,
                    iso_codes.1
                )
            })
    }

    fn driving_side(&self) -> String {
        self.driving_side.map_or_else(String::new, |driving_side| {
            format!("{}: {}\n", self.colored("Driving side"), driving_side)
        })
    }

    fn currency(&self) -> String {
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
                },
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
                },
            }
        } else {
            String::new()
        }
    }

    fn palette(&self) -> String {
        self.palette.map_or_else(String::new, |palette| {
            format!(
                "\n{}\n",
                palette.iter().fold(String::new(), |mut output, color| {
                    let _ = write!(output, "{}", "███".truecolor(color.0, color.1, color.2));
                    output
                })
            )
        })
    }

    fn neighbours(&self) -> String {
        self.neighbours.map_or_else(String::new, |neighbours| {
            let neigh = neighbours
                .iter()
                .filter_map(|cc3| {
                    generated_country_data::Country::from_country_code(cc3)
                        .map(|a| a.country_name())
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
        })
    }

    fn continent(&self) -> String {
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
            String::new()
        }
    }

    fn established_date(&self) -> String {
        self.established_date
            .map_or_else(String::new, |established_date| {
                format!("{}: {}\n", self.colored("Established"), established_date)
            })
    }

    fn top_level_domain(&self) -> String {
        self.top_level_domain
            .map_or_else(String::new, |top_level_domain| {
                format!(
                    "{}: {}\n",
                    self.colored(&format!(
                        "Top Level Domain{s}",
                        s = if top_level_domain.len() == 1 { "" } else { "s" }
                    )),
                    top_level_domain.join(", ")
                )
            })
    }

    fn languages(&self) -> String {
        self.languages
            .as_ref()
            .map_or_else(String::new, |languages| {
                format!(
                    "{}: {}\n",
                    self.colored(&format!(
                        "Language{s}",
                        s = if languages.len() == 1 { "" } else { "s" }
                    )),
                    languages.join(", ")
                )
            })
    }

    fn flag(&self) -> String {
        self.flag_emoji
            .map(|flag| format!(" {flag}"))
            .unwrap_or_default()
    }

    fn separator() -> &'static str {
        "\n-------\n"
    }

    /// Generates the complete country information text
    fn generate_information(&self) -> String {
        let mut output = String::new();

        for part in [
            self.country_name,
            &self.flag(),
            Self::separator(),
            &self.area(),
            &self.continent(),
            &self.population(),
            &self.neighbours(),
            &self.capital(),
            &self.iso_codes(),
            &self.driving_side(),
            &self.dialing_code(),
            &self.languages(),
            &self.established_date(),
            &self.currency(),
            &self.top_level_domain(),
            &self.palette(),
        ] {
            output.push_str(part);
        }

        textwrap::wrap(&output, 40).join("\n")
    }
}

impl fmt::Display for CountryOutput<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let information = self.generate_information();

        if let Some(flag) = self.flag {
            let lines = flag
                .lines()
                .map(|line| format!("  {line}"))
                .zip(information.lines().chain(core::iter::repeat("")));

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

/// Passing `gen_country` is required, passing other fields is optional and will
/// further refine the output.
pub fn format_country(
    gen_country: generated_country_data::Country,
    country: Option<&crate::Country>,
    location: Option<&crate::Location>,
    args: &Args,
) -> String {
    let area_km = country.map_or(gen_country.area_km(), |c| c.area_km);

    // PERF: We don't need to clone and to_string everything, CountryOutput should
    // be able to just be a struct with no owned values.
    CountryOutput {
        flag: (!args.no_flag).then_some(if env::var_os("NO_COLOR").is_some() {
            gen_country.flag_no_color()
        } else {
            gen_country.flag()
        }),
        flag_emoji: (!args.no_emoji)
            .then_some(country.map_or(gen_country.emoji(), |c| c.emoji.as_str())),
        area_km: (!args.no_area).then_some(country.map_or(gen_country.area_km(), |c| c.area_km)),
        // rounds to the nearest 100
        area_mi: (!args.no_area).then_some((area_km * (0.62137_f64.powi(2)) * 0.01).round() / 0.01),
        country_name: country.map_or(
            gen_country.country_name(),
            super::country::Country::country_name,
        ),

        continent: (!args.no_continent).then_some(&country.map_or_else(
            || {
                gen_country
                    .continents()
                    .iter()
                    .map(|s| (*s).to_owned())
                    .collect()
            },
            |c| c.continents.clone(),
        )),
        continent_code: location
            .map(|l| l.continent_code.as_str())
            .filter(|_| (!args.no_continent)),
        population: (!args.no_population)
            .then_some(country.map_or(gen_country.population(), |c| c.population)),
        top_level_domain: (!args.no_tlds).then_some(&country.map_or_else(
            || {
                gen_country
                    .top_level_domain()
                    .iter()
                    .map(|a| (*a).to_owned())
                    .collect::<Vec<_>>()
            },
            |c| c.top_level_domain.clone(),
        )),
        languages: (!args.no_languages).then_some(country.map_or_else(
            || {
                gen_country
                    .languages()
                    .iter()
                    .map(|(_, lang)| (*lang).to_owned())
                    .collect()
            },
            |c| c.languages.clone().into_values().collect(),
        )),
        currency: (!args.no_currencies).then_some((
            extra_country_data::currency_position(gen_country),
            country.map_or_else(
                || {
                    gen_country
                        .currencies()
                        .iter()
                        .map(|c| (c.0.to_owned(), c.1.to_owned(), c.2.to_owned()))
                        .collect()
                },
                |c| {
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
                },
            ),
        )),
        neighbours: (!args.no_neighbours).then_some(&country.map_or_else(
            || {
                gen_country
                    .neighbours()
                    .iter()
                    .map(|n| (*n).to_owned())
                    .collect()
            },
            |c| c.neighbours.clone(),
        )),
        established_date: (!args.no_established_date)
            .then(|| extra_country_data::established_date(gen_country))
            .flatten(),
        iso_codes: (!args.no_iso_codes).then_some(country.map_or_else(
            || {
                (
                    gen_country.country_code2().to_owned(),
                    gen_country.country_code3().to_owned(),
                )
            },
            |c| (c.country_code2.clone(), c.country_code3.clone()),
        )),
        driving_side: (!args.no_driving_side).then_some(country.map_or(
            gen_country.driving_side(),
            super::country::Country::driving_side,
        )),
        capital: (!args.no_capital).then_some(&country.map_or_else(
            || {
                gen_country
                    .capital()
                    .iter()
                    .map(|s| (*s).to_owned())
                    .collect()
            },
            |c| c.capital.clone(),
        )),
        dialing_code: (!args.no_dialing_code).then_some(country.map_or_else(
            || gen_country.dialing_code().to_owned(),
            super::country::Country::dialing_code,
        )),
        palette: (!args.no_palette).then_some(gen_country.palette()),
        brightest_color: gen_country.brightest_color(),
    }
    .to_string()
}
