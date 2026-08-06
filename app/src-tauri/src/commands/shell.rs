//! Shell lifecycle — explicit Exit only (Zero-Trap: window close collapses).

use tauri::AppHandle;

/// Terminate the Workspace process after an explicit Exit Workspace action.
#[tauri::command]
pub fn exit_workspace(app: AppHandle) {
    log::info!("exit_workspace: explicit user exit");
    app.exit(0);
}
