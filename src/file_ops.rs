use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Given a path to a file, it returns a `String` (hopefully). It works well for
/// small files, but future improvements include a better approach for large files.
/// 
/// Errors
/// 
/// It will return an `io::Error` if `path` does not exist.
pub fn read_file_to_string<T: AsRef<Path>>(path: &T) -> Result<String, io::Error> {
    let mut file = File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
