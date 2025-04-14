use image::{ImageDecoder, codecs::ico::IcoDecoder};

pub struct IcoInfo {
    pub dimensions: (u32, u32),
}

/// Extracts information about the ICO file
pub fn info<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<IcoInfo> {
    let file = std::io::BufReader::new(std::fs::File::open(path)?);
    let decoder = IcoDecoder::new(file)?;
    Ok(IcoInfo {
        dimensions: decoder.dimensions(),
    })
}
