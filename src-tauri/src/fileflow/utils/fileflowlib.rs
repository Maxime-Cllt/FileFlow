use crate::fileflow::stuct::save_config::SaveConfig;
use csv::{ReaderBuilder, StringRecord};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, Read};
use std::path::PathBuf;

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

/// Sanitize a value for safe insertion into the database
pub fn sanitize_value(value: &str) -> String {
    value.trim().replace("'", "''").replace("\\", "\\\\")
}

/// Escape values for SQL insert statement to avoid SQL injection attacks and other issues with special characters in values.
pub fn escaped_values(values: StringRecord) -> String {
    let vec: Vec<String> = values
        .iter()
        .map(|v| format!("'{}'", sanitize_value(v)))
        .collect();
    vec.join(", ")
}

/// This function is used to get the size of a file.
pub fn get_all_saved_configs(config_file : &str) -> Vec<SaveConfig> {
    let default_configs: Vec<SaveConfig> = Vec::new();

    let path: PathBuf = PathBuf::from(config_file);
    let file: File = match File::open(path) {
        Ok(file) => file,
        Err(_) => return default_configs,
    };

    if file.metadata().unwrap().len() == 0 {
        return default_configs;
    }

    let configs: Vec<SaveConfig> = serde_json::from_reader(file).unwrap_or(default_configs);
    configs
}

/// This function is used to save a vector of SaveConfig to a json file.
pub fn save_config(configs: &[SaveConfig], config_file: &str) -> io::Result<()> {
    let file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(config_file)
        .map_err(|e| format!("Failed to open file for writing: {e}"))
        .unwrap();
    serde_json::to_writer_pretty(file, &configs)
        .map_err(|e| format!("Failed to write to file: {e}"))
        .unwrap();
    Ok(())
}
