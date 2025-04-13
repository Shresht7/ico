use resvg::tiny_skia::{self, Pixmap};

pub fn open<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let file_contents = std::fs::read(path)?;

    let tree = usvg::Tree::from_data(&file_contents, &usvg::Options::default())?;

    let size = 256.0;
    let size =
        tiny_skia::Size::from_wh(size, size).expect("unsigned values should always be valid");

    let scaled_size = tree.size().scale_to(size);
    let sx = scaled_size.width() / tree.size().width();
    let sy = scaled_size.height() / tree.size().height();
    let transform = tiny_skia::Transform::from_scale(sx, sy);

    let pixmap_size = scaled_size.to_int_size();
    let mut pixmap =
        Pixmap::new(pixmap_size.width(), pixmap_size.height()).expect("failed to create pixmap");
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let img = image::RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec())
        .ok_or("failed to convert pixmap to image")
        .unwrap();

    Ok(image::DynamicImage::ImageRgba8(img))
}
