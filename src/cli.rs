use clap::{Parser, Subcommand};

use std::path::PathBuf;

/// Simple ICO generator from SVG/PNG files
#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a new ICO file from PNG or SVG
    Generate {
        /// Input image file (SVG or PNG)
        input: PathBuf,

        /// Output .ico file
        #[arg(short, long, default_value = "output.ico")]
        output: PathBuf,

        /// The comma-separated list of icon sizes to include (e.g. 16,32,48,64,256)
        #[arg(short, long, value_delimiter = ',', default_value = "16,32,48,64,256")]
        sizes: Vec<u32>,
    },

    /// Show information about an ICO file
    Info {
        /// Input ICO file
        input: PathBuf,

        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },

    /// Extract images from an ICO file
    Extract {
        /// Input ICO file
        input: PathBuf,

        /// Output directory for extracted images
        #[arg(short, long, default_value = "ico")]
        output: PathBuf,
    },
}

/// Parse the command-line arguments
pub fn parse() -> Args {
    Args::parse()
}
