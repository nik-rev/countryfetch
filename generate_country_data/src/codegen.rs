//! Generate the entire Country enum and all methods associated with it
use core::fmt;

use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use strum::IntoEnumIterator as _;

use crate::Country;
use crate::country_parts::{self, CountryData};

#[derive(strum::EnumIter, strum::Display)]
#[strum(serialize_all = "snake_case")]
enum CountryMethod {
    // --- takes &self
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
    // --- takes &str
    FromStr,
    FromCountryCode,
    CountryCode3FromCountryCode2,
    // ---
}

impl CountryMethod {
    /// Parameters that this method takes
    pub fn param(&self) -> (&'static str, &'static str) {
        match self {
            Self::Description
            | Self::CountryName
            | Self::CountryCode3
            | Self::CountryCode2
            | Self::TopLevelDomain
            | Self::Currencies
            | Self::Languages
            | Self::Neighbours
            | Self::Capital
            | Self::Palette
            | Self::AreaKm
            | Self::DialingCode
            | Self::DrivingSide
            | Self::Emoji
            | Self::BrightestColor
            | Self::Population
            | Self::Flag
            | Self::FlagNoColor
            | Self::Continents => ("self", "&Self"),
            Self::FromStr | Self::FromCountryCode | Self::CountryCode3FromCountryCode2 => {
                ("s", "&str")
            }
        }
    }

    /// When matching `self`, this is the final match arm
    pub fn end_part(&self) -> &'static str {
        match self {
            Self::Description
            | Self::CountryName
            | Self::CountryCode3
            | Self::CountryCode2
            | Self::TopLevelDomain
            | Self::Currencies
            | Self::Languages
            | Self::Neighbours
            | Self::Capital
            | Self::Palette
            | Self::AreaKm
            | Self::DialingCode
            | Self::DrivingSide
            | Self::Emoji
            | Self::BrightestColor
            | Self::Population
            | Self::Flag
            | Self::FlagNoColor
            | Self::Continents => "        }\n    }\n",
            Self::FromStr | Self::FromCountryCode | Self::CountryCode3FromCountryCode2 => {
                "            _ => None\n        }\n    }\n"
            }
        }
    }

    /// Generate the match arm for this method, given
    /// some information about a country
    fn generate_match_arm(&self, parts: &CountryData) -> String {
        match self {
            Self::Description => {
                format!(
                    "            {} => {},\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts
                        .flag_description
                        .as_ref()
                        .map_or_else(|| "None".to_owned(), |d| format!("Some(r###\"{d}\"###)"))
                )
            }
            Self::CountryName => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.country_name
                )
            }
            Self::CountryCode3 => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.country_code3
                )
            }
            Self::DialingCode => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.dialing_code
                )
            }
            Self::DrivingSide => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.driving_side
                )
            }
            Self::CountryCode2 => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.country_code2
                )
            }
            Self::TopLevelDomain => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.top_level_domains.join(", ")
                )
            }
            Self::Currencies => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.currencies
                )
            }
            Self::Languages => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.languages
                )
            }
            Self::Capital => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.capital.join(", ")
                )
            }
            Self::Palette => {
                format!(
                    "            {} => {},\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.flag_colors
                )
            }
            Self::Neighbours => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.neighbours.join(", ")
                )
            }
            Self::AreaKm => {
                format!(
                    "            {} => {}_f64,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.area_km
                )
            }
            Self::BrightestColor => {
                format!(
                    "            {} => {},\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.most_colorful_flag_color
                )
            }
            Self::Emoji => {
                format!(
                    "            {} => r###\"{}\"###,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.emoji
                )
            }
            Self::Population => {
                format!(
                    "            {} => {}_u64,\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.population
                )
            }
            Self::Continents => {
                format!(
                    "            {} => &[{}],\n",
                    format_args!("Self::{}", parts.enum_name),
                    parts.continents.join(", ")
                )
            }
            Self::FromStr => {
                format!(
                    "            \"{}\" => Some(Self::{}),\n",
                    parts.deunicoded_name, parts.enum_name
                )
            }
            Self::FromCountryCode => {
                format!(
                    "            \"{}\" => Some(Self::{}),\n",
                    parts.country_code3, parts.enum_name
                )
            }
            Self::CountryCode3FromCountryCode2 => {
                format!(
                    "            \"{}\" => Some(\"{}\"),\n",
                    parts.country_code2, parts.country_code3
                )
            }
            Self::Flag => {
                format!(
                    "            Country::{} => r###\"{}\"###,\n",
                    parts.enum_name, parts.flag_color
                )
            }
            Self::FlagNoColor => {
                format!(
                    "            Country::{} => r###\"{}\"###,\n",
                    parts.enum_name, parts.flag_nocolor
                )
            }
        }
    }

    /// What this method returns
    pub fn return_type(&self) -> &'static str {
        match self {
            Self::Currencies => "&'static [(&'static str, &'static str, &'static str)]",
            Self::Languages => "&'static [(&'static str, &'static str)]",
            Self::Palette => "&'static [(u8, u8, u8)]",
            Self::AreaKm => "f64",
            Self::DialingCode
            | Self::DrivingSide
            | Self::Emoji
            | Self::CountryCode3
            | Self::FlagNoColor
            | Self::Flag
            | Self::CountryCode2
            | Self::CountryName => "&'static str",
            Self::BrightestColor => "(u8, u8, u8)",
            Self::Population => "u64",
            Self::Continents | Self::TopLevelDomain | Self::Neighbours | Self::Capital => {
                "&'static [&'static str]"
            }
            Self::FromStr | Self::FromCountryCode => "Option<Self>",
            Self::CountryCode3FromCountryCode2 | Self::Description => "Option<&'static str>",
        }
    }
}

/// Generate code for a given country
struct Codegen {
    /// Starts with this string
    pub prefix: String,
    /// Closure to compute each individual item
    pub item: Box<dyn Fn(&CountryData) -> String>,
    /// Ends with this string
    pub suffix: String,
}

impl Codegen {
    /// Generate code for a specific country
    pub fn codegen(&mut self, parts: &CountryData) {
        self.prefix.push_str(&(self.item)(parts));
    }
}

impl fmt::Display for Codegen {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.prefix)?;
        f.write_str(&self.suffix)
    }
}

/// Generates Rust code for country enum and its implementation.
#[expect(clippy::future_not_send, reason = "TODO")]
pub async fn generate_code(countries: &[Country]) -> (String, String) {
    let mut country_enum = Codegen {
        prefix: String::from(
            "// @generated
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::needless_arbitrary_self_type)]

pub mod extras;
pub use extras::*;

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug, clap::ValueEnum)]
#[clap(rename_all = \"PascalCase\")]
pub enum Country {
",
        ),
        item: Box::new(|data| {
            format!(
                "    #[clap(alias = \"{}\")]
    {},
",
                data.country_code2, data.enum_name
            )
        }),
        suffix: "}\n".to_owned(),
    };

    let mut all_countries = Codegen {
        prefix: String::from("    pub const ALL_COUNTRIES: &[Self] = &[\n"),
        item: Box::new(|parts| format!("        Country::{},\n", parts.enum_name)),
        suffix: "    ];\n".to_owned(),
    };

    let mut country_methods: Vec<Codegen> = CountryMethod::iter()
        .map(|method| {
            let (arg, ty) = method.param();

            Codegen {
                prefix: format!(
                    "    pub fn {method}({arg}: {ty}) -> {} {{\n        match {arg} {{\n",
                    method.return_type()
                ),
                suffix: method.end_part().to_owned(),
                item: Box::new(move |parts| method.generate_match_arm(parts)),
            }
        })
        .collect();

    // Generate all country parts in parallel because it is an expensive operation
    // that also makes network requests
    let country_data: Vec<CountryData> = futures::future::join_all(
        countries
            .par_iter()
            .map(country_parts::generate_country_data)
            .collect::<Vec<_>>(),
    )
    .await;

    // Append all the generated parts to the respective Codegen objects
    for country in country_data {
        // Write the Country enum variant
        country_enum.codegen(&country);
        all_countries.codegen(&country);

        // Generate all methods for the country
        for country_method in &mut country_methods {
            // Write each match arm for the country's method
            country_method.codegen(&country);
        }
    }

    // All method implementations on the country
    let country_impl = format!(
        "\
impl Country {{
{all_countries}{}}}\n",
        country_methods
            .into_iter()
            .map(|method_impl| format!("{method_impl}"))
            .collect::<String>()
    );

    (country_enum.to_string(), country_impl)
}
