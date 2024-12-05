// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{GlobalShortcutManager, Manager};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn mine() -> String
{
    String::from("HEHEH")
}

fn main() {
    tauri::Builder::default()
//     .setup(|app| 
//     {
//         let mut shortcut_manager = app.global_shortcut_manager();
//         shortcut_manager
//         .register("Ctrl+Shift+A", move || {
//             println!("Ctrl+Shift+A detected!");
//         })
//         .unwrap();
    
//     Ok(())
// })
    .invoke_handler(tauri::generate_handler![greet,mine])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
