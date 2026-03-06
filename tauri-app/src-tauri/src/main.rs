// Pilot — AI Command Center for Ubuntu LTS
// Tauri v2 application entry point

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod hotkey;
mod tray;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.hide().unwrap();

            tray::setup_tray(app)?;
            hotkey::register_hotkey(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::toggle_window,
            commands::get_daemon_status,
            commands::send_to_daemon,
            commands::confirm_action,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Pilot");
}
