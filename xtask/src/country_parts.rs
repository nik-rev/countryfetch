use crate::{image_url_to_ascii, most_colorful_color};
use deunicode::deunicode;
use heck::ToPascalCase as _;

/// Represents all the parts needed to generate code for a country
pub struct CountryParts {
    pub enum_name: String,
    pub deunicoded_name: String,
    pub capital: Vec<String>,
    pub dialing_code: String,
    pub driving_side: String,
    pub country_name: String,
    pub country_code2: String,
    pub most_colorful: String,
    pub country_code3: String,
    pub flag_color: String,
    pub flag_nocolor: String,
    pub colors: String,
    pub description: Option<String>,
    pub top_level_domains: Vec<String>,
    pub currencies: String,
    pub languages: String,
    pub neighbours: Vec<String>,
    pub area_km: f64,
    pub emoji: String,
    pub population: u64,
    pub continents: Vec<String>,
}

/// Generate all the parts needed for a single country
pub async fn generate_country_parts(country: &countryfetch::Country) -> CountryParts {
    let country_name = country.country_name();
    let deunicoded_name = deunicode(country_name);
    let enum_name = deunicoded_name.to_pascal_case();

    let (flag_color, flag_nocolor, colors) = image_url_to_ascii(&country.flag.url).await.unwrap();

    let most_colorful = most_colorful_color(&colors);
    let most_colorful = format!(
        "({}, {}, {})",
        most_colorful.r, most_colorful.g, most_colorful.b
    );

    let top_level_domains = country
        .top_level_domain
        .iter()
        .map(|tld| format!("\"{}\"", tld))
        .collect();

    let colors = format!(
        "&[{}]",
        colors
            .into_iter()
            .map(|color| format!("({}, {}, {})", color.r, color.g, color.b))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let currencies = country
        .currencies
        .iter()
        .map(|(id, currency)| {
            let name = &currency.name;
            let symbol = &currency.symbol;
            format!("(\"{id}\", \"{name}\", \"{symbol}\")")
        })
        .collect::<Vec<_>>()
        .join(", ");

    let languages = country
        .languages
        .iter()
        .map(|(a, b)| format!("(\"{a}\", \"{b}\")"))
        .collect::<Vec<_>>()
        .join(", ");

    let capital = country
        .capital
        .iter()
        .map(|n| format!("\"{}\"", n))
        .collect();

    let neighbours = country
        .neighbours
        .iter()
        .map(|n| format!("\"{}\"", n))
        .collect();

    let continents = country
        .continents
        .iter()
        .map(|c| format!("\"{}\"", c))
        .collect();

    CountryParts {
        enum_name,
        deunicoded_name,
        country_name: country_name.to_owned(),
        country_code2: country.country_code2.clone(),
        country_code3: country.country_code3.clone(),
        flag_color,
        flag_nocolor,
        most_colorful,
        colors,
        driving_side: country.driving_side().to_owned(),
        dialing_code: country.dialing_code(),
        capital,
        description: country.flag.description.clone(),
        top_level_domains,
        currencies,
        languages,
        neighbours,
        area_km: country.area_km,
        emoji: country.emoji.clone(),
        population: country.population,
        continents,
    }
}

const LIBERLAND_FLAG_URL: &str =
    "https://www.comprarbanderas.es/images/banderas/400/21183-liberland_400px.jpg";

pub async fn manual_liberland_country_parts() -> CountryParts {
    let (flag_color, flag_nocolor, colors) = image_url_to_ascii(LIBERLAND_FLAG_URL).await.unwrap();
    let most_colorful = most_colorful_color(&colors);
    let most_colorful = format!(
        "({}, {}, {})",
        most_colorful.r, most_colorful.g, most_colorful.b
    );
    let colors = format!(
        "&[{}]",
        colors
            .into_iter()
            .map(|color| format!("({}, {}, {})", color.r, color.g, color.b))
            .collect::<Vec<_>>()
            .join(", ")
    );

    CountryParts {
        enum_name: "Liberland".to_owned(),
        deunicoded_name: "Liberland".to_owned(),
        capital: Vec::new(),
        dialing_code: String::new(),
        driving_side: "right".to_owned(),
        country_name: "Liberland".to_owned(),
        country_code2: "LL".to_owned(),
        most_colorful,
        country_code3: "LIB".to_owned(),
        flag_color,
        flag_nocolor,
        colors,
        description: Some(
            "The flag of Liberland has black-yellow-black horizontal bands with the coat of arms centered on the yellow band."
                .to_owned(),
        ),
        top_level_domains: Vec::new(),
        currencies: "(\"LLD\", \"Liberland dollar\", \"LLD\")".to_owned(),
        languages: "(\"eng\", \"English\")".to_owned(),
        neighbours: vec!["\"HRV\"".to_owned(), "\"SRB\"".to_owned()],
        area_km: 7.0,
        emoji: "🟨".to_owned(),
        population: 63,
        continents: vec!["\"Europe\"".to_owned()],
    }
}
