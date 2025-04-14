use image::codecs::ico::IcoEncoder;

mod cli;
mod ico;
mod png;
mod svg;

fn main() {
    let cli = cli::parse();
    if let Err(e) = run(&cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(args: &cli::Args) -> anyhow::Result<()> {
    match &args.command {
        cli::Command::Generate {
            input,
            output,
            sizes,
        } => {
            let frames = match input.extension().and_then(|e| e.to_str()) {
                Some("png") => png::generate_frames(input, sizes)?,
                Some("svg") => svg::generate_frames(input, sizes)?,
                x => {
                    return Err(anyhow::anyhow!(
                        "Unsupported image format: {}",
                        x.unwrap_or_default()
                    ));
                }
            };

            let file = std::fs::File::create(output)?;
            IcoEncoder::new(file)
                .encode_images(&frames)
                .expect("failed to encode to ico");
        }

        cli::Command::Info { input } => {
            ico::info(input)?;
        }

        cli::Command::Extract { input, output } => {
            todo!("Implement extraction of images from ICO file");
        }
    }

    Ok(())
}
