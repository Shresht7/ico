use anyhow::Context;
use image::codecs::ico::IcoFrame;

use crate::cli;

pub fn generate_frames(args: &cli::Args) -> anyhow::Result<Vec<IcoFrame>> {
    let img = image::open(&args.input).context("failed to open file")?;

    let img = img.resize(16, 16, image::imageops::FilterType::Lanczos3);
    let rgba = img.to_rgba8();
    let frame = IcoFrame::as_png(&rgba, 16, 16, image::ExtendedColorType::Rgba8)
        .context("failed to make ico frame")?;
    Ok(vec![frame])
}
