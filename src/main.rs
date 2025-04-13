use image::codecs::ico::IcoEncoder;

mod cli;
mod png;
mod svg;

fn main() {
    let cli = cli::parse();
    if let Err(e) = run(&cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(args: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    let frames = match args.input.extension().and_then(|e| e.to_str()) {
        Some("png") => png::generate_frames(args)?,
        Some("svg") => svg::generate_frames(args)?,
        x => return Err(format!("Unsupported image format: {}", x.unwrap_or_default()).into()),
    };

    let file = std::fs::File::create(&args.output).expect("failed to create output file");
    IcoEncoder::new(file)
        .encode_images(&frames)
        .expect("failed to encode to ico");

    Ok(())
}
