use image::codecs::ico::{IcoEncoder, IcoFrame};

mod cli;

fn main() {
    let cli = cli::parse();
    if let Err(e) = run(&cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(args: &cli::Args) -> std::io::Result<()> {
    if !args.input.extension().is_some_and(|e| e == "png") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Unsupported extension",
        ));
    }

    let img = image::open(&args.input).expect("failed to open image");

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
