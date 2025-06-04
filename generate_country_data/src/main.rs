//! Generate country data
use std::fs::File;
use std::io::Write as _;
use std::path::PathBuf;

use countryfetch::Country;
use icu_collator::Collator;

mod codegen;
mod country_parts;

type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

/// Obtain all of the country data
async fn fetch_countries() -> Result<Vec<Country>> {
    Ok(reqwest::get("https://restcountries.com/v3.1/all")
        .await?
        .json::<Vec<Country>>()
        .await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut all_countries = fetch_countries().await?;

    // sort the countries in lexical order
    let mut collator_options = icu_collator::CollatorOptions::new();
    collator_options.strength = Some(icu_collator::Strength::Secondary);
    let collator = Collator::try_new(Default::default(), collator_options).unwrap();
    // alphabetical sort of the countries, by their name
    all_countries.sort_unstable_by(|a, b| collator.compare(a.country_name(), b.country_name()));

    let (country_enum, country_impl) = codegen::generate_code(&all_countries).await;

    File::create(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("src")
            .join("generated_country_data.rs"),
    )
    .expect("Failed to create generated country enum")
    .write_all(format!("{}\n{}", country_enum, country_impl).as_bytes())
    .expect("Failed to write to country.rs");

    Ok(())
}
