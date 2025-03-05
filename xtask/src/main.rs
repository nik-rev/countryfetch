use countryfetch::Country;
use std::io::Write;
use std::{fs::File, path::PathBuf};

mod codegen;
mod country_parts;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

#[tokio::main]
async fn main() -> Result<()> {
    let all_countries = fetch_countries().await?;

    let (country_enum, country_impl) = codegen::generate_code(&all_countries).await;

    File::create(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("countryfetch")
            .join("src")
            .join("generated.rs"),
    )
    .expect("Failed to create country.rs")
    .write_all(format!("{}\n{}", country_enum, country_impl).as_bytes())
    .expect("Failed to write to country.rs");

    Ok(())
}
