use std::fs;

#[tauri::command]
pub fn list_cnc_files() -> Vec<String> {
    let entries = match fs::read_dir("/media") {
        Ok(dir) => dir,
        Err(_) => panic!("readDir is broken!"),
    };

    entries
        .map(|path| path.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>()
}
