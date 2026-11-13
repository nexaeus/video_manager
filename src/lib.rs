use std::{ env, fs, io, path::PathBuf };

/// # Resolve Absolute Location.
/// Get user home path on unix based systems and
/// create folder abs path from concatenation of the home path and folder.
pub fn resolve_abs_location(folder: &str) -> Option<PathBuf> {
    if let Some(path) = env::home_dir() {
        let mut location = path.to_path_buf();
        location.push(folder);

        Some(location)
    } else {
        None
    }
}

/// # Video Files Discovery
/// Locate all files in the directories and sub-directories.
pub fn files_discovery(dir: PathBuf, store: &mut Vec<PathBuf>) -> io::Result<()>{
    if dir.is_dir() {
        let result = fs::read_dir(dir)?
        .map(|de| de.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;

        let result = result.iter();

        for path in result {
            files_discovery(path.to_path_buf(), store)?;
        }
    } else {
        store.push(dir);
    }

    Ok(())
}

/// # Filter Files
/// Remove unsupported file formats.
pub fn filter_files(list: Vec<PathBuf>, format: &str) -> Vec<PathBuf> {
    let list = list.iter()
    .filter(|e| {
        if let Some(val) = e.extension() {
            val != format
        } else {
            true
        }
    })
    .map(|e| e.to_path_buf())
    .collect::<Vec<_>>();

    list
}
