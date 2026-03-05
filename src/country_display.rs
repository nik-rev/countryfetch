use core::fmt;
use std::fmt::Write as _;

use colored::Colorize as _;
use docstr::docstr;
use separator::Separatable as _;

use crate::countries::Country;
use crate::extra_country_data::CurrencyPosition;

/// Gets the brightest color that should be used for a country
pub fn brightest_color(country: &Country) -> (u8, u8, u8) {
    // NOTE: This assumes a static brightest color is computed and added to the
    // Country struct, which is not in the JSON but *is* in the `flag_palette`
    // from the user's struct definition. We will take the first color from the
    // palette for simplicity.
    country.flag_palette.first().copied().unwrap_or((0, 0, 0))
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let area_km = self.area;

        // The new API doesn't provide a direct emoji flag, so we'll check if the
        // flag field (which is a string, often the emoji) exists. If not, it's None.
        let flag_emoji: Option<&str> = self.flag.as_deref();

        // The new API provides flag_ascii_plain and flag_ascii_colored
        let flag_ascii = &self.flag_ascii_colored;

        let dialing_code = if let Some(idd) = &self.idd {
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

        let currency_data = {
            let position = crate::extra_country_data::currency_position(self.kind());

            let currencies: Vec<(String, String, String)> = self
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
        };

        let brightest_color = brightest_color(self);

        let c_flag = flag_ascii;
        let c_flag_emoji = flag_emoji;
        let c_area_km = area_km;
        // rounds to the nearest 100
        let c_area_mi = (area_km * (0.62137_f64.powi(2)) * 0.01).round() / 0.01;
        let c_country_name = &self.name.common;

        let c_continent = &self.continents;
        let c_population = self.population;
        let c_top_level_domain = &self.tld;

        let c_languages: Vec<_> = self
            .languages
            .as_ref()
            .map(|langs| langs.values().cloned().collect())
            .unwrap_or_default();

        let c_currency = currency_data;

        let c_neighbours = &self.borders;

        let c_iso_codes = (self.cca2.clone(), self.cca3.clone());

        let c_driving_side = self.car.side.as_str();

        let c_capital = self.capital.as_ref();
        let c_dialing_code = dialing_code;

        let c_palette = &self.flag_palette;

        let (r, g, b) = brightest_color;

        let km = format!("{:.0}", c_area_km.separated_string());
        let mi = format!("{:.0}", c_area_mi.separated_string());

        let c = crate::rgb(r, g, b);
        let r = simply_colored::RESET;
        let country_name = c_country_name;
        let flag_emoji = c_flag_emoji
            .map(|flag| format!(" {flag}"))
            .unwrap_or_default();
        let continents_s = if c_continent.len() == 1 { "" } else { "s" };
        let continents = c_continent.join(", ");
        let people_count = c_population.separated_string();

        let neigh = c_neighbours.join(", ");

        let neigh_text = if neigh.is_empty() {
            "No neighbours"
        } else {
            &neigh
        };

        let neighbours_s = if c_neighbours.len() == 1 { "" } else { "s" };

        let capital =
            c_capital
                .filter(|capital| !capital.is_empty())
                .map_or_else(String::new, |capital| {
                    format!(
                        "\n{c}Capital{}{r}: {}",
                        if capital.len() == 1 { "" } else { " Cities" },
                        capital.join(", ")
                    )
                });

        let iso_code_1 = &c_iso_codes.0;
        let iso_code_2 = &c_iso_codes.1;
        let driving_side = c_driving_side;

        let dialing_code = c_dialing_code
            .as_ref()
            .map_or_else(String::new, |dialing_code| {
                format!("\n{c}Dialing code{r}: {}", dialing_code)
            });

        let languages_s = if c_languages.len() == 1 { "" } else { "s" };
        let languages = c_languages.join(", ");

        let (currency_position, currencies) = &c_currency;
        let currency_ies = if currencies.len() == 1 { "y" } else { "ies" };

        let currencies = match currency_position {
            CurrencyPosition::Left => {
                currencies
                    .iter()
                    .map(|(id, name, symbol)| {
                        format!("{symbol} {id} ({name})", symbol = symbol.as_str(),)
                    })
                    .collect::<Vec<_>>()
            }
            CurrencyPosition::Right => {
                currencies
                    .iter()
                    .map(|(id, name, symbol)| {
                        format!("{id} {symbol} ({name})", symbol = symbol.as_str())
                    })
                    .collect::<Vec<_>>()
            }
        };
        let currencies = currencies.join(", ");

        let top_level_domain_suffix = if c_top_level_domain.len() == 1 {
            ""
        } else {
            "s"
        };
        let top_level_domain = c_top_level_domain.join(", ");

        let palette = c_palette.iter().fold(String::new(), |mut output, color| {
            let _ = write!(output, "{}", "███".truecolor(color.0, color.1, color.2));
            output
        });

        let output = docstr!(format!
            /// {country_name}{flag_emoji}
            /// -------
            /// {c}Area{r}: {km} km² ({mi} miles²)
            /// {c}Continent{continents_s}{r}: {continents}
            /// {c}Population{r}: {people_count} People
            /// {c}Neighbour{neighbours_s}{r}: {neigh_text}{capital}
            /// {c}ISO Codes{r}: {iso_code_1} / {iso_code_2}
            /// {c}Driving side{r}: {driving_side}{dialing_code}
            /// {c}Language{languages_s}{r}: {languages}
            /// {c}Currenc{currency_ies}{r}: {currencies}
            /// {c}Top Level Domain{top_level_domain_suffix}{r}: {top_level_domain}
            ///
            /// {palette}
        );

        let output = textwrap::wrap(&output, 40).join("\n");

        let flag_line_len = self
            .flag_ascii_plain
            .lines()
            .next()
            .unwrap_or_default()
            .len();

        let lines = c_flag
            .lines()
            .map(|line| format!("  {line}"))
            .chain(core::iter::repeat_with(|| {
                format!("  {}", " ".repeat(flag_line_len))
            }))
            .zip(output.lines().chain(core::iter::repeat("")))
            .take(c_flag.lines().count().max(output.lines().count()));

        for (flag_part, info_part) in lines {
            let extra_space = if info_part.is_empty() { "" } else { "  " };
            writeln!(f, "{flag_part}{extra_space}{info_part}")?;
        }

        Ok(())
    }
}
