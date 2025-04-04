use crate::fileflow::stuct::save_config::SaveConfig;
use csv::StringRecord;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader};
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

/// Sanitize a value for safe insertion into the database
pub fn sanitize_value(value: &str) -> String {
    value.trim().replace("'", "''").replace("\\", "\\\\")
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

/// Escape values for SQL insert statement to avoid SQL injection attacks and other issues with special characters in values.
pub fn escaped_values(values: StringRecord) -> String {
    let vec: Vec<String> = values
        .iter()
        .map(|v| format!("'{}'", sanitize_value(v)))
        .collect();
    vec.join(", ")
}

/// This function is used to get the size of a file.
pub fn get_all_saved_configs(config_file: &str) -> Vec<SaveConfig> {
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
        .expect("Failed to open file for writing");
    serde_json::to_writer_pretty(file, &configs)
        .map_err(|e| format!("Failed to write to file: {e}"))
        .expect("Failed to write to file");
    Ok(())
}
