#![allow(
    clippy::missing_docs_in_private_items,
    missing_docs,
    reason = "this just models the JSON API"
)]
use std::collections::HashMap;

use serde::Deserialize;

/// Information obtained for a specific country
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    /// Name of the country
    #[serde(rename = "name")]
    #[expect(clippy::struct_field_names, reason = "this is cleaner")]
    country_name: Name,
    #[serde(default)]
    #[serde(rename = "tld")]
    pub top_level_domain: Vec<String>,
    #[serde(rename = "cca3")]
    pub country_code3: String,
    #[serde(rename = "cca2")]
    pub country_code2: String,
    #[serde(default)]
    pub currencies: HashMap<String, Currency>,
    #[serde(default)]
    pub languages: HashMap<String, String>,
    #[serde(default)]
    #[serde(rename = "borders")]
    pub neighbours: Vec<String>,
    #[serde(rename = "area")]
    pub area_km: f64,
    #[serde(rename = "flag")]
    pub emoji: String,
    pub population: u64,
    pub continents: Vec<String>,
    #[serde(rename = "flags")]
    pub flag: Flag,
    #[serde(default)]
    pub capital: Vec<String>,
    car: Car,
    #[serde(rename = "idd")]
    dialing_code: Idd,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Idd {
    // TODO: Option<String>
    #[serde(default)]
    root: String,
    #[serde(default)]
    suffixes: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    side: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    // #[serde(default)]
    pub common: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    #[serde(rename = "png")]
    pub url: String,
    #[serde(rename = "alt")]
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub name: String,
    pub symbol: String,
}

impl Country {
    /// Fetch a single country from the API
    ///
    /// # Panics
    ///
    /// When the API returns an empty array
    pub async fn from_cc2(country_code2: &str) -> Result<Self, reqwest::Error> {
        Ok(reqwest::get(format!(
            "https://restcountries.com/v3.1/alpha/{country_code2}"
        ))
        .await?
        .json::<Vec<Self>>()
        .await?
        .into_iter()
        .next()
        .expect("API always returns an array of 1 Country when fetching for a specific country"))
    }

    /// Fetch all countries from the API
    pub async fn fetch_all() -> Result<Vec<Self>, reqwest::Error> {
        reqwest::get("https://restcountries.com/v3.1/all")
            .await?
            .json::<Vec<Self>>()
            .await
    }

    #[must_use]
    pub fn country_name(&self) -> &str {
        &self.country_name.common
    }
    #[must_use]
    pub fn driving_side(&self) -> &str {
        &self.car.side
    }
    #[must_use]
    pub fn dialing_code(&self) -> String {
        let codes = self
            .dialing_code
            .suffixes
            .iter()
            .take(3)
            .map(|suffix| format!("{root}{suffix}", root = self.dialing_code.root))
            .collect::<Vec<_>>();
        // .join(", ");

        let codes_left = self.dialing_code.suffixes.len() - codes.len();

        let extra_codes = if codes_left == 0 {
            String::new()
        } else {
            format!(" (+{codes_left} more...)")
        };

        format!("{}{extra_codes}", codes.join(", "))
    }
}
