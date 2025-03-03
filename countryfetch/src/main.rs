use serde_query::{DeserializeQuery, Query};
use std::{collections::HashMap, net::IpAddr};

use serde::{Deserialize, Serialize};

mod generated;

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
    #[serde(rename = "tld")]
    pub top_level_domain: Vec<String>,
    #[serde(rename = "cca2")]
    pub country_code: String,
    pub currencies: HashMap<String, Currency>,
    pub languages: HashMap<String, String>,
    #[serde(rename = "borders")]
    pub neighbours: Vec<String>,
    #[serde(rename = "area")]
    pub area_km: f64,
    #[serde(rename = "flag")]
    pub emoji: String,
    pub population: u32,
    pub continents: Vec<String>,
    #[serde(rename = "flags")]
    pub flag: Flag,
    pub coat_of_arms: CoatOfArms,
}

impl Country {
    pub fn country_name(&self) -> &str {
        &self.country_name.common
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub common: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    #[serde(rename = "png")]
    pub url: String,
    #[serde(rename = "alt")]
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoatOfArms {
    pub png: String,
    pub svg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub name: String,
    pub symbol: String,
}

async fn get_data() -> Result<(Location, Country), Box<dyn std::error::Error>> {
    let ip = public_ip::addr().await.unwrap();
    let client = reqwest::Client::new();

    let ip_data = client.get(format!(
        "http://ip-api.com/json/{ip}?fields=status,message,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting,query"
    )).send().await?.json::<Location>().await?;

    let country = client
        .get(format!(
            "https://restcountries.com/v3.1/alpha/{}",
            ip_data.country_code
        ))
        .send()
        .await?
        .json::<Vec<Country>>()
        .await?
        .into_iter()
        .next()
        .expect("Returns a single country from querying for a Country Code, when there is a valid country code (which would have failed earlier)");

    Ok((ip_data, country))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = get_data().await.unwrap();

    dbg!(data);

    Ok(())
}
