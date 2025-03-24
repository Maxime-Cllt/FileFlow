// main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fileflow;

#[cfg(test)]
mod tests;

use crate::fileflow::action::database_actions::get_table_list;
use fileflow::action::actions::*;
use fileflow::action::database_actions::*;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() {
    let database_state: Arc<DatabaseState> = Arc::new(DatabaseState(Mutex::new(None)));

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(database_state)
        .invoke_handler(tauri::generate_handler![
            connect_to_database,
            insert_csv_data,
            disconnect_from_database,
            save_database_config,
            load_database_config_by_name,
            get_size_of_file,
            is_connected,
            get_all_database_configs_name,
            delete_database_config,
            get_table_list,
            download_table
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
