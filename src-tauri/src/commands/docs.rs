use std::fs;

#[tauri::command]
pub fn list_docs(handle: tauri::AppHandle) -> Vec<String> {
    let resource_path = handle
        .path_resolver()
        .resolve_resource("ifscRauCNCInterfaceManuals")
        .expect("failed to resolve resource");

    let entries = match fs::read_dir(&resource_path) {
        Ok(dir) => dir,
        Err(_) => panic!("readDir is broken!"),
    };

    entries
        .map(|f| match f {
            Ok(f) => f,
            Err(_) => panic!("readDir is broken!"),
        })
        .map(|fd| {
            fd.path()
                .display()
                .to_string()
                .split("/")
                .last()
                .unwrap()
                .to_string()
        })
        .filter(|s| s.contains(&".pdf"))
        .collect::<Vec<String>>()
}
