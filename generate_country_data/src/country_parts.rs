//! Compute all of the data associated with a single country

use deunicode::deunicode;
use heck::ToPascalCase as _;

use crate::Result;

/// Represents all the data associated with a country
pub struct CountryData {
    /// Name of the enum variant for the country
    pub enum_name: String,
    /// ASCII-only string representation of the country's name
    pub deunicoded_name: String,
    /// Capital city
    pub capital: Vec<String>,
    /// Telephone dialing code of the country
    pub dialing_code: String,
    /// Which side the people drive on
    pub driving_side: String,
    /// Name of the country
    pub country_name: String,
    /// The most colorful color of the flag
    pub most_colorful_flag_color: String,
    /// 2-letter country code
    pub country_code2: String,
    /// 3-letter country code
    pub country_code3: String,
    /// Colored ASCII representation of the flag of the country
    /// It is colored using terminal true color escape codes.
    pub flag_color: String,
    /// ASCII representation of the flag of the country
    pub flag_nocolor: String,
    /// Colors of the country's flag
    pub flag_colors: String,
    /// Information about the country's flag
    pub flag_description: Option<String>,
    /// TLDs available for this country
    pub top_level_domains: Vec<String>,
    /// Currencies used here
    pub currencies: String,
    /// Languages spoken here
    pub languages: String,
    /// Neightboring countries, in 3-letter country codes
    pub neighbours: Vec<String>,
    /// Area
    pub area_km: f64,
    /// Flag emoji
    pub emoji: String,
    /// How many people live here
    pub population: u64,
    /// Adjacent continents
    pub continents: Vec<String>,
}

/// Given a list of colors, finds the most colorful color
#[expect(clippy::integer_division, reason = "Cannot compare floats")]
fn most_colorful_color(colors: &[palette_extract::Color]) -> palette_extract::Color {
    *colors
        .iter()
        .max_by(|a, b| {
            // finds the "colorfulness" of the color
            let colorfulness = |color: palette_extract::Color| {
                let min = color.r.min(color.r).min(color.b);
                let max = color.r.max(color.r).max(color.b);
                ((u16::from(max) + u16::from(min)) * (u16::from(max) - u16::from(min)))
                    / u16::from(max)
            };

            colorfulness(**a).cmp(&colorfulness(**b))
        })
        .expect("There is at least 1 color")
}

/// Given a URL to a `.png` file, convert the file into:
/// - A colored ascii flag
/// - A non-colored ascii flag
/// - List of brighest colors associated with this flag
async fn png_url_to_ascii(png_url: &str) -> Result<(String, String, Vec<palette_extract::Color>)> {
    let pixels: Vec<u8> = reqwest::get(png_url).await?.bytes().await?.to_vec();

    let image = image::load_from_memory_with_format(&pixels, image::ImageFormat::Png)?;

    let pixels = image.to_rgb8().into_raw();

    let mut colors = palette_extract::get_palette_rgb(pixels.as_slice());
    colors.sort_unstable_by(|a, b| {
        // finds the "colorfulness" of the color
        #[expect(clippy::cast_possible_truncation, reason = "number will be positive")]
        #[expect(clippy::cast_sign_loss, reason = "same as above")]
        let brightness = |color: palette_extract::Color| {
            f32::from(color.b).mul_add(
                0.0722,
                f32::from(color.r).mul_add(0.2126, f32::from(color.g) * 0.7152),
            )
        } as u16;

        brightness(*a).cmp(&brightness(*b))
    });

    let mut flag_color = Vec::new();
    let mut flag_nocolor = Vec::new();

    // Colorful version
    rascii_art::render_image(
        &image,
        &mut flag_color,
        &rascii_art::RenderOptions::new()
            .width(40)
            .height(17)
            .colored(true),
    )?;

    // Colorless version
    rascii_art::render_image(
        &image,
        &mut flag_nocolor,
        &rascii_art::RenderOptions::new()
            .width(40)
            .height(17)
            .colored(false),
    )?;

    Ok((
        String::from_utf8(flag_color)?,
        String::from_utf8(flag_nocolor)?,
        colors,
    ))
}

/// Process a `Country` obtained from the API into a data structure which
/// includes additional information, such as:
/// - Fully colored flag of the country, obtained from the image
/// - The country's most colorful color
pub async fn generate_country_data(country: &countryfetch::Country) -> CountryData {
    let country_name = country.country_name();
    let deunicoded_name = deunicode(country_name);
    let enum_name = deunicoded_name.to_pascal_case();

    let (flag_color, flag_nocolor, colors) = png_url_to_ascii(&country.flag.url).await.unwrap();

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

    CountryData {
        enum_name,
        deunicoded_name,
        country_name: country_name.to_owned(),
        country_code2: country.country_code2.clone(),
        country_code3: country.country_code3.clone(),
        flag_color,
        flag_nocolor,
        most_colorful_flag_color: most_colorful,
        flag_colors: colors,
        driving_side: country.driving_side().to_owned(),
        dialing_code: country.dialing_code(),
        capital,
        flag_description: country.flag.description.clone(),
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
