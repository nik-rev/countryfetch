//! This module is responsible for taking the data that we have in our app, and
//! converting that into a String ready to be printed to the terminal.
use core::fmt;
use core::fmt::Write as _;
use std::env;

use colored::Colorize as _;
use separator::Separatable as _;

use crate::Cli;
use crate::countries::Country;
use crate::extra_country_data::CurrencyPosition;

/// TODO: This would be modelled better as a `struct`
///
/// `(Left | Right, List<3-letter name, Name of currency, Symbol>)`
type CurrencyData = Option<(CurrencyPosition, Vec<(String, String, String)>)>;

/// This describes the
struct CountryOutput<'a> {
    /// The country's flag (ASCII colored or plain)
    flag: Option<&'a str>,
    /// Emoji of the country's flag
    flag_emoji: Option<&'a str>, // NOTE: New API doesn't explicitly have 'flag_emoji', but we'll keep the field for compatibility if it comes from `crate::Country` or a lookup. Keeping the logic simple, assuming it's available or we can use the main country name.
    /// Name of the country (common name)
    country_name: &'a str,
    /// Area in KM
    area_km: Option<f64>,
    /// Area in miles
    area_mi: Option<f64>,
    /// How many people live in this country
    population: Option<u64>,
    /// A list of continents that the country is on
    continent: Option<&'a Vec<String>>,
    /// TLD like .uk
    top_level_domain: Option<&'a Vec<String>>,
    /// A list of languages spoken in the country
    languages: Option<Vec<String>>,
    /// Currency position and data
    currency: CurrencyData,
    /// Countries that are physically neighbours to this country (borders)
    neighbours: Option<&'a Vec<String>>,
    /// Established date. (Removed, as it was tied to 'extra_country_data')
    established_date: Option<&'static str>,
    /// Dialing code
    dialing_code: Option<String>,
    /// A country can have several capital cities
    capital: Option<&'a Vec<String>>,
    /// Left or Right driving side.
    driving_side: Option<&'a str>,
    /// (2-letter, 3-letter) ISO codes
    iso_codes: Option<(String, String)>,
    /// A list of brightest colours from the country's flag
    palette: Option<&'a [(u8, u8, u8)]>,
    /// (R, G, B)
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

    /// Render the country's area
    fn area(&self) -> String {
        if let (Some(area_km), Some(area_mi)) = (self.area_km, self.area_mi) {
            let km = format!("{:.0}", area_km.separated_string());
            let mi = format!("{:.0}", area_mi.separated_string());
            format!("{}: {km} km² ({mi} miles²)\n", self.colored("Area"))
        } else {
            String::new()
        }
    }

    /// Render the country's population
    fn population(&self) -> String {
        self.population.map_or_else(String::new, |population| {
            format!(
                "{}: {} People\n",
                self.colored("Population"),
                population.separated_string()
            )
        })
    }

    /// Render the country's capital
    fn capital(&self) -> String {
        self.capital.map_or_else(String::new, |capital| {
            if capital.is_empty() {
                String::new()
            } else {
                format!(
                    "{}: {}\n",
                    self.colored(&format!(
                        "Capital{s}",
                        s = if capital.len() == 1 { "" } else { " Cities" }
                    )),
                    capital.join(", ")
                )
            }
        })
    }

    /// Render the dialing code of the country
    fn dialing_code(&self) -> String {
        self.dialing_code
            .as_ref()
            .map_or_else(String::new, |dialing_code| {
                format!("{}: {}\n", self.colored("Dialing code"), dialing_code)
            })
    }

    /// Render the ISO Codes
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

    /// Render the driving side of the country
    fn driving_side(&self) -> String {
        self.driving_side.map_or_else(String::new, |driving_side| {
            format!("{}: {}\n", self.colored("Driving side"), driving_side)
        })
    }

    /// Render the currency of the country
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
                            .map(|(id, name, symbol)| {
                                format!(
                                    "{symbol} {id} ({name})",
                                    symbol = symbol.as_str(), // Symbol is now a String or Option<String>
                                )
                            })
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
                            .map(|(id, name, symbol)| {
                                format!("{id} {symbol} ({name})", symbol = symbol.as_str())
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
        } else {
            String::new()
        }
    }

    /// Render colors of the country's flag
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

    /// Render neighbours of the country
    fn neighbours(&self) -> String {
        self.neighbours.map_or_else(String::new, |neighbours| {
            // Note: The original logic for resolving 3-letter codes to full names
            // is removed because `generated_country_data` is gone. We'll output
            // the 3-letter codes directly or filter if empty.
            let neigh = neighbours.join(", ");

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

    /// Render the continent
    fn continent(&self) -> String {
        if let Some(continent) = self.continent {
            format!(
                "{}: {}\n",
                self.colored(&format!(
                    "Continent{s}",
                    s = if continent.len() == 1 { "" } else { "s" }
                )),
                continent.join(", ")
            )
        } else {
            String::new()
        }
    }

    /// Render the established date
    fn established_date(&self) -> String {
        // This is now always empty since `extra_country_data` is removed
        self.established_date
            .map_or_else(String::new, |established_date| {
                format!("{}: {}\n", self.colored("Established"), established_date)
            })
    }

    /// Render the TLD
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

    /// Render languages
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

    /// Render the emoji flag
    fn flag(&self) -> String {
        self.flag_emoji
            .map(|flag| format!(" {flag}"))
            .unwrap_or_default()
    }

    /// Render a separator
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Passing `country` is required. The `location` and `old_country` parameters
/// from the previous API are now considered obsolete or are replaced by fields
/// directly on the new `Country` struct.
pub fn format_country(country: &'static Country, args: &Cli) -> String {
    let area_km = country.area;

    // The new API doesn't provide a direct emoji flag, so we'll check if the
    // flag field (which is a string, often the emoji) exists. If not, it's None.
    let flag_emoji: Option<&str> = (!args.no_emoji)
        .then_some(country.flag.as_deref())
        .flatten();

    // The new API provides flag_ascii_plain and flag_ascii_colored
    let flag_ascii = if env::var_os("NO_COLOR").is_some() {
        &country.flag_ascii_plain
    } else {
        &country.flag_ascii_colored
    };

    let dialing_code = if let Some(idd) = &country.idd {
        if let (Some(root), Some(suffixes)) = (&idd.root, &idd.suffixes) {
            // Simplification: Join the root and the first suffix.
            Some(format!(
                "{}{}",
                root,
                suffixes.first().unwrap_or(&"".to_string())
            ))
        } else {
            None
        }
    } else {
        None
    };

    let currency_data = (!args.no_currencies).then(|| {
        let position = crate::extra_country_data::currency_position(country.kind());

        let currencies: Vec<(String, String, String)> = country
            .currencies
            .as_ref()
            .map(|map| {
                map.iter()
                    .map(|(id, currency)| {
                        (
                            id.clone(),
                            currency.name.clone(),
                            currency.symbol.clone().unwrap_or_default(), // Symbol can be Option<String>
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();

        (position, currencies)
    });

    // NOTE: This assumes a static brightest color is computed and added to the
    // Country struct, which is not in the JSON but *is* in the `flag_palette`
    // from the user's struct definition. We will take the first color from the
    // palette for simplicity.
    let brightest_color = country.flag_palette.first().copied().unwrap_or((0, 0, 0));

    // PERF: We don't need to clone and to_string everything, CountryOutput should
    // be able to just be a struct with no owned values.
    CountryOutput {
        flag: (!args.no_flag).then_some(flag_ascii),
        flag_emoji,
        area_km: (!args.no_area).then_some(area_km),
        // rounds to the nearest 100
        area_mi: (!args.no_area).then_some((area_km * (0.62137_f64.powi(2)) * 0.01).round() / 0.01),
        country_name: &country.name.common,

        continent: (!args.no_continent).then_some(&country.continents),
        population: (!args.no_population).then_some(country.population),
        top_level_domain: (!args.no_tlds).then_some(&country.tld),

        languages: (!args.no_languages).then_some(
            country
                .languages
                .as_ref()
                .map(|langs| langs.values().cloned().collect())
                .unwrap_or_default(),
        ),

        currency: currency_data,

        neighbours: (!args.no_neighbours).then_some(&country.borders),

        established_date: None, // Established date is removed

        iso_codes: (!args.no_iso_codes).then_some((country.cca2.clone(), country.cca3.clone())),

        driving_side: (!args.no_driving_side).then_some(country.car.side.as_str()),

        capital: (!args.no_capital)
            .then_some(country.capital.as_ref())
            .flatten(),

        dialing_code: (!args.no_dialing_code).then_some(dialing_code).flatten(),

        palette: (!args.no_palette).then_some(&country.flag_palette),

        brightest_color,
    }
    .to_string()
}
