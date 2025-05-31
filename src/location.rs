use core::net;

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

impl Location {
    pub async fn from_ip(ip: net::IpAddr) -> Result<Self, reqwest::Error> {
        reqwest::get(
            format!(
                "http://ip-api.com/json/{ip}?fields=status,message,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting,query"
            )
        ).await?.json::<Self>().await
    }
}
