use std::fmt::Write as _;
use std::fs::File;
use std::io::Write as _;

use heck::ToPascalCase;

fn main() {
    let svg_flags = std::fs::read_dir(std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/flag-svgs/4x3"
    )))
    .unwrap();

    let destination = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../flags"));

    std::fs::create_dir_all(&destination).unwrap();

    let mut country_enum = String::from(
        "\
enum Countries {
",
    );

    let mut country_enum_impl = String::from(
        "impl Countries {
",
    );

    let mut country_enum_impl_fn_from_str = String::from(
        "    fn from_str(s: &str) -> Result<Self> {
        match s {
",
    );

    let mut country_enum_impl_fn_from_country_code = String::from(
        "    fn from_country_code(s: &str) -> Result<Self, ()> {
        match s {
",
    );

    for svg_flag in svg_flags.map(|a| a.unwrap()) {
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

        // Add match for Countries::from_country_code
        writeln!(
            &mut country_enum_impl_fn_from_country_code,
            "            \"gb\" => Err(()),"
        )
        .unwrap();
    }

    writeln!(
        &mut country_enum_impl_fn_from_str,
        "            _ => Err(),
        }}
    }}"
    )
    .unwrap();
    writeln!(
        &mut country_enum_impl_fn_from_country_code,
        "            _ => Err(),
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

    let lol = format!("{country_enum}\n{country_enum_impl}");

    let mut m = File::create(std::path::PathBuf::from("innit.rs")).unwrap();

    m.write_all(lol.as_bytes()).unwrap();
}
