// main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod optimized_insert;
mod fast_insert;
mod fileflow;

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::fileflow::action::actions::{
    DatabaseState, connect_to_database, insert_csv_data, disconnect_from_database,
    save_database_config, load_database_config, get_size_of_file,
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
