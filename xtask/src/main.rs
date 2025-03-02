use std::io::Write;

fn svg_path_to_img(svg_path: std::path::PathBuf) -> image::DynamicImage {
    let tree = {
        let opt = resvg::usvg::Options {
            resources_dir: Some(svg_path.to_path_buf()),
            ..Default::default()
        };
        let svg_data = std::fs::read(svg_path).unwrap();

        resvg::usvg::Tree::from_data(&svg_data, &opt).unwrap()
    };

    let pixmap_size = tree.size().to_int_size();
    let width = pixmap_size.width();
    let height = pixmap_size.height();

    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    pixmap
        .encode_png()
        .map(std::io::Cursor::new)
        .map_err(drop)
        .and_then(|png_bytes| {
            image::io::Reader::with_format(png_bytes, image::ImageFormat::Png)
                .decode()
                .map_err(drop)
        })
        .expect("Format is not valid PNG")
}

fn main() {
    let svg_flags = std::fs::read_dir(std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/flag-svgs/4x3"
    )))
    .unwrap();

    let destination = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../flags"));

    std::fs::create_dir_all(&destination).unwrap();

    for svg_flag in svg_flags.map(|a| a.unwrap()) {
        let img = svg_path_to_img(svg_flag.path());

        let mut ascii_buf: Vec<u8> = Vec::new();

        rascii_art::render_image(
            &img,
            &mut ascii_buf,
            &rascii_art::RenderOptions::new()
                .width(40)
                .height(17)
                .colored(true),
        )
        .expect("Could not render SVG");

        let ascii_flag_destination = destination.join(svg_flag.file_name());
        let mut file = std::fs::File::create(ascii_flag_destination).unwrap();

        file.write_all(&ascii_buf).unwrap();
    }
}
