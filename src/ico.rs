use std::io::Write;

use serde::Serialize;

#[derive(Serialize)]
pub struct Ico {
    path: std::path::PathBuf,
    resource_type: String,
    entries: Vec<IcoFrame>,
}

impl std::fmt::Display for Ico {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Path: {}", self.path.display())?;
        writeln!(f, "Type: {}", self.resource_type)?;
        writeln!(f, "Entries: {}", self.entries.len())?;
        for (i, entry) in self.entries.iter().enumerate() {
            writeln!(f, "  IcoFrame {}: {}", i + 1, entry)?;
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct IcoFrame {
    width: u32,
    height: u32,
    format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_depth: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor_hotspot: Option<(u16, u16)>,
}

impl std::fmt::Display for IcoFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("width: {}  ", self.width));
        str.push_str(&format!("height: {}  ", self.height));
        str.push_str(&format!("format: {}  ", self.format));

        if let Some(color_depth) = self.color_depth {
            str.push_str(&format!("bits-per-pixel: {}  ", color_depth));
        }

        if let Some(cursor_hotspot) = self.cursor_hotspot {
            str.push_str(&format!(
                "cursor-hotspot: ({}, {})",
                cursor_hotspot.0, cursor_hotspot.1
            ));
        }

        write!(f, "{}", str)
    }
}

impl From<&ico::IconDirEntry> for IcoFrame {
    fn from(entry: &ico::IconDirEntry) -> Self {
        Self {
            width: entry.width(),
            height: entry.height(),
            format: if entry.is_png() {
                "png".into()
            } else {
                "bmp".into()
            },
            color_depth: if entry.bits_per_pixel() == 0 {
                None
            } else {
                Some(entry.bits_per_pixel())
            },
            cursor_hotspot: entry.cursor_hotspot(),
        }
    }
}

/// Extracts information about the ICO file
pub fn info<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<()> {
    let file = std::io::BufReader::new(std::fs::File::open(&path)?);
    let ico = ico::IconDir::read(file)?;

    let ico = Ico {
        path: path.as_ref().to_path_buf(),
        resource_type: match ico.resource_type() {
            ico::ResourceType::Icon => "icon".into(),
            ico::ResourceType::Cursor => "cursor".into(),
        },
        entries: ico.entries().iter().map(IcoFrame::from).collect(),
    };

    println!("{}", ico);

    Ok(())
}

/// Extracts information about the ICO file in JSON format
pub fn info_json<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<()> {
    let file = std::io::BufReader::new(std::fs::File::open(&path)?);
    let ico = ico::IconDir::read(file)?;

    let ico = Ico {
        path: path.as_ref().to_path_buf(),
        resource_type: match ico.resource_type() {
            ico::ResourceType::Icon => "icon".into(),
            ico::ResourceType::Cursor => "cursor".into(),
        },
        entries: ico.entries().iter().map(IcoFrame::from).collect(),
    };

    println!("{}", serde_json::to_string_pretty(&ico)?);

    Ok(())
}

/// Extracts the frames from the ICO file and saves them as PNG files
pub fn extract<P: AsRef<std::path::Path>>(input: P, output: P) -> anyhow::Result<()> {
    let file = std::io::BufReader::new(std::fs::File::open(&input)?);
    let ico = ico::IconDir::read(file)?;

    let output = output.as_ref();
    if !output.exists() {
        std::fs::create_dir_all(output)?;
    } else if !output.is_dir() {
        return Err(anyhow::anyhow!("Output path is not a directory"));
    }

    for entry in ico.entries() {
        let size = format!("{}x{}", entry.width(), entry.height());
        let output_path = output.join(format!("frame_{size}.png"));
        let mut file = std::fs::File::create(output_path)?;
        file.write_all(entry.data())?;
    }

    Ok(())
}
