// main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fileflow;

#[cfg(test)]
mod tests;

use fileflow::action::actions::{
    connect_to_database, disconnect_from_database, execute_sql, generate_load_data_sql,
    get_size_of_file, insert_csv_data, load_database_config, save_database_config, DatabaseState,
};
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() {
    let database_state: Arc<DatabaseState> = Arc::new(DatabaseState(Mutex::new(None)));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_http::init())
        .manage(database_state)
        .invoke_handler(tauri::generate_handler![
            connect_to_database,
            insert_csv_data,
            disconnect_from_database,
            save_database_config,
            load_database_config,
            get_size_of_file,
            generate_load_data_sql,
            execute_sql
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
