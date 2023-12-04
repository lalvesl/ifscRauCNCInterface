// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            let entries = match fs::read_dir("./src") {
                Ok(f) => f,
                Err(_) => panic!("readDir is broken!"),
            };

            let fds = entries
                .map(|f| match f {
                    Ok(f) => f,
                    Err(_) => panic!("readDir is broken!"),
                })
                .map(|fd| fd.path().display().to_string());
            println!(
                "{:?}",
                fds.reduce(|buff: String, fd: String| buff + "," + &fd)
                    .unwrap()
            );
            // let mut fds = match entries.collect::<Result<Vec<fs::DirEntry>, io::Error>>() {
            //     Ok(fds) => for fd in fds{ fd.path()},
            //     Err(_) => panic!("Error on read dir"),
            // };
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
