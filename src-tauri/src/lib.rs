pub mod commands;
pub mod models;
// pub mod utils;
// pub mod database;

// use crate::database::connection::get_migrations;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .plugin(
        //     tauri_plugin_sql::Builder::default()
        //         .add_migrations("sqlite:preservation.db", get_migrations())
        //         .build(),
        // )
        .invoke_handler(tauri::generate_handler![
            commands::preservation_simple::archive_project,
            commands::preservation_simple::create_bagit_package,
            commands::preservation_simple::get_archived_projects,
            commands::preservation_simple::quarantine_project,
            commands::preservation_simple::restore_project,
            commands::preservation_simple::scan_vault_integrity,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application")
}