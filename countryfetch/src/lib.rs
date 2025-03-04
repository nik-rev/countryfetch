use std::collections::HashMap;

mod generated;

use serde::Deserialize;

/// Information obtained from the user's IP
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub status: String,
    pub message: Option<String>,
    pub continent: String,
    pub continent_code: String,
    pub country: String,
    pub country_code: String,
    pub region: String,
    pub region_name: String,
    pub city: String,
    pub district: String,
    pub zip: String,
    #[serde(rename = "lat")]
    pub latitude: f64,
    #[serde(rename = "lon")]
    pub longitude: f64,
    pub timezone: String,
    pub currency: String,
    pub mobile: bool,
    pub proxy: bool,
    pub hosting: bool,
    pub query: String,
}

/// Information obtained for a specific country
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    #[serde(rename = "name")]
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
    pub capital: Vec<String>,
    car: Car,
    #[serde(rename = "idd")]
    dialing_code: Idd,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Idd {
    root: String,
    #[serde(default)]
    suffixes: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    side: String,
}

impl Country {
    pub fn country_name(&self) -> &str {
        &self.country_name.common
    }
    pub fn driving_side(&self) -> &str {
        &self.car.side
    }
    pub fn dialing_code(&self) -> String {
        format!(
            "{}{}",
            self.dialing_code.root,
            self.dialing_code.suffixes.join("")
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub common: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    #[serde(rename = "png")]
    pub url: String,
    #[serde(rename = "alt")]
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub name: String,
    pub symbol: String,
}
