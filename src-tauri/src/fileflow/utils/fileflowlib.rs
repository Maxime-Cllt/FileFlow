use crate::fileflow::stuct::save_config::SaveConfig;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;

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
