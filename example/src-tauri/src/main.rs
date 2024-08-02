// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalPosition, Manager};
use tauri_plugin_trafficlights_positioner::WindowExt;

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            if let Some(window) = app.get_window("main") {
                #[cfg(target_os = "macos")]
                // NOTE: Make sure you only call this ONCE per window.
                let _ = window.setup_traffic_lights_inset(LogicalPosition::new(20.0, 24.0));
            };

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
