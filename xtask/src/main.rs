use countryfetch::Country;
use deunicode::deunicode;
use heck::ToPascalCase;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs::{File, create_dir_all};
use std::io::Write as _;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Paths {
    // Path where generated code lives.
    generated_dir: PathBuf,
    // Re-exports of generated code for ease of use.
    mod_rs: PathBuf,
    // country.rs: Contains implementations of all methods for the Country enum.
    country_rs: PathBuf,
    // flag.rs: Contains a single implementation of the Country::flag method.
    // Impl is in a separate file due to the huge size of this file.
    flag_rs: PathBuf,
}

impl Paths {
    fn new() -> Self {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let generated_dir = manifest_dir
            .join("..")
            .join("countryfetch")
            .join("src")
            .join("generated");

        Self {
            generated_dir: generated_dir.clone(),
            mod_rs: generated_dir.join("mod.rs"),
            country_rs: generated_dir.join("country.rs"),
            flag_rs: generated_dir.join("flag.rs"),
        }
    }
}

async fn fetch_countries() -> Result<Vec<Country>> {
    Ok(reqwest::get("https://restcountries.com/v3.1/all")
        .await?
        .json::<Vec<Country>>()
        .await?)
}

/// Given a URL to a .png file, convert the file into colored Ascii
async fn png_url_to_ascii(png_url: &str) -> Result<(String, String)> {
    let bytes: Vec<u8> = reqwest::get(png_url).await?.bytes().await?.to_vec();

    let image = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)?;

    let mut flag_color = Vec::new();
    let mut flag_nocolor = Vec::new();
    rascii_art::render_image(
        &image,
        &mut flag_color,
        &rascii_art::RenderOptions::new()
            .width(40)
            .height(17)
            .colored(true),
    )?;
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
    ))
}

/// Generates Rust code for country enum and its implementation.
async fn generate_code(countries: &[Country]) -> (String, String, String) {
    // ----- Types to represent the code structure -----

    /// Represents a method to generate for the Country impl
    struct Method {
        name: &'static str,
        return_type: &'static str,
        default_case: Option<&'static str>,
    }

    /// Represents a static method that takes a string parameter
    struct StringMatchMethod {
        name: &'static str,
        return_type: &'static str,
        match_expr: &'static str,
        default_case: &'static str,
    }

    // ----- Configuration of methods to generate -----

    // Regular instance methods that match on self
    let instance_methods = vec![
        Method {
            name: "description",
            return_type: "Option<&'static str>",
            default_case: None,
        },
        Method {
            name: "country_name",
            return_type: "&'static str",
            default_case: None,
        },
        Method {
            name: "country_code3",
            return_type: "&'static str",
            default_case: None,
        },
        Method {
            name: "country_code2",
            return_type: "&'static str",
            default_case: None,
        },
        Method {
            name: "top_level_domain",
            return_type: "&'static [&'static str]",
            default_case: None,
        },
        Method {
            name: "currencies",
            return_type: "&'static [(&'static str, &'static str, &'static str)]",
            default_case: None,
        },
        Method {
            name: "languages",
            return_type: "&'static [(&'static str, &'static str)]",
            default_case: None,
        },
        Method {
            name: "neighbours",
            return_type: "&'static [&'static str]",
            default_case: None,
        },
        Method {
            name: "area_km",
            return_type: "f64",
            default_case: None,
        },
        Method {
            name: "emoji",
            return_type: "&'static str",
            default_case: None,
        },
        Method {
            name: "population",
            return_type: "u64",
            default_case: None,
        },
        Method {
            name: "continents",
            return_type: "&'static [&'static str]",
            default_case: None,
        },
    ];

    // Static methods that match on a string parameter
    let string_match_methods = [
        StringMatchMethod {
            name: "from_str",
            return_type: "Option<Self>",
            match_expr: "s",
            default_case: "_ => None,",
        },
        StringMatchMethod {
            name: "from_country_code",
            return_type: "Option<Self>",
            match_expr: "s",
            default_case: "_ => None,",
        },
        StringMatchMethod {
            name: "country_code3_from_country_code2",
            return_type: "Option<&'static str>",
            match_expr: "s",
            default_case: "_ => None,",
        },
    ];

    // ----- Code generation -----

    let mut country_enum = String::from(
        "#![cfg_attr(rustfmt, rustfmt_skip)]\n#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]\npub enum Country {\n",
    );

    let mut country_impl = String::from("impl Country {\n");

    // The flag implementation goes in a separate file with its own header
    let mut flag_impl = String::from(
        "#![cfg_attr(rustfmt, rustfmt_skip)]\n\nuse super::Country;\n\nimpl Country {\n    pub const fn flag(&self) -> &'static str {\n        match self {\n",
    );

    // The flag implementation goes in a separate file with its own header
    let mut flag_no_color_impl = String::from(
        "\n\nimpl Country {\n    pub const fn flag_nocolor(&self) -> &'static str {\n        match self {\n",
    );

    let mut all_countries = String::from("    pub const ALL_COUNTRIES: &[Self] = &[\n");

    // Initialize method implementations
    let mut method_impls = instance_methods
        .iter()
        .map(|method| {
            (
                method,
                format!(
                    "    pub const fn {}(&self) -> {} {{\n        match self {{\n",
                    method.name, method.return_type
                ),
            )
        })
        .collect::<Vec<_>>();

    let mut string_method_impls = string_match_methods
        .iter()
        .map(|method| {
            (
                method,
                format!(
                    "    pub fn {}({}: &str) -> {} {{\n        match {} {{\n",
                    method.name, method.match_expr, method.return_type, method.match_expr
                ),
            )
        })
        .collect::<Vec<_>>();

    let country_parts: Vec<CountryParts> = futures::future::join_all(
        countries
            .par_iter()
            .map(generate_country_parts)
            .collect::<Vec<_>>(),
    )
    .await;

    // Append all the generated parts to the respective strings
    for parts in country_parts {
        // Add to the enum
        country_enum.push_str(&format!("    {},\n", parts.enum_name));

        // Add to ALL_COUNTRIES
        all_countries.push_str(&format!("        Country::{},\n", parts.enum_name));

        // Add to flag implementation
        flag_impl.push_str(&format!(
            "            Country::{} => r###\"{}\"###,\n",
            parts.enum_name, parts.flag_color
        ));

        // Add to flag implementation
        flag_no_color_impl.push_str(&format!(
            "            Country::{} => r###\"{}\"###,\n",
            parts.enum_name, parts.flag_nocolor
        ));

        // Add to string match methods
        for (method, impl_str) in &mut string_method_impls {
            match method.name {
                "from_str" => {
                    impl_str.push_str(&format!(
                        "            \"{}\" => Some(Country::{}),\n",
                        parts.deunicoded_name, parts.enum_name
                    ));
                }
                "from_country_code" => {
                    impl_str.push_str(&format!(
                        "            \"{}\" => Some(Country::{}),\n",
                        parts.country_code3, parts.enum_name
                    ));
                }
                "country_code3_from_country_code2" => {
                    impl_str.push_str(&format!(
                        "            \"{}\" => Some(\"{}\"),\n",
                        parts.country_code2, parts.country_code3
                    ));
                }
                _ => panic!("Unknown string match method: {}", method.name),
            }
        }

        // Add to instance methods
        for (method, impl_str) in &mut method_impls {
            match method.name {
                "description" => {
                    impl_str.push_str(&format!(
                        "            {} => {},\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts
                            .description
                            .as_ref()
                            .map(|d| format!("Some(r###\"{d}\"###)"))
                            .unwrap_or_else(|| "None".to_string())
                    ));
                }
                "country_name" => {
                    impl_str.push_str(&format!(
                        "            {} => r###\"{}\"###,\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.country_name
                    ));
                }
                "country_code3" => {
                    impl_str.push_str(&format!(
                        "            {} => r###\"{}\"###,\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.country_code3
                    ));
                }
                "country_code2" => {
                    impl_str.push_str(&format!(
                        "            {} => r###\"{}\"###,\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.country_code2
                    ));
                }
                "top_level_domain" => {
                    impl_str.push_str(&format!(
                        "            {} => &[{}],\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.top_level_domains.join(", ")
                    ));
                }
                "currencies" => {
                    impl_str.push_str(&format!(
                        "            {} => &[{}],\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.currencies
                    ));
                }
                "languages" => {
                    impl_str.push_str(&format!(
                        "            {} => &[{}],\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.languages
                    ));
                }
                "neighbours" => {
                    impl_str.push_str(&format!(
                        "            {} => &[{}],\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.neighbours.join(", ")
                    ));
                }
                "area_km" => {
                    impl_str.push_str(&format!(
                        "            {} => {}_f64,\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.area_km
                    ));
                }
                "emoji" => {
                    impl_str.push_str(&format!(
                        "            {} => r###\"{}\"###,\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.emoji
                    ));
                }
                "population" => {
                    impl_str.push_str(&format!(
                        "            {} => {}_u64,\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.population
                    ));
                }
                "continents" => {
                    impl_str.push_str(&format!(
                        "            {} => &[{}],\n",
                        format_args!("Country::{}", parts.enum_name),
                        parts.continents.join(", ")
                    ));
                }
                _ => panic!("Unknown method: {}", method.name),
            }
        }
    }

    // Close the enum
    country_enum.push_str("}\n");

    // Close ALL_COUNTRIES
    all_countries.push_str("    ];\n");

    // Close all method implementations and add default cases
    for (method, impl_str) in &mut method_impls {
        if let Some(default_case) = method.default_case {
            impl_str.push_str(&format!("            {}\n", default_case));
        }
        impl_str.push_str("        }\n    }\n");
    }

    // Close string match methods with default cases
    for (method, impl_str) in &mut string_method_impls {
        impl_str.push_str(&format!("            {}\n", method.default_case));
        impl_str.push_str("        }\n    }\n");
    }

    // Close flag implementation
    flag_impl.push_str("        }\n    }\n}\n");
    flag_no_color_impl.push_str("        }\n    }\n}\n");
    flag_impl.push_str(&flag_no_color_impl);

    // Combine all method implementations into the country_impl
    country_impl.push_str(&all_countries);

    for (_, impl_str) in method_impls {
        country_impl.push_str(&impl_str);
    }

    for (_, impl_str) in string_method_impls {
        country_impl.push_str(&impl_str);
    }

    // Close the implementation
    country_impl.push_str("}\n");

    (country_enum, country_impl, flag_impl)
}

/// Represents all the parts needed to generate code for a country
struct CountryParts {
    enum_name: String,
    deunicoded_name: String,
    country_name: String,
    country_code2: String,
    country_code3: String,
    flag_color: String,
    flag_nocolor: String,
    description: Option<String>,
    top_level_domains: Vec<String>,
    currencies: String,
    languages: String,
    neighbours: Vec<String>,
    area_km: f64,
    emoji: String,
    population: u64,
    continents: Vec<String>,
}

/// Generate all the parts needed for a single country
async fn generate_country_parts(country: &Country) -> CountryParts {
    let country_name = country.country_name();
    let deunicoded_name = deunicode(country_name);
    let enum_name = deunicoded_name.to_pascal_case();

    let (flag_color, flag_nocolor) = png_url_to_ascii(&country.flag.url).await.unwrap();

    let top_level_domains = country
        .top_level_domain
        .iter()
        .map(|tld| format!("\"{}\"", tld))
        .collect();

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
        country_name: country_name.to_string(),
        country_code2: country.country_code2.clone(),
        country_code3: country.country_code3.clone(),
        flag_color,
        flag_nocolor,
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
/// Writes generated Rust code to appropriate files.
fn write_files(paths: &Paths, country_enum: &str, country_impl: &str, flag_impl: &str) {
    create_dir_all(&paths.generated_dir).expect("Failed to create generated directory");

    File::create(&paths.country_rs)
        .expect("Failed to create country.rs")
        .write_all(format!("{}\n{}", country_enum, country_impl).as_bytes())
        .expect("Failed to write to country.rs");

    File::create(&paths.flag_rs)
        .expect("Failed to create flag.rs")
        .write_all(flag_impl.as_bytes())
        .expect("Failed to write to flag.rs");

    File::create(&paths.mod_rs)
        .expect("Failed to create mod.rs")
        .write_all(b"mod country;\nmod flag;\n\npub use country::*;\npub use flag::*;")
        .expect("Failed to write to mod.rs");
}

#[tokio::main]
async fn main() -> Result<()> {
    let paths = Paths::new();
    let all_countries = fetch_countries().await?;
    let (country_enum, country_impl, flag_impl) = generate_code(&all_countries).await;
    write_files(&paths, &country_enum, &country_impl, &flag_impl);

    Ok(())
}
