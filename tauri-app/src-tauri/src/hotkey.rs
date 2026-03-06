use tauri::{App, Manager};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

pub fn register_hotkey(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();

    app.global_shortcut().on_shortcut("Super+J", move |_app, _shortcut, _event| {
        if let Some(window) = app_handle.get_webview_window("main") {
            if window.is_visible().unwrap_or(false) {
                let _ = window.hide();
            } else {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    })?;

    Ok(())
}
