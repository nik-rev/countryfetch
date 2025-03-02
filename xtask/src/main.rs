use heck::ToPascalCase;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs::{self, File, create_dir_all};
use std::io::Write as _;
use std::path::{Path, PathBuf};

struct Paths {
    // Path where generated code lives.
    generated_dir: PathBuf,
    // Path where the .svg files for each of the flags are located.
    svg_src_dir: PathBuf,
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
            svg_src_dir: manifest_dir.join("flag-svgs").join("4x3"),
            mod_rs: generated_dir.join("mod.rs"),
            country_rs: generated_dir.join("country.rs"),
            flag_rs: generated_dir.join("flag.rs"),
        }
    }
}

/// Reads the available SVG flag files from the source directory.
fn read_svg_files(svg_src_dir: &Path) -> Vec<PathBuf> {
    svg_src_dir
        .read_dir()
        .expect("Failed to read SVG directory")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect()
}

/// Generates ASCII art representation of a flag from an SVG file.
fn generate_ascii_art(svg_path: &Path) -> String {
    let svg_data = fs::read(svg_path).expect("Failed to read SVG file");
    let tree =
        resvg::usvg::Tree::from_data(&svg_data, &Default::default()).expect("Invalid SVG data");
    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .expect("Failed to create Pixmap");
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let img = image::io::Reader::with_format(
        std::io::Cursor::new(pixmap.encode_png().expect("Failed to encode PNG")),
        image::ImageFormat::Png,
    )
    .decode()
    .expect("Failed to decode PNG image");

    let mut ascii_buf = Vec::new();
    rascii_art::render_image(
        &img,
        &mut ascii_buf,
        &rascii_art::RenderOptions::new()
            .width(40)
            .height(17)
            .colored(true),
    )
    .expect("Could not render SVG to ASCII");

    String::from_utf8(ascii_buf).expect("Invalid UTF-8 in ASCII art")
}

/// Parses the filename to extract the country name and code.
fn parse_filename(file_name: &str) -> (String, String) {
    let parts: Vec<&str> = file_name.split('.').collect();

    assert!(
        parts.len() == 3,
        "File name: {file_name}, File name must have the following form: {{human-readable-name}}.{{country-code}}.svg"
    );

    (parts[0].to_string(), parts[1].to_string()) // (country_name, country_code)
}

/// Generates Rust code for country enum and its implementation.
fn generate_code(svg_files: &[PathBuf]) -> (String, String, String) {
    let mut country_enum = String::from(
        "#![cfg_attr(rustfmt, rustfmt_skip)]\n#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]\npub enum Country {\n",
    );

    let mut country_impl = String::from("impl Country {\n");
    let mut from_str_match =
        String::from("    pub fn from_str(s: &str) -> Option<Self> {\n        match s {\n");
    let mut from_code_match = String::from(
        "    pub fn from_country_code(s: &str) -> Option<Self> {\n        match s {\n",
    );
    let mut country_code_match =
        String::from("    pub fn country_code(&self) -> &'static str {\n        match self {\n");
    let mut all_countries = String::from("    pub const ALL_COUNTRIES: &[Self] = &[\n");

    let mut flag_impl = String::from(
        "#![cfg_attr(rustfmt, rustfmt_skip)]\n\nuse super::Country;\n\nimpl Country {\n    pub fn flag(&self) -> &'static str {\n        match self {\n",
    );

    // parallel iteration, each thread does heavy computation:
    // 1. Rendering the SVG into a PNG
    // 2. Parsing the PNG into an Ascii representation
    let additions: Vec<_> = svg_files
        .par_iter()
        .map(|svg_path| {
            let file_name = svg_path.file_name().unwrap().to_string_lossy().to_string();
            let (country_name, country_code) = parse_filename(&file_name);
            let country_enum_member = country_name.to_pascal_case();
            let ascii_art = generate_ascii_art(svg_path);

            (
                format!(
                    "            Country::{} => r###\"{}\"###,\n",
                    country_enum_member, ascii_art
                ),
                format!("    {},\n", country_enum_member),
                format!(
                    "            \"{}\" => Some(Country::{}),\n",
                    country_name, country_enum_member
                ),
                format!(
                    "            \"{}\" => Some(Country::{}),\n",
                    country_code, country_enum_member
                ),
                format!(
                    "            Country::{} => \"{}\",\n",
                    country_enum_member, country_code
                ),
                format!("        Country::{},\n", country_enum_member),
            )
        })
        .collect();

    // Where mutation is required, no parallel iteration
    for (
        flag_line,
        enum_line,
        from_str_line,
        from_code_line,
        country_code_line,
        all_countries_line,
    ) in additions
    {
        flag_impl.push_str(&flag_line);
        country_enum.push_str(&enum_line);
        from_str_match.push_str(&from_str_line);
        from_code_match.push_str(&from_code_line);
        country_code_match.push_str(&country_code_line);
        all_countries.push_str(&all_countries_line);
    }

    from_str_match.push_str("            _ => None,\n        }\n    }\n");
    from_code_match.push_str("            _ => None,\n        }\n    }\n");
    country_code_match.push_str("        }\n    }\n");
    all_countries.push_str("    ];\n");

    flag_impl.push_str("        }\n    }\n}");
    country_enum.push('}');

    country_impl.push_str(&all_countries);
    country_impl.push_str(&country_code_match);
    country_impl.push_str(&from_str_match);
    country_impl.push_str(&from_code_match);
    country_impl.push_str("}\n");

    (country_enum, country_impl, flag_impl)
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

fn main() {
    let paths = Paths::new();
    let svg_files = read_svg_files(&paths.svg_src_dir);
    let (country_enum, country_impl, flag_impl) = generate_code(&svg_files);
    write_files(&paths, &country_enum, &country_impl, &flag_impl);
}
