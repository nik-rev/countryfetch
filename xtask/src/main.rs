use std::path::Path;

use rascii_art::render_image;
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

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    pixmap.save_png("uk.png").unwrap();

    // let pixmap_size = tiny_skia::Pixmap::new(110, 120).unwrap();
    // tree.si
    // resvg::default_backend();
    // resvg::Render::render_to_image(&self, tree, opt)
    // render_image(buffer, to, options)
}
