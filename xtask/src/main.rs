use resvg::usvg;

fn main() {
    let tree = {
        let opt = usvg::Options {
            resources_dir: Some(std::path::PathBuf::from("uk.svg")),
            ..Default::default()
        };
        let svg_data = std::fs::read("uk.svg").unwrap();

        usvg::Tree::from_data(&svg_data, &opt).unwrap()
    };

    let pixmap_size = tree.size().to_int_size();
    let width = pixmap_size.width();
    let height = pixmap_size.height();

    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let png = pixmap
        .encode_png()
        .map(std::io::Cursor::new)
        .map_err(drop)
        .and_then(|png_bytes| {
            image::io::Reader::with_format(png_bytes, image::ImageFormat::Png)
                .decode()
                .map_err(drop)
        })
        .expect("Format is not valid PNG");

    let mut buf: Vec<u8> = Vec::new();

    rascii_art::render_image(
        &png,
        &mut buf,
        &rascii_art::RenderOptions::new()
            .width(40)
            .height(17)
            .colored(true),
    )
    .expect("Could not render SVG");

    let ascii = String::from_utf8(buf).unwrap();

    println!("{ascii}");
}
