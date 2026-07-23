mod commands;

use std::sync::{Arc, Mutex};

use commands::settings::{get_settings, update_settings};
use commands::status::get_workspace_status;
use tauri::Manager;
use workspace_kernel::WorkspaceKernel;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_workspace_status,
            get_settings,
            update_settings,
        ])
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");
            let db_path = app_data_dir.join("workspace.db");

            let kernel =
                WorkspaceKernel::initialize(db_path).expect("failed to initialize workspace kernel");

            app.manage(Arc::new(Mutex::new(kernel)));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
