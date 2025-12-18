---
package.edition = "2024"

[dependencies]
resvg = "0.45"
tiny-skia = "0.11"
palette_extract = "0.1"
image = "0.24"
rascii_art = "0.4"
eyre = "0.6"
color-eyre = "0.6"
rayon = "1"
ureq = { version = "3", features = ["json"] }
serde_json = "1"
serde = { version = "1", features = ["derive"] }
subdef = "0.1"
rkyv = "0.8"
flate2 = "1"

quote = "1"
quip = "0.2"
syn = { version = "2", features = ["full"] }
proc-macro2 = "1"
heck = "0.5"
prettyplease = "0.2"
deunicode = "1.6"
---

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/countries.rs"));

use eyre::OptionExt as _;
use eyre::Result;
use eyre::WrapErr as _;
use eyre::eyre;
use quip::quip;
use rayon::prelude::*;
use std::io::Write;

const COUNTRIES_JSON_URL: &str = "https://gitlab.com/restcountries/restcountries/-/raw/master/src/main/resources/countriesV3.1.json?ref_type=heads";

fn main() -> Result<()> {
    let countries = ureq::get(COUNTRIES_JSON_URL)
        .call()?
        .body_mut()
        .read_json::<Vec<serde_json::Map<String, serde_json::Value>>>()?;

    let countries = countries
        .into_par_iter()
        .map(|mut country| -> Result<_> {
            // "name"."common": String
            let name = || {
                country
                    .get("name")
                    .expect("country has no name")
                    .get("common")
                    .expect("country has no common name")
            };

            // "flag"."svg": String (url)
            let flag = country
                .get("flags")
                .ok_or_else(|| eyre!("expected `flag` for country `{}`", name()))?
                .get("svg")
                .ok_or_else(|| eyre!("expected `svg` in `flag` field for country `{}`", name()))?
                .as_str()
                .ok_or_else(|| eyre!("`svg.flag` field must be a string for country `{}`", name()))?
                .to_string();

            let svg = ureq::get(flag).call()?.body_mut().read_to_vec()?;

            // &str -> svg
            let svg = resvg::usvg::Tree::from_data(&svg, &resvg::usvg::Options::default())?;
            // svg -> png
            let image = svg_to_image(svg)?;
            // png -> ascii
            let ascii = image_to_ascii(image)?;

            // Insert the 3 extra variables
            // camelCase because we use `rename_all = "camelCase"`

            country.insert("flagAsciiPlain".into(), ascii.plain.into());
            country.insert("flagAsciiColored".into(), ascii.colored.into());
            country.insert(
                "flagPalette".into(),
                ascii
                    .palette
                    .into_iter()
                    .map(|palette_extract::Color { r, g, b }| [r, g, b])
                    .collect(),
            );

            Ok(country)
        })
        .filter_map(|country| {
            match country {
                Ok(country) => Some(country),
                Err(err) => {
                    eprintln!("{:?}", err.wrap_err("failed to obtain data for country"));
                    None
                }
            }
        })
        .collect::<Vec<_>>();
    // add ID to all countries
    let countries = countries
        .into_iter()
        .enumerate()
        .map(|(i, mut country)| {
            country.insert("countryId".to_string(), i.into());
            country
        })
        .collect::<Vec<_>>();
    let countries: Vec<Country> = serde_json::from_str(&serde_json::to_string(&countries)?)?;

    let (countries_ident, country_variants): (Vec<_>, Vec<_>) = countries
        .iter()
        .map(|country| {
            let variant_name = heck::AsPascalCase(country.name.common.clone()).to_string();
            let variant_name = deunicode::deunicode(&variant_name);
            let variant_name = syn::Ident::new(&variant_name, proc_macro2::Span::call_site());
            (
                variant_name.clone(),
                quip! {
                    #[clap(alias = #{country.cca2})]
                    #variant_name
                },
            )
        })
        .unzip();
    let countries_ident_2 = countries_ident.clone();
    let country_variant_numbers = countries.iter().enumerate().map(|(i, _)| i);
    let country_variant_numbers_2 = countries.iter().enumerate().map(|(i, _)| i);
    let country_variant_numbers_3 = countries.iter().enumerate().map(|(i, _)| i);
    let country_enum = quip! {
        use crate::countries::Country;
        use crate::countries::COUNTRIES_DATA;

        #[rustfmt::skip]
        #[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Debug, clap::ValueEnum, strum::VariantArray)]
        #[clap(rename_all = "PascalCase")]
        pub enum CountryKind {
            #(#country_variants,)*
        }

        /// Slice containing information about every country
        #[rustfmt::skip]
        pub fn all_countries() -> [&'static Country; #{countries.len()}] {
            [#(&COUNTRIES_DATA.0[#country_variant_numbers_2],)*]
        }

        #[rustfmt::skip]
        impl CountryKind {
            /// Data about this specific country
            pub fn data(self) -> &'static Country {
                match self {
                    #(Self::#countries_ident => &COUNTRIES_DATA.0[#country_variant_numbers],)*
                }
            }
        }

        #[rustfmt::skip]
        impl Country {
            pub fn kind(&self) -> CountryKind {
                match self.country_id {
                    #(#country_variant_numbers_3 => CountryKind::#countries_ident_2,)*
                    _ => panic!("unknown `country_id`")
                }
            }
        }
    };

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&Countries(countries)).unwrap();

    // Compress into `.gz`
    let mut writer = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    writer.write_all(&bytes)?;
    let bytes = writer.finish()?;

    std::fs::write(
        concat!(env!("CARGO_MANIFEST_DIR"), "/countries.rkyv.gz"),
        bytes,
    )
    .wrap_err("failed to create `countries.rkyv`")?;

    let countries_enum = prettyplease::unparse(&syn::parse_file(&country_enum.to_string())?);
    std::fs::write(
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/gen_countries.rs"),
        countries_enum.as_bytes(),
    )?;

    Ok(())
}

struct AsciiFlag {
    /// Ascii flag without colors
    plain: String,
    /// Ascii flag with colors
    colored: String,
    /// List of brightest colors associated with this flag
    palette: Vec<palette_extract::Color>,
}

fn svg_to_image(svg: resvg::usvg::Tree) -> Result<image::DynamicImage> {
    let pixmap_size = svg.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or_eyre("failed to create pixmap")?;
    resvg::render(&svg, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    let png = pixmap.encode_png()?;
    let png = image::io::Reader::with_format(std::io::Cursor::new(png), image::ImageFormat::Png);
    Ok(png.decode()?)
}

fn image_to_ascii(image: image::DynamicImage) -> Result<AsciiFlag> {
    let pixels = image.to_rgb8().into_raw();

    let mut palette = palette_extract::get_palette_rgb(pixels.as_slice());
    palette.sort_unstable_by(|a, b| {
        // finds the "colorfulness" of the color
        let colorfulness = |color: palette_extract::Color| {
            f32::from(color.b).mul_add(
                0.0722,
                f32::from(color.r).mul_add(0.2126, f32::from(color.g) * 0.7152),
            ) as u16
        };

        colorfulness(*a).cmp(&colorfulness(*b))
    });

    let mut flag_plain = Vec::new();
    let mut flag_colored = Vec::new();

    // Plain version
    rascii_art::render_image(
        &image,
        &mut flag_plain,
        &rascii_art::RenderOptions::new()
            .width(40)
            .height(17)
            .colored(false),
    )?;

    // Colorful version
    rascii_art::render_image(
        &image,
        &mut flag_colored,
        &rascii_art::RenderOptions::new()
            .width(40)
            .height(17)
            .colored(true),
    )?;

    Ok(AsciiFlag {
        plain: String::from_utf8(flag_plain)?,
        colored: String::from_utf8(flag_colored)?,
        palette,
    })
}
