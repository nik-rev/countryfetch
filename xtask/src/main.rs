use heck::ToPascalCase;
use std::fmt::Write as _;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Location of the current script file
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Path where generated code lives.
    let generated_dir = manifest_dir
        .join("..")
        .join("countryfetch")
        .join("src")
        .join("generated");

    // Path where the .svg files for each of the flags are located.
    let svg_src_dir = manifest_dir.join("flag-svgs").join("4x3");

    // Re-exports of generated code for ease of use.
    let mod_rs = generated_dir.join("mod.rs");

    // country.rs: Contains implementations of all methods for the Country enum.
    let country_rs = generated_dir.join("country.rs");

    // flag.rs: Contains a single implementation of the `Country::flag` method.
    // Impl is in a separate file due to the huge size of this file.
    let flag_rs = generated_dir.join("flag.rs");

    std::fs::create_dir_all(generated_dir).unwrap();

    let mut country_enum = String::from(
        "\
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Country {
",
    );

    let mut country_enum_impl = String::from(
        "impl Country {
",
    );

    let mut country_enum_impl_fn_from_str = String::from(
        "    pub fn from_str(s: &str) -> Option<Self> {
        match s {
",
    );

    let mut country_enum_impl_const_countries = String::from(
        "    pub const ALL_COUNTRIES: &[Self] = &[
",
    );

    let mut country_enum_impl_fn_from_country_code = String::from(
        "    pub fn from_country_code(s: &str) -> Option<Self> {
        match s {
",
    );

    let mut country_enum_impl_fn_country_code = String::from(
        "    pub fn country_code(&self) -> &'static str {
        match self {
",
    );

    // because this one is so large, it goes into a separate file
    let mut flag_rs_contents = String::from(
        "#![cfg_attr(rustfmt, rustfmt_skip)]

use super::Country;

impl Country {
    pub fn flag(&self) -> &'static str {
        match self {
",
    );

    for svg_flag in (*svg_src_dir).read_dir().unwrap().map(|a| a.unwrap()) {
        // let img = {
        //     let svg_path = svg_flag.path();
        //     let tree = {
        //         let opt = resvg::usvg::Options {
        //             resources_dir: Some(svg_path.to_path_buf()),
        //             ..Default::default()
        //         };
        //         let svg_data = std::fs::read(svg_path).unwrap();

        //         resvg::usvg::Tree::from_data(&svg_data, &opt).unwrap()
        //     };

        //     let pixmap_size = tree.size().to_int_size();
        //     let width = pixmap_size.width();
        //     let height = pixmap_size.height();

        //     let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();

        //     resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

        //     pixmap
        //         .encode_png()
        //         .map(std::io::Cursor::new)
        //         .map_err(drop)
        //         .and_then(|png_bytes| {
        //             image::io::Reader::with_format(png_bytes, image::ImageFormat::Png)
        //                 .decode()
        //                 .map_err(drop)
        //         })
        //         .expect("Format is not valid PNG")
        // };

        // let mut ascii_buf: Vec<u8> = Vec::new();

        // rascii_art::render_image(
        //     &img,
        //     &mut ascii_buf,
        //     &rascii_art::RenderOptions::new()
        //         .width(40)
        //         .height(17)
        //         .colored(true),
        // )
        // .expect("Could not render SVG");

        // let ascii_flag_destination = destination.join(flag_name);
        // let mut file = std::fs::File::create(ascii_flag_destination).unwrap();

        // file.write_all(&ascii_buf).unwrap();

        let ascii_buf = "temp";

        let file_name = svg_flag
            .file_name()
            .into_string()
            .expect("filename only consists of ASCII characters");

        let flag_parts: Vec<&str> = file_name.split('.').collect();

        let country_name = flag_parts[0];
        let country_code = flag_parts[1];

        let flag_name_enum_member = country_name.to_pascal_case();

        // Add enum member
        writeln!(&mut country_enum, "    {},", flag_name_enum_member).unwrap();

        // Add match for Countries::from_str
        writeln!(
            &mut country_enum_impl_fn_from_str,
            "            \"{}\" => Some(Country::{}),",
            country_name, flag_name_enum_member
        )
        .unwrap();

        // Add flag for each country
        writeln!(
            &mut flag_rs_contents,
            "            Country::{} => {},",
            flag_name_enum_member, ascii_buf
        )
        .unwrap();

        // Add country code for each country
        writeln!(
            &mut country_enum_impl_fn_country_code,
            "            Country::{} => \"{}\",",
            flag_name_enum_member, country_code
        )
        .unwrap();

        // Add country to array (for iteration)
        writeln!(
            &mut country_enum_impl_const_countries,
            "        Country::{},",
            flag_name_enum_member
        )
        .unwrap();

        // Add match for Countries::from_country_code
        writeln!(
            &mut country_enum_impl_fn_from_country_code,
            "            \"{}\" => Some(Country::{}),",
            country_code, flag_name_enum_member
        )
        .unwrap();
    }

    writeln!(
        &mut country_enum_impl_fn_from_str,
        "            _ => None,
        }}
    }}"
    )
    .unwrap();
    writeln!(
        &mut flag_rs_contents,
        "        }}
    }}
}}"
    )
    .unwrap();
    writeln!(
        &mut country_enum_impl_fn_country_code,
        "        }}
    }}"
    )
    .unwrap();
    writeln!(
        &mut country_enum_impl_fn_from_country_code,
        "            _ => None,
        }}
    }}"
    )
    .unwrap();
    writeln!(&mut country_enum, "}}").unwrap();
    writeln!(&mut country_enum_impl_const_countries, "    ];\n").unwrap();

    write!(
        &mut country_enum_impl,
        "{country_enum_impl_const_countries}{country_enum_impl_fn_country_code}{country_enum_impl_fn_from_str}{country_enum_impl_fn_from_country_code}"
    )
    .unwrap();

    writeln!(&mut country_enum_impl, "}}").unwrap();

    let country_rs_contents = format!("{country_enum}\n{country_enum_impl}");

    let mut country_file = File::create(country_rs).unwrap();
    let mut flag_file = File::create(flag_rs).unwrap();
    let mut mod_file = File::create(mod_rs).unwrap();

    country_file
        .write_all(country_rs_contents.as_bytes())
        .unwrap();

    flag_file.write_all(flag_rs_contents.as_bytes()).unwrap();

    mod_file
        .write_all(
            b"mod country;
mod flag;

pub use country::*;
pub use flag::*;",
        )
        .unwrap();
}
