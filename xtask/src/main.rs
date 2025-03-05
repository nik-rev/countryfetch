use countryfetch::Country;
use std::{
    fs::File,
    io::{Read as _, Write as _},
    path::PathBuf,
};

mod codegen;
mod country_parts;

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

#[allow(unused)]
async fn save_countries_to_json() {
    let value = reqwest::get("https://restcountries.com/v3.1/all")
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let paths = Paths::new();

    let mut all_countries =
        File::open(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("countries.json")).unwrap();

    let mut buf = String::new();
    all_countries.read_to_string(&mut buf).unwrap();
    let all_countries = serde_json::de::from_str::<Vec<Country>>(&buf).unwrap();
    let (country_enum, country_impl, flag_impl) = codegen::generate_code(&all_countries).await;

    write_files(&paths, &country_enum, &country_impl, &flag_impl);
}

async fn fetch_countries() -> Result<Vec<Country>> {
    Ok(reqwest::get("https://restcountries.com/v3.1/all")
        .await?
        .json::<Vec<Country>>()
        .await?)
}

fn most_colorful_color(colors: &[palette_extract::Color]) -> palette_extract::Color {
    *colors
        .iter()
        .max_by(|a, b| {
            // finds the "colorfulness" of the color
            let colorfulness = |color: palette_extract::Color| {
                let min = color.r.min(color.r).min(color.b);
                let max = color.r.max(color.r).max(color.b);
                ((max as u16 + min as u16) * (max as u16 - min as u16)) / max as u16
            };

            colorfulness(**a).cmp(&colorfulness(**b))
        })
        .expect("There is at least 1 color")
}

/// Given a URL to a .png file, convert the file into colored Ascii
async fn png_url_to_ascii(png_url: &str) -> Result<(String, String, Vec<palette_extract::Color>)> {
    let pixels: Vec<u8> = reqwest::get(png_url).await?.bytes().await?.to_vec();

    let image = image::load_from_memory_with_format(&pixels, image::ImageFormat::Png)?;

    let pixels = image.to_rgb8().into_raw();

    let mut colors = palette_extract::get_palette_rgb(pixels.as_slice());
    colors.sort_unstable_by(|a, b| {
        // finds the "colorfulness" of the color
        let brightness = |color: palette_extract::Color| {
            color.r as f32 * 0.2126 + color.g as f32 * 0.7152 + color.b as f32 * 0.0722
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

/// Writes generated Rust code to appropriate files.
fn write_files(paths: &Paths, country_enum: &str, country_impl: &str, flag_impl: &str) {
    std::fs::create_dir_all(&paths.generated_dir).expect("Failed to create generated directory");

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
        .write_all(b"mod country;\nmod flag;\n\npub use country::*;")
        .expect("Failed to write to mod.rs");
}

#[tokio::main]
async fn main() -> Result<()> {
    let all_countries = fetch_countries().await?;
    let paths = Paths::new();

    let (country_enum, country_impl, flag_impl) = codegen::generate_code(&all_countries).await;
    write_files(&paths, &country_enum, &country_impl, &flag_impl);

    Ok(())
}
