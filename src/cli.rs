use clap::Parser;

/// Simple ICO generator from SVG/PNG files
#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Input image file (SVG or PNG)
    pub input: std::path::PathBuf,

    /// Output .ico file
    #[arg(short, long, default_value = "output.ico")]
    pub output: std::path::PathBuf,

    /// The comma-separated list of icon sizes to include (e.g. 16,32,48,64,256)
    #[arg(short, long, value_delimiter = ',', default_value = "16,32,48,64,256")]
    pub sizes: Vec<u32>,
}

pub fn parse() -> Args {
    Args::parse()
}
