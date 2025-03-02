use std::net::IpAddr;

use serde::{Deserialize, Serialize};

mod generated;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpResponse {
    pub status: Option<String>,
    pub message: Option<String>,
    pub continent: Option<String>,
    pub continent_code: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub region_name: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub zip: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub timezone: Option<String>,
    pub currency: Option<String>,
    pub mobile: Option<bool>,
    pub proxy: Option<bool>,
    pub hosting: Option<bool>,
    pub query: Option<String>,
}

async fn get_ip_data() -> Result<IpResponse, Box<dyn std::error::Error>> {
    let ip = public_ip::addr().await.unwrap();

    let ip_data = reqwest::get(format!(
        "http://ip-api.com/json/{ip}?fields=status,message,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting,query"
    )).await?.json::<IpResponse>().await?;

    Ok(ip_data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = get_ip_data().await.unwrap();

    dbg!(data);

    Ok(())
}
