use anyhow::Context;
use image::codecs::ico::IcoFrame;

use crate::cli;

/// Generates [`IcoFrame`]s from a PNG file
/// This function takes a file path and a list of sizes, and generates ICO frames for each size.
pub fn generate_frames(args: &cli::Args) -> anyhow::Result<Vec<IcoFrame>> {
    let img = image::open(&args.input).context("failed to open file")?;
    let frames: Vec<IcoFrame> = args
        .sizes
        .iter()
        .map(|size| -> Result<IcoFrame<'_>, anyhow::Error> {
            let img = img.resize_exact(*size, *size, image::imageops::FilterType::Lanczos3);
            let rgba = img.to_rgba8();
            Ok(
                IcoFrame::as_png(&rgba, *size, *size, image::ExtendedColorType::Rgba8)
                    .context("failed to make ico frame")?,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(frames)
}
