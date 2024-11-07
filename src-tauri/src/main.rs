// main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fileflow;
mod tests;

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::fileflow::action::actions::{
    connect_to_database, disconnect_from_database, get_size_of_file, insert_csv_data,
    load_database_config, save_database_config, DatabaseState,
};


fn main() {
    let database_state: Arc<DatabaseState> = Arc::new(DatabaseState(Mutex::new(None)));

    tauri::Builder::default()
        .manage(database_state)
        .invoke_handler(tauri::generate_handler![
            connect_to_database,
            insert_csv_data,
            disconnect_from_database,
            save_database_config,
            load_database_config,
            get_size_of_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
