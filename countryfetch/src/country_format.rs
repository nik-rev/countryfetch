//! This module is responsible for taking the data that we have in our app, and
//! converting that into a String ready to be printed to the terminal.
use colored::Colorize as _;
use core::fmt;
use core::fmt::Write as _;
use separator::Separatable as _;
use std::env;

use crate::Args;
use gen_country::CurrencyPosition;

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
    status_note: Option<&'static str>,
    dialing_code: Option<String>,
    capital: Option<&'a Vec<String>>,
    driving_side: Option<&'a str>,
    iso_codes: Option<(String, String)>,
    palette: Option<&'static [(u8, u8, u8)]>,
    brightest_color: (u8, u8, u8),
}

impl CountryOutput<'_> {
    fn format_decimal(value: f64) -> String {
        let rounded = (value * 100.0).round() / 100.0;
        let trimmed = format!("{rounded:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned();

        let (integer, fraction) = trimmed
            .split_once('.')
            .map_or((trimmed.as_str(), None), |(integer, fraction)| {
                (integer, Some(fraction))
            });

        let integer = integer
            .parse::<u64>()
            .expect("formatted decimal keeps a valid positive integer part")
            .separated_string();

        fraction.map_or(integer.clone(), |fraction| format!("{integer}.{fraction}"))
    }

    fn is_blank(value: &str) -> bool {
        value.trim().is_empty()
    }

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
            let km = Self::format_decimal(area_km);
            let mi = Self::format_decimal(area_mi);
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
            if capital.is_empty() {
                return String::new();
            }

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
                if Self::is_blank(dialing_code) {
                    return String::new();
                }

                format!("{}: {}\n", self.colored("Dialing code"), dialing_code)
            })
    }

    fn iso_codes(&self) -> String {
        self.iso_codes
            .as_ref()
            .map_or_else(String::new, |iso_codes| {
                if Self::is_blank(&iso_codes.0) || Self::is_blank(&iso_codes.1) {
                    return String::new();
                }

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
            if currencies.is_empty() {
                return String::new();
            }

            let currency_label = self.colored(&format!(
                "Currenc{y}",
                y = if currencies.len() == 1 { "y" } else { "ies" }
            ));

            let format_currency = |(id, name, symbol): &(String, String, String)| {
                if Self::is_blank(symbol) || symbol == id {
                    format!("{id} ({name})")
                } else {
                    match currency_position {
                        CurrencyPosition::Left => format!("{symbol} {id} ({name})"),
                        CurrencyPosition::Right => format!("{id} {symbol} ({name})"),
                    }
                }
            };

            format!(
                "{}: {}\n",
                currency_label,
                currencies
                    .iter()
                    .map(format_currency)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
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
            if neighbours.is_empty() {
                return String::new();
            }

            let neigh = neighbours
                .iter()
                .filter_map(|cc3| {
                    gen_country::Country::from_country_code(cc3).map(|a| a.country_name())
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
            if continent.is_empty() {
                return String::new();
            }

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

    fn status_note(&self) -> String {
        self.status_note.map_or_else(String::new, |status_note| {
            format!("{}: {}\n", self.colored("Status"), status_note)
        })
    }

    fn top_level_domain(&self) -> String {
        self.top_level_domain
            .map_or_else(String::new, |top_level_domain| {
                if top_level_domain.is_empty() {
                    return String::new();
                }

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
                if languages.is_empty() {
                    return String::new();
                }

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
            .filter(|flag| !Self::is_blank(flag))
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
            &self.status_note(),
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
    gen_country: gen_country::Country,
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
        // rounds to 2 decimal places
        area_mi: (!args.no_area)
            .then_some((area_km * (0.62137_f64.powi(2)) * 100.0).round() / 100.0),
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
            .filter(|_| !args.no_continent),
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
            gen_country::currency_position(gen_country),
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
            .then_some(gen_country::established_date(gen_country)),
        status_note: gen_country::status_note(gen_country),
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

#[cfg(test)]
mod tests {
    use super::format_country;
    use crate::Args;

    fn strip_ansi(input: &str) -> String {
        let mut output = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\u{1b}' && chars.next_if_eq(&'[').is_some() {
                while let Some(next) = chars.next() {
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else {
                output.push(ch);
            }
        }

        output
    }

    fn default_args() -> Args {
        Args {
            country: None,
            all_countries: false,
            list_countries: false,
            no_area: false,
            no_flag: true,
            no_emoji: false,
            no_continent: false,
            no_population: false,
            no_tlds: false,
            no_languages: false,
            no_currencies: false,
            no_neighbours: false,
            no_established_date: false,
            no_iso_codes: false,
            no_driving_side: false,
            no_capital: false,
            no_dialing_code: false,
            no_palette: true,
            no_color: true,
        }
    }

    #[test]
    fn liberland_output_includes_seeded_metadata() {
        let output = format_country(gen_country::Country::Liberland, None, None, &default_args());
        let output = strip_ansi(&output);

        assert!(output.contains("Liberland"));
        assert!(output.contains("Established: April 13, 2015"));
        assert!(output.contains("Status: Self-proclaimed micronation"));
        assert!(output.contains("disputed territory between Croatia and"));
        assert!(output.contains("Area: 7 km² (2.7 miles²)"));
        assert!(output.contains("Population: 63 People"));
        assert!(output.contains("Neighbours: Croatia, Serbia"));
        assert!(output.contains("Language: English"));
        assert!(output.contains("Currency: LLD (Liberland dollar)"));
        assert!(output.contains("ISO Codes: LL / LIB"));
    }

    #[test]
    fn liberland_output_omits_unknown_fields() {
        let output = format_country(gen_country::Country::Liberland, None, None, &default_args());
        let output = strip_ansi(&output);

        assert!(!output.contains("Capital:"));
        assert!(!output.contains("Dialing code:"));
        assert!(!output.contains("Top Level Domain:"));
    }
}
