mod commands;

use commands::status::get_workspace_status;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_workspace_status])
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");
            let db_path = app_data_dir.join("workspace.db");

            let db = workspace_database::Database::open(db_path)
                .expect("failed to open workspace database");
            let runner = workspace_database::MigrationRunner::new();
            runner
                .apply_all(&db)
                .expect("failed to apply database migrations");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
