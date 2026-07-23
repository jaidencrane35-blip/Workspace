mod commands;

use std::sync::{Arc, Mutex};

use commands::health::get_workspace_health;
use commands::settings::{get_settings, update_settings};
use commands::status::get_workspace_status;
use tauri::Manager;
use workspace_kernel::WorkspaceKernel;

fn init_logging() {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .try_init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();
    log::info!("workspace application starting");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_workspace_status,
            get_workspace_health,
            get_settings,
            update_settings,
        ])
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");
            let db_path = app_data_dir.join("workspace.db");

            let kernel = WorkspaceKernel::initialize(db_path).unwrap_or_else(|error| {
                log::error!("workspace kernel initialization failed: {error}");
                panic!("failed to initialize workspace kernel");
            });

            app.manage(Arc::new(Mutex::new(kernel)));
            log::info!("workspace application setup complete");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
