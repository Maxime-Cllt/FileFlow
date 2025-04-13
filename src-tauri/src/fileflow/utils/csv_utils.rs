use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

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

/// Read the first line of a file
pub fn read_first_line(file_path: &str) -> io::Result<String> {
    let file: File = File::open(file_path).expect("Could not open file");
    let reader: BufReader<File> = BufReader::new(file);
    if let Some(line) = reader.lines().next() {
        return line;
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "File is empty"))
}
