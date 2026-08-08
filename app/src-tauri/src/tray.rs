//! P16.PR2 / Gate E1 — tray presence that serves Conversation (Product Gravity).
//! P19.S1 — Show Conversation restores ShellMode Form B, not only the native window.
//! Minimal menu: restore Conversation, Exit Workspace. No catalogue chrome.

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Runtime,
};

/// Event frontend listens for to restore React ShellMode Conversation Form.
pub const SHOW_CONVERSATION_EVENT: &str = "workspace-show-conversation";

/// Bring Conversation forward — same intent as single-instance secondary launch.
/// Emits ShellMode restore first; native show is a fallback if the webview is slow.
pub fn show_conversation<R: Runtime>(app: &AppHandle<R>) {
    // P19.S1: lifecycle transition only — frontend owns ShellMode → Conversation Form.
    if let Err(err) = app.emit(SHOW_CONVERSATION_EVENT, ()) {
        log::warn!("tray: emit {SHOW_CONVERSATION_EVENT} failed: {err}");
    } else {
        log::info!("tray: emitted {SHOW_CONVERSATION_EVENT} (ShellMode restore)");
    }

    if let Some(main) = app.get_webview_window("main") {
        let _ = main.unminimize();
        let _ = main.show();
        let _ = main.set_focus();
        log::info!("tray: showed Conversation (main)");
        return;
    }
    if let Some(operator) = app.get_webview_window("operator") {
        let _ = operator.unminimize();
        let _ = operator.show();
        let _ = operator.set_focus();
        log::info!("tray: showed operator (main unavailable)");
    }
}

/// Install tray icon after kernel setup. Failure is logged; app continues window-only.
pub fn install_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, "show_conversation", "Show Conversation", true, None::<&str>)?;
    let exit = MenuItem::with_id(app, "exit_workspace", "Exit Workspace", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &exit])?;

    let icon = app
        .default_window_icon()
        .cloned()
        .ok_or("default window icon missing for tray")?;

    let _tray = TrayIconBuilder::with_id("workspace-tray")
        .icon(icon)
        .tooltip("Workspace")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show_conversation" => show_conversation(app),
            "exit_workspace" => {
                log::info!("tray: Exit Workspace");
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_conversation(tray.app_handle());
            }
        })
        .build(app)?;

    log::info!("tray: Workspace tray icon installed");
    Ok(())
}
