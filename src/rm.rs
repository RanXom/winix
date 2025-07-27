use std::fs;
use std::io;
use std::path::Path;

pub fn rm<S: AsRef<Path>>(files: Vec<S>) -> io::Result<()> {
    for file_path in files {
        let path = file_path.as_ref();

        if path.exists() {
            if path.is_file() {
                fs::remove_file(path)?;
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, format!("'{}' is not a file", path.display())));
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("File '{}' not found", path.display())));
        }
    }
    Ok(())
}
