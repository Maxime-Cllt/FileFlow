use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

/// This function is used to detect the separator from a string.
pub fn find_separator(line: &str) -> Result<char, String> {
    const POSSIBLE_SEPARATORS: [char; 6] = [',', ';', '\t', '|', ' ', '\0'];
    for sep in POSSIBLE_SEPARATORS.iter() {
        if line.contains(*sep) {
            return Ok(*sep);
        }
    }
    Err("Could not detect a valid separator".into())
}

/// Read the first line of a file as raw bytes then convert to string
pub fn read_first_line(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut line_bytes = Vec::new();
    let mut buffer = [0u8; 1];

    loop {
        match reader.read_exact(&mut buffer) {
            Ok(()) => {
                if buffer[0] == b'\n' {
                    break;
                }
                if buffer[0] != b'\r' {
                    line_bytes.push(buffer[0]);
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                if line_bytes.is_empty() {
                    return Err(io::Error::new(io::ErrorKind::NotFound, "File is empty"));
                }
                break;
            }
            Err(e) => return Err(e),
        }
    }

    // Convert to string with lossy UTF-8 conversion
    Ok(String::from_utf8_lossy(&line_bytes).to_string())
}
