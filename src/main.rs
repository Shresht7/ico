use image::codecs::ico::{IcoEncoder, IcoFrame};

mod cli;
mod svg;

fn main() {
    let cli = cli::parse();
    if let Err(e) = run(&cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn load_image<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    match path.as_ref().extension().and_then(|e| e.to_str()) {
        Some("png") => Ok(image::open(path)?),
        Some("svg") => Ok(svg::open(path)?),
        x => Err(format!("Unsupported image format: {}", x.unwrap_or_default()).into()),
    }
}

fn run(args: &cli::Args) -> std::io::Result<()> {
    let img = load_image(&args.input).expect("failed to load image");

    let img = img.resize(16, 16, image::imageops::FilterType::Lanczos3);
    let rgba = img.to_rgba8();

    let frame = IcoFrame::as_png(&rgba, 16, 16, image::ExtendedColorType::Rgba8)
        .expect("failed to make ico frame");

    let file = std::fs::File::create(&args.output).expect("failed to create output file");
    IcoEncoder::new(file)
        .encode_images(&[frame])
        .expect("failed to encode to ico");

    Ok(())
}
