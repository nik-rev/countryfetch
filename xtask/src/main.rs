use std::fmt::Write as _;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::LazyLock;

/// Location of the current script file
static MANIFEST_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));

/// Path where generated code lives.
static GENERATED_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    MANIFEST_DIR
        .join("..")
        .join("countryfetch")
        .join("src")
        .join("generated")
});

/// Path where are the .svg files for each of the flags
static SVG_SRC_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| MANIFEST_DIR.join("flag-svgs").join("4x3"));

/// Re-exports of generated code for ease of use.
static MOD_RS: LazyLock<PathBuf> = LazyLock::new(|| GENERATED_DIR.join("mod.rs"));

/// country.rs: Contains implementations of all methods for the Country enum.
static COUNTRY_RS: LazyLock<PathBuf> = LazyLock::new(|| GENERATED_DIR.join("country.rs"));

/// flag.rs: Contains a single implementation of the `Country::flag` method.
/// Impl is in a separate file due to the huge size of this file.
static FLAG_RS: LazyLock<PathBuf> = LazyLock::new(|| GENERATED_DIR.join("flag.rs"));

use heck::ToPascalCase;

fn main() {
    std::fs::create_dir_all(GENERATED_DIR.clone()).unwrap();

    let mut country_enum = String::from(
        "\
pub enum Countries {
",
    );

    let mut country_enum_impl = String::from(
        "pub impl Countries {
",
    );

    let mut country_enum_impl_fn_from_str = String::from(
        "    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
",
    );

    let mut country_enum_impl_fn_from_country_code = String::from(
        "    pub fn from_country_code(s: &str) -> Result<Self, ()> {
        match s {
",
    );

    // because this one is so large, it goes into a separate file
    let mut flag_rs = String::from(
        "use super::Countries;

impl Countries {
    pub fn from_str(&self) -> &'static str {
        match self {
",
    );

    for svg_flag in (*SVG_SRC_DIR).read_dir().unwrap().map(|a| a.unwrap()) {
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

        let flag_name = svg_flag.file_name();

        let flag_name_enum_member = flag_name.clone().into_string().unwrap().to_pascal_case();

        // Add enum member
        writeln!(&mut country_enum, "    {},", flag_name_enum_member).unwrap();

        // Add match for Countries::from_str
        writeln!(
            &mut country_enum_impl_fn_from_str,
            "            \"{}\" => Ok(Countries::{}),",
            flag_name.to_str().expect("Only contains ASCII characters"),
            flag_name_enum_member
        )
        .unwrap();

        // Add flag for each country
        writeln!(
            &mut flag_rs,
            "            Countries::{} => {},",
            flag_name_enum_member, ascii_buf
        )
        .unwrap();

        // Add match for Countries::from_country_code
        writeln!(
            &mut country_enum_impl_fn_from_country_code,
            "            \"gb\" => Err(()),"
        )
        .unwrap();
    }

    writeln!(
        &mut country_enum_impl_fn_from_str,
        "            _ => Err(()),
        }}
    }}"
    )
    .unwrap();
    writeln!(
        &mut flag_rs,
        "        }}
    }}
}}"
    )
    .unwrap();
    writeln!(
        &mut country_enum_impl_fn_from_country_code,
        "            _ => Err(()),
        }}
    }}"
    )
    .unwrap();
    writeln!(&mut country_enum, "}}").unwrap();

    write!(
        &mut country_enum_impl,
        "{country_enum_impl_fn_from_str}{country_enum_impl_fn_from_country_code}"
    )
    .unwrap();

    writeln!(&mut country_enum_impl, "}}").unwrap();

    let country_rs = format!("{country_enum}\n{country_enum_impl}");

    let mut country_file = File::create(COUNTRY_RS.clone()).unwrap();
    let mut flag_file = File::create(FLAG_RS.clone()).unwrap();
    let mut mod_file = File::create(MOD_RS.clone()).unwrap();

    country_file.write_all(country_rs.as_bytes()).unwrap();
    flag_file.write_all(flag_rs.as_bytes()).unwrap();
    mod_file
        .write_all(
            b"mod country;
mod flag;

pub use country::*;
pub use flag::*;",
        )
        .unwrap();
}
