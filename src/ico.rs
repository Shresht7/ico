pub struct Ico {
    path: std::path::PathBuf,
    resource_type: ico::ResourceType,
    entries: Vec<IcoFrame>,
}

impl std::fmt::Display for Ico {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Path: {}", self.path.display())?;
        writeln!(f, "Type: {:?}", self.resource_type)?;
        writeln!(f, "Entries: {}", self.entries.len())?;
        for (i, entry) in self.entries.iter().enumerate() {
            writeln!(f, "  IcoFrame {}: {}", i + 1, entry)?;
        }
        Ok(())
    }
}

pub struct IcoFrame {
    width: u32,
    height: u32,
    data: Vec<u8>,
    format: IcoFrameFormat,
}

impl From<&ico::IconDirEntry> for IcoFrame {
    fn from(entry: &ico::IconDirEntry) -> Self {
        Self {
            width: entry.width(),
            height: entry.height(),
            data: entry.data().to_vec(),
            format: if entry.is_png() {
                IcoFrameFormat::PNG
            } else {
                IcoFrameFormat::BMP
            },
        }
    }
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

pub enum IcoFrameFormat {
    BMP,
    PNG,
}

impl std::fmt::Display for IcoFrameFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BMP => "BMP",
                Self::PNG => "PNG",
            }
        )
    }
}

/// Extracts information about the ICO file
pub fn info<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<()> {
    let file = std::io::BufReader::new(std::fs::File::open(&path)?);
    let ico = ico::IconDir::read(file)?;

    let ico = Ico {
        path: path.as_ref().to_path_buf(),
        resource_type: ico.resource_type(),
        entries: ico.entries().iter().map(IcoFrame::from).collect(),
    };

    println!("{}", ico);

    Ok(())
}
