use std::env;

use countryfetch::{Country, Location};

mod generated;

async fn get_data() -> Result<(Location, Country), Box<dyn std::error::Error>> {
    let ip = public_ip::addr().await.unwrap();

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (location, country) = get_data().await.unwrap();

    let country_cached_data = generated::Country::from_country_code(&country.country_code3)
        .expect("All countries have been cached");

    let flag = if env::var_os("NO_COLOR").is_some() {
        country_cached_data.flag()
    } else {
        country_cached_data.flag_nocolor()
    };

    Ok(())
}
