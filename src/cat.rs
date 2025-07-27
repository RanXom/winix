use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn cat<S: AsRef<Path>>(files: Vec<S>) -> io::Result<String> {
    let mut result = String::new();

    for file_path in files {
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut line = line?;
            // Normalize Windows-style line endings (\r\n) to Unix-style (\n)
            if line.ends_with('\r') {
                line.pop(); // Remove '\r'
            }
            result.push_str(&line);
            result.push('\n');
        }
    }

    Ok(result)
}
