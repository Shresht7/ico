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
}

impl std::fmt::Display for IcoFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ width: {}, height: {}, format: {} }}",
            self.width, self.height, self.format
        )
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
