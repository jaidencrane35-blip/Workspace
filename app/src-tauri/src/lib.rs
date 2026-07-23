mod actor;
mod commands;

use std::sync::{Arc, Mutex};

use commands::audit::get_audit_history;
use commands::resources::{
    create_application, create_widget, create_zone, delete_application, delete_widget,
    delete_zone, get_application, get_widget, get_zone,
};
use commands::workspace::{create_workspace, get_workspace};
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
            create_workspace,
            get_workspace,
            create_zone,
            delete_zone,
            get_zone,
            create_application,
            delete_application,
            get_application,
            create_widget,
            delete_widget,
            get_widget,
            get_audit_history,
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
