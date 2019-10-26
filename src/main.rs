use std::env;
use walkdir::WalkDir;

type Result<T = (), E = Box<dyn std::error::Error + 'static>> = std::result::Result<T, E>;

fn main() -> Result {
    let cwd = env::current_dir()?;
    let paths = WalkDir::new(&cwd).into_iter().filter_entry(|entry| {
        // Should filter out all the "Foo_files" folders but still descend into others.
        entry.file_type().is_file() || !entry.file_name().to_string_lossy().ends_with("_files")
    });

    for entry in paths {
        let entry = entry?;
        let relative_path = entry.path().strip_prefix(&cwd)?;
        println!("{}", relative_path.display());
    }

    Ok(())
}
