use std::env;
use walkdir::WalkDir;

type Result<T = (), E = Box<dyn std::error::Error + 'static>> = std::result::Result<T, E>;

fn main() -> Result {
    let cwd = env::current_dir()?;
    let paths = WalkDir::new(&cwd).into_iter().filter_entry(|entry| {
        entry
            .file_name()
            .to_str()
            .map(|name| !name.starts_with('.') && !name.ends_with("_files"))
            .unwrap_or_default()
    });

    for entry in paths {
        let entry = entry?;
        let relative_path = entry.path().strip_prefix(&cwd)?;
        println!("{}", relative_path.display());
    }

    Ok(())
}
