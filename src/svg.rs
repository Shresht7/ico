use anyhow::Context;
use image::codecs::ico::IcoFrame;
use resvg::tiny_skia::{self, Pixmap};

use crate::cli;

/// Generates [`IcoFrame`]s from an SVG file
pub fn generate_frames(args: &cli::Args) -> anyhow::Result<Vec<IcoFrame>> {
    let file_contents = std::fs::read(&args.input)?;
    let tree = usvg::Tree::from_data(&file_contents, &usvg::Options::default())?;
    let frame = generate_frame(&tree, 256)?;
    Ok(vec![frame])
}

/// Rasterizes the given [SVG tree][usvg::Tree] into an [`IcoFrame`] of given `size`
fn generate_frame(tree: &usvg::Tree, size: usize) -> anyhow::Result<IcoFrame<'static>> {
    // Scale the SVG to fit the given size
    let size =
        tiny_skia::Size::from_wh(size as f32, size as f32).context("failed to create size")?;
    let scaled_size = tree.size().scale_to(size);
    let sx = scaled_size.width() / tree.size().width();
    let sy = scaled_size.height() / tree.size().height();
    let transform = tiny_skia::Transform::from_scale(sx, sy);

    // Render the SVG to a pixmap of the specified size
    let pixmap_size = scaled_size.to_int_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .context("failed to create pixmap")?;
    resvg::render(tree, transform, &mut pixmap.as_mut());

    // Convert the pixmap to an image and create an IcoFrame from it
    let img = image::RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec())
        .ok_or_else(|| anyhow::anyhow!("failed to convert pixmap to image"))?;

    let frame = IcoFrame::as_png(
        &img,
        pixmap_size.width(),
        pixmap_size.height(),
        image::ExtendedColorType::Rgba8,
    )
    .context("failed to create ICO frame")?;

    Ok(frame)
}
