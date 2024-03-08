use std::fs;

#[tauri::command]
pub fn list_docs(handle: tauri::AppHandle) -> String {
    let resource_path = handle
        .path_resolver()
        .resolve_resource("ifscRauCNCInterfaceManuals")
        .expect("failed to resolve resource");

    let entries = match fs::read_dir(&resource_path) {
        Ok(dir) => dir,
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

    // let file = std::fs::File::open(&resource_path).unwrap();
    // let lang_de: serde_json::Value = serde_json::from_reader(file).unwrap();

    // lang_de.get("hello").unwrap()
    String::from("123")
}
