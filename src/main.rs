// FIXME: add file type and extension filters
// Maybe also a regex/string.contains() filter

use std::{env, error, fmt::Display, io::{self, Write}, str::FromStr};

mod characterization;

use clap::Clap;
use walkdir::WalkDir;

type Result<T = (), E = Box<dyn std::error::Error + 'static>> = std::result::Result<T, E>;

#[derive(Clap, Clone, Debug)]
struct Opts {
    #[clap(short, long)]
    extension: Option<String>,
    #[clap(short, long)]
    kind: Option<FileKind>,
    #[clap(short, long)]
    contains: Option<String>,
}

#[derive(Clone, Debug)]
enum FileKind {
    Image,
    Text,
    Video,
}

impl FromStr for FileKind {
    type Err = ParseFileKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = s.to_ascii_lowercase();
        match &*kind {
            "i" | "image" => Ok(FileKind::Image),
            "t" | "text" => Ok(FileKind::Text),
            "v" | "video" => Ok(FileKind::Video),
            _ => Err(ParseFileKindError),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ParseFileKindError;

impl Display for ParseFileKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("use image, text, or video")
    }
}

impl error::Error for ParseFileKindError {}

fn main() -> Result {
    let opts = Opts::parse();
    
    let cwd = env::current_dir()?;
    let paths = WalkDir::new(&cwd).into_iter().filter_entry(|entry| {
        entry
            .file_name()
            .to_str()
            .map(|name| !name.starts_with('.') && !name.ends_with("_files"))
            .unwrap_or_default()
    });

    let output = io::stdout();
    let mut output = output.lock();

    for entry in paths {
        let entry = entry?;
        if entry.file_type().is_file() {
            let relative_path = entry.path().strip_prefix(&cwd)?;
            writeln!(output, "{}", relative_path.display())?;
        }
    }

    Ok(())
}
