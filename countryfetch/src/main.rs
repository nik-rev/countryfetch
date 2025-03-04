use core::fmt;
use std::{borrow::BorrowMut, env, io::Read, path::PathBuf};

use colored::Colorize;
use countryfetch::{Country, Location};
use separator::Separatable;

mod generated;

async fn get_data() -> Result<(Location, Country), Box<dyn std::error::Error>> {
    let ip = public_ip::addr().await.unwrap();
    dbg!(ip);

    let client = reqwest::Client::new();

    let location = client.get(format!(
        "http://ip-api.com/json/{ip}?fields=status,message,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting,query"
    )).send().await?.json::<Location>().await?;

    let country = client
        .get(format!(
            "https://restcountries.com/v3.1/alpha/{}",
            location.country_code
        ))
        .send()
        .await?
        .json::<Vec<Country>>()
        .await?
        .into_iter()
        .next()
        .expect("Returns a single country from querying for a Country Code, when there is a valid country code (which would have failed earlier)");

    Ok((location, country))
}

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
    currency: (
        generated::CurrencyPosition,
        Vec<(&'a String, String, String)>,
    ),
    neighbours: &'a Vec<String>,
    established_date: &'static str,
    dialing_code: String,
    capital: &'a Vec<String>,
    driving_side: &'a str,
    iso_codes: (&'a str, &'a str),
    palette: &'static [(u8, u8, u8)],
    brightest_color: (u8, u8, u8),
}

impl fmt::Display for CountryOutput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let country_name = self.country_name;
        let flag_emoji = self.flag_emoji;

        let colored = |s: &str| {
            s.truecolor(
                self.brightest_color.0,
                self.brightest_color.1,
                self.brightest_color.2,
            )
        };

        let km = self.area_km.separated_string();
        let mi = self.area_mi.separated_string();

        let population = format!(
            "{}: {} People",
            colored("Population"),
            self.population.separated_string()
        );

        let area = format!("{}: {km} km ({mi} mi)", colored("Area"));

        let capital = format!(
            "{}: {}",
            colored(&format!(
                "Capital{s}",
                s = if self.capital.len() == 1 {
                    ""
                } else {
                    " Cities"
                }
            )),
            self.capital.join(", "),
        );
        let dialing_code = format!("{}: {}", colored("Dialing code"), self.dialing_code);
        let iso_codes = format!(
            "{}: {} / {}",
            colored("ISO Codes"),
            self.iso_codes.0,
            self.iso_codes.1
        );
        let driving_side = format!("{}: {}", colored("Driving side"), self.driving_side);

        let currency = match self.currency.0 {
            generated::CurrencyPosition::Left => {
                format!(
                    "{}: {}",
                    colored(&format!(
                        "Currenc{y}",
                        y = if self.currency.1.len() == 1 {
                            "y"
                        } else {
                            "ies"
                        }
                    )),
                    self.currency
                        .1
                        .iter()
                        .map(|(id, name, symbol)| format!("{symbol} {id} ({name})"))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            }
            generated::CurrencyPosition::Right => {
                format!(
                    "{}: {}",
                    colored(&format!(
                        "Currenc{y}",
                        y = if self.currency.1.len() == 1 {
                            "y"
                        } else {
                            "ies"
                        }
                    )),
                    self.currency
                        .1
                        .iter()
                        .map(|(id, name, symbol)| format!("{id} {symbol} ({name})"))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            }
        };

        let palette = self
            .palette
            .iter()
            .map(|color| format!("{}", "███".truecolor(color.0, color.1, color.2)))
            .collect::<Vec<_>>()
            .join("");

        let neigh = self
            .neighbours
            .iter()
            .flat_map(|cc3| generated::Country::from_country_code(cc3).map(|a| a.country_name()))
            .collect::<Vec<_>>()
            .join(", ");
        let neigh = if neigh.is_empty() {
            "No neighbours"
        } else {
            &neigh
        };

        let neighbours = format!(
            "{}: {}",
            colored(&format!(
                "Neighbour{s}",
                s = if self.neighbours.len() == 1 { "" } else { "s" }
            )),
            neigh,
        );

        let continent = format!(
            "{}: {}{}",
            colored(&format!(
                "Continent{s}",
                s = if self.continent.len() == 1 { "" } else { "s" }
            )),
            self.continent.join(", "),
            self.continent_code
                .clone()
                .map(|c| format!(" ({c})"))
                .unwrap_or_default(),
        );

        let established = format!("{}: {}", colored("Established"), self.established_date);

        let top_level_domain = format!(
            "{}: {}",
            colored(&format!(
                "Top Level Domain{s}",
                s = if self.top_level_domain.len() == 1 {
                    ""
                } else {
                    "s"
                }
            )),
            self.top_level_domain.join(", "),
        );

        let language = format!(
            "{}: {}",
            colored(&format!(
                "Language{s}",
                s = if self.languages.len() == 1 { "" } else { "s" }
            )),
            self.languages.join(", "),
        );

        let information = format!(
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

{palette}"
        );

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

fn format_country(
    gen_country: generated::Country,
    country: Option<&Country>,
    location: Option<&Location>,
) -> String {
    let area_km = country.map(|c| c.area_km).unwrap_or(gen_country.area_km());
    CountryOutput {
        flag: if env::var_os("NO_COLOR").is_some() {
            gen_country.flag_nocolor()
        } else {
            gen_country.flag()
        },
        flag_emoji: &country
            .map(|c| c.emoji)
            .unwrap_or(gen_country.emoji().to_string()),
        area_km: country.map(|c| c.area_km).unwrap_or(gen_country.area_km()),
        // rounds to the nearest 100
        area_mi: (area_km * 0.62137 * 0.01).round() / 0.01,
        country_name: country
            .map(|c| c.country_name())
            .unwrap_or(&gen_country.country_name()),
        continent: &country.continents,
        continent_code: location.as_deref().map(|l| l.continent_code.clone()),
        population: country
            .map(|c| c.population)
            .unwrap_or(gen_country.population()),
        top_level_domain: &country.map(|c| c.top_level_domain).unwrap_or(
            gen_country
                .top_level_domain()
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>(),
        ),
        languages: country.languages.values().cloned().collect(),
        currency: (
            generated::currency_position(gen_country),
            country
                .map(|c| {
                    c.currencies
                        .iter()
                        .map(|(currency_id, currency)| {
                            (currency_id, currency.name.clone(), currency.symbol.clone())
                        })
                        .collect()
                })
                .unwrap_or(
                    gen_country
                        .currencies()
                        .iter()
                        .map(|c| (&c.0.to_string(), c.1.to_string(), c.2.to_string()))
                        .collect(),
                ),
        ),
        neighbours: &country.map(|c| c.neighbours).unwrap_or(
            gen_country
                .neighbours()
                .iter()
                .map(|n| n.to_string())
                .collect(),
        ),
        established_date: generated::established_date(gen_country),
        iso_codes: (&country.country_code2, &country.country_code3),
        driving_side: country.driving_side(),
        capital: &country.capital,
        dialing_code: country.dialing_code(),
        palette: count.palette(),
        brightest_color: count.brightest_color(),
    }
    .to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let (location, country) = get_data().await.unwrap();

    let mut country_json =
        std::fs::File::open(PathBuf::from("../../xtask/countries.json")).unwrap();

    let mut buf = String::new();

    country_json.read_to_string(&mut buf).unwrap();

    let countries = serde_json::de::from_str::<Vec<Country>>(&buf).unwrap();

    for country in countries {
        let country = format_country(country, None);

        println!("{country}");
    }

    // let country_cached_data = generated::Country::from_country_code(&country.country_code3)
    //     .expect("All countries have been cached");

    // dbg!(location, country);

    Ok(())
}
