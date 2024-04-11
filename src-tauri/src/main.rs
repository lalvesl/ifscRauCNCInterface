// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::{cnc_code, docs};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            docs::list_docs,
            cnc_code::list_cnc_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
