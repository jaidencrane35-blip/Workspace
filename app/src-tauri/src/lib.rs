mod actor;
mod commands;

use std::sync::{Arc, Mutex};

use commands::audit::get_audit_history;
use commands::layout::{
    create_layout, delete_layout, get_layout, get_layout_snapshot, reset_layout, update_layout,
};
use commands::analytics::get_workspace_metrics;
use commands::context::get_workspace_context;
use commands::desktop_window::get_desktop_windows;
use commands::discovery::get_actor_capabilities;
use commands::observation::get_observations;
use commands::projection::get_workspace_snapshot;
use commands::execution::{
    create_suggestion_intent_request, execute_intent_request, get_execution_outcomes,
    get_execution_state, get_execution_states, request_execution_cancellation,
};
use commands::suggestion::{accept_suggestion, get_suggestions, reject_suggestion};
use commands::suggestion_lifecycle::get_suggestion_lifecycle;
use commands::resources::{
    create_application, create_widget, create_zone, delete_application, delete_widget,
    delete_zone, get_application, get_widget, get_zone, launch_application,
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
            // Runtime / settings
            get_workspace_status,
            get_workspace_health,
            get_settings,
            update_settings,
            // Product shell (Canvas + Diagnostic)
            create_workspace,
            get_workspace,
            create_zone,
            create_layout,
            update_layout,
            get_layout,
            get_workspace_context,
            get_suggestions,
            accept_suggestion,
            reject_suggestion,
            get_suggestion_lifecycle,
            get_desktop_windows,
            create_application,
            launch_application,
            create_suggestion_intent_request,
            execute_intent_request,
            get_execution_outcomes,
            get_execution_states,
            request_execution_cancellation,
            // Registered for CommandHandler parity / future UI — unused by React today
            // (see docs/03-Engineering/IPC-SURFACE.md)
            delete_zone,
            get_zone,
            delete_application,
            get_application,
            create_widget,
            delete_widget,
            get_widget,
            delete_layout,
            reset_layout,
            get_layout_snapshot,
            get_workspace_snapshot,
            get_actor_capabilities,
            get_audit_history,
            get_observations,
            get_workspace_metrics,
            get_execution_state,
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
