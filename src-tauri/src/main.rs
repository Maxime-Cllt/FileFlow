#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benches;

use fileflow::fileflow::action::actions::DatabaseState;
use fileflow::fileflow::action::actions::*;
use fileflow::fileflow::action::database_command::get_table_list;
use fileflow::fileflow::action::database_command::*;
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
