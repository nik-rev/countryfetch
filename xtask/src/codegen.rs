use core::fmt;

use crate::{
    Country,
    country_parts::{self, CountryParts},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum::IntoEnumIterator;

#[derive(strum::EnumIter, strum::Display)]
#[strum(serialize_all = "snake_case")]
enum CountryMethod {
    // takes &self
    Description,
    CountryName,
    CountryCode3,
    CountryCode2,
    TopLevelDomain,
    Currencies,
    Languages,
    Neighbours,
    Capital,
    Palette,
    AreaKm,
    DialingCode,
    DrivingSide,
    Emoji,
    BrightestColor,
    Population,
    Continents,
    Flag,
    FlagNoColor,
    // takes &str
    FromStr,
    FromCountryCode,
    CountryCode3FromCountryCode2,
}

impl CountryMethod {
    pub fn param(&self) -> (&'static str, &'static str) {
        match self {
            CountryMethod::Description
            | CountryMethod::CountryName
            | CountryMethod::CountryCode3
            | CountryMethod::CountryCode2
            | CountryMethod::TopLevelDomain
            | CountryMethod::Currencies
            | CountryMethod::Languages
            | CountryMethod::Neighbours
            | CountryMethod::Capital
            | CountryMethod::Palette
            | CountryMethod::AreaKm
            | CountryMethod::DialingCode
            | CountryMethod::DrivingSide
            | CountryMethod::Emoji
            | CountryMethod::BrightestColor
            | CountryMethod::Population
            | CountryMethod::Flag
            | CountryMethod::FlagNoColor
            | CountryMethod::Continents => ("self", "&Self"),
            CountryMethod::FromStr
            | CountryMethod::FromCountryCode
            | CountryMethod::CountryCode3FromCountryCode2 => ("s", "&str"),
        }
    }

    pub fn end_part(&self) -> &'static str {
        match self {
            CountryMethod::Description
            | CountryMethod::CountryName
            | CountryMethod::CountryCode3
            | CountryMethod::CountryCode2
            | CountryMethod::TopLevelDomain
            | CountryMethod::Currencies
            | CountryMethod::Languages
            | CountryMethod::Neighbours
            | CountryMethod::Capital
            | CountryMethod::Palette
            | CountryMethod::AreaKm
            | CountryMethod::DialingCode
            | CountryMethod::DrivingSide
            | CountryMethod::Emoji
            | CountryMethod::BrightestColor
            | CountryMethod::Population
            | CountryMethod::Flag
            | CountryMethod::FlagNoColor
            | CountryMethod::Continents => "        }\n    }\n",
            CountryMethod::FromStr
            | CountryMethod::FromCountryCode
            | CountryMethod::CountryCode3FromCountryCode2 => {
                "            _ => None\n        }\n    }\n"
            }
        }
    }

    fn format_part(&self, parts: &CountryParts) -> String {
        match self {
            CountryMethod::Description => {
                format!(
                    "            {} => {},\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts
                        .description
                        .as_ref()
                        .map(|d| format!("Some(r###\"{d}\"###)"))
                        .unwrap_or_else(|| "None".to_string())
                )
            }
            CountryMethod::CountryName => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.country_name
                )
            }
            CountryMethod::CountryCode3 => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.country_code3
                )
            }
            CountryMethod::DialingCode => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.dialing_code
                )
            }
            CountryMethod::DrivingSide => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.driving_side
                )
            }
            CountryMethod::CountryCode2 => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.country_code2
                )
            }
            CountryMethod::TopLevelDomain => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.top_level_domains.join(", ")
                )
            }
            CountryMethod::Currencies => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.currencies
                )
            }
            CountryMethod::Languages => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.languages
                )
            }
            CountryMethod::Capital => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.capital.join(", ")
                )
            }
            CountryMethod::Palette => {
                format!(
                    "            {} => {},\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.colors
                )
            }
            CountryMethod::Neighbours => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.neighbours.join(", ")
                )
            }
            CountryMethod::AreaKm => {
                format!(
                    "            {} => {}_f64,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.area_km
                )
            }
            CountryMethod::BrightestColor => {
                format!(
                    "            {} => {},\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.most_colorful
                )
            }
            CountryMethod::Emoji => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.emoji
                )
            }
            CountryMethod::Population => {
                format!(
                    "            {} => {}_u64,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.population
                )
            }
            CountryMethod::Continents => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.continents.join(", ")
                )
            }
            CountryMethod::FromStr => {
                format!(
                    "            \"{}\" => Some(Self::{}),\n",
                    parts.deunicoded_name, parts.enum_name
                )
            }
            CountryMethod::FromCountryCode => {
                format!(
                    "            \"{}\" => Some(Self::{}),\n",
                    parts.country_code3, parts.enum_name
                )
            }
            CountryMethod::CountryCode3FromCountryCode2 => {
                format!(
                    "            \"{}\" => Some(\"{}\"),\n",
                    parts.country_code2, parts.country_code3
                )
            }
            CountryMethod::Flag => format!(
                "            Country::{} => r###\"{}\"###,\n",
                parts.enum_name, parts.flag_color
            ),
            CountryMethod::FlagNoColor => format!(
                "            Country::{} => r###\"{}\"###,\n",
                parts.enum_name, parts.flag_nocolor
            ),
        }
    }
    pub fn return_type(&self) -> &'static str {
        match self {
            CountryMethod::Description => "Option<&'static str>",
            CountryMethod::CountryName => "&'static str",
            CountryMethod::CountryCode3 => "&'static str",
            CountryMethod::CountryCode2 => "&'static str",
            CountryMethod::TopLevelDomain => "&'static [&'static str]",
            CountryMethod::Currencies => "&'static [(&'static str, &'static str, &'static str)]",
            CountryMethod::Languages => "&'static [(&'static str, &'static str)]",
            CountryMethod::Neighbours => "&'static [&'static str]",
            CountryMethod::Capital => "&'static [&'static str]",
            CountryMethod::Palette => "&'static [(u8, u8, u8)]",
            CountryMethod::AreaKm => "f64",
            CountryMethod::DialingCode => "&'static str",
            CountryMethod::DrivingSide => "&'static str",
            CountryMethod::Flag => "&'static str",
            CountryMethod::FlagNoColor => "&'static str",
            CountryMethod::Emoji => "&'static str",
            CountryMethod::BrightestColor => "(u8, u8, u8)",
            CountryMethod::Population => "u64",
            CountryMethod::Continents => "&'static [&'static str]",
            CountryMethod::FromStr => "Option<Self>",
            CountryMethod::FromCountryCode => "Option<Self>",
            CountryMethod::CountryCode3FromCountryCode2 => "Option<&'static str>",
        }
    }
}

struct Codegen {
    /// Starts with this string
    pub start: String,
    /// Closure to compute each individual item
    pub item: Box<dyn Fn(&CountryParts) -> String>,
    /// Ends with this string
    pub end: String,
}

impl Codegen {
    pub fn codegen(&mut self, parts: &CountryParts) {
        self.start.push_str(&(self.item)(parts));
    }
}

impl fmt::Display for Codegen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.start)?;
        f.write_str(&self.end)
    }
}

/// Generates Rust code for country enum and its implementation.
pub async fn generate_code(countries: &[Country]) -> (String, String) {
    let mut country_enum_ = Codegen {
        start: String::from(
            "#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::needless_arbitrary_self_type)]
#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug, clap::ValueEnum)]
#[clap(rename_all = \"PascalCase\")]
pub enum Country {
",
        ),
        item: Box::new(|parts| {
            format!(
                "    #[clap(alias = \"{}\")]
    {},
",
                parts.country_code2, parts.enum_name
            )
        }),
        end: "}\n".to_owned(),
    };

    let country_impl_ = Codegen {
        start: String::from("impl Country {\n"),
        item: Box::new(|_| String::new()),
        end: "}\n".to_owned(),
    };

    let mut all_countries_ = Codegen {
        start: String::from("    pub const ALL_COUNTRIES: &[Self] = &[\n"),
        item: Box::new(|parts| format!("        Country::{},\n", parts.enum_name)),
        end: "    ];\n".to_owned(),
    };

    let mut method_impls: Vec<Codegen> = CountryMethod::iter()
        .map(|method| {
            let (arg, ty) = method.param();

            Codegen {
                start: format!(
                    "    pub fn {method}({arg}: {ty}) -> {} {{\n        match {arg} {{\n",
                    method.return_type()
                ),
                end: method.end_part().to_owned(),
                item: Box::new(move |parts| method.format_part(parts)),
            }
        })
        .collect();

    // Generate all country parts in parallel because it is an expensive operation
    // that also makes network requests
    let country_parts: Vec<CountryParts> = futures::future::join_all(
        countries
            .par_iter()
            .map(country_parts::generate_country_parts)
            .collect::<Vec<_>>(),
    )
    .await;

    // Append all the generated parts to the respective Codegen objects
    for parts in country_parts {
        country_enum_.codegen(&parts);
        all_countries_.codegen(&parts);

        for method_impl in &mut method_impls {
            method_impl.codegen(&parts);
        }
    }

    // Build final strings
    let country_enum = country_enum_.to_string();

    // Build country_impl by combining all method implementations
    let mut country_impl = country_impl_.start.clone();
    country_impl.push_str(&all_countries_.to_string());

    for method_impl in method_impls {
        country_impl.push_str(&method_impl.start);
        country_impl.push_str(&method_impl.end);
    }

    country_impl.push_str(&country_impl_.end);

    (country_enum, country_impl)
}
