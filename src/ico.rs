use image::{ImageDecoder, codecs::ico::IcoDecoder};

pub struct IcoInfo {
    pub path: std::path::PathBuf,
    pub dimensions: (u32, u32),
}

impl std::fmt::Display for IcoInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ICO file: {}", self.path.display())?;
        writeln!(f, "Dimensions: {}x{}", self.dimensions.0, self.dimensions.1)
    }
}

/// Extracts information about the ICO file
pub fn info<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<IcoInfo> {
    let file = std::io::BufReader::new(std::fs::File::open(&path)?);
    let decoder = IcoDecoder::new(file)?;
    Ok(IcoInfo {
        path: path.as_ref().to_path_buf(),
        dimensions: decoder.dimensions(),
    })
}
