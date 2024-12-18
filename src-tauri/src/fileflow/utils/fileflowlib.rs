use csv::ReaderBuilder;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

/// This function is used to generate the column names for a CSV file.
pub fn get_formated_column_names(headers: Vec<String>) -> Vec<String> {
    const COLUMN_PREFIX: &str = "column_";

    let mut headers: Vec<String> = headers;
    for (i, item) in headers.iter_mut().enumerate() {
        if item.trim().is_empty() {
            *item = format!("{COLUMN_PREFIX}{}", i + 1);
        }
    }
    headers
        .iter()
        .map(|h| h.to_lowercase().replace(' ', "_"))
        .collect()
}

/// This function is used to detect the separator in a CSV file.
pub fn detect_separator_in_file(file_path: &str) -> io::Result<char> {
    const POSSIBLE_SEPARATORS: [char; 6] = [',', ';', '\t', '|', ' ', '\0'];

    let file: File = File::open(file_path)?;

    if !std::path::Path::new(file_path)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("csv"))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "The file is not a CSV file",
        ));
    }

    let mut reader: BufReader<File> = BufReader::new(file);
    let mut buffer: String = String::new();
    reader.read_to_string(&mut buffer)?;

    for sep in &POSSIBLE_SEPARATORS {
        let mut csv_reader = ReaderBuilder::new()
            .delimiter(*sep as u8)
            .has_headers(false)
            .from_reader(buffer.as_bytes());

        if csv_reader.records().next().is_some() {
            return Ok(*sep);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Could not detect a valid separator",
    ))
}
