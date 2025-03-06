#![allow(clippy::cargo_common_metadata, reason = "TODO")]
#![allow(clippy::multiple_crate_versions, reason = "TODO")]
use countryfetch::Country;
use std::io::Write as _;
use std::{fs::File, path::PathBuf};

mod codegen;
mod country_parts;

type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

async fn fetch_countries() -> Result<Vec<Country>> {
    Ok(reqwest::get("https://restcountries.com/v3.1/all")
        .await?
        .json::<Vec<Country>>()
        .await?)
}

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

/// Given a URL to a .png file, convert the file into colored Ascii
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

#[tokio::main]
async fn main() -> Result<()> {
    let all_countries = fetch_countries().await?;

    let (country_enum, country_impl) = codegen::generate_code(&all_countries).await;

    File::create(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("gen_country")
            .join("src")
            .join("lib.rs"),
    )
    .expect("Failed to create country.rs")
    .write_all(format!("{}\n{}", country_enum, country_impl).as_bytes())
    .expect("Failed to write to country.rs");

    Ok(())
}
