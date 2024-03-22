use qrcode_generator::QrCodeEcc;
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
        .filter(|s| s.ends_with(&".pdf"))
        .map(|s| {
            let qr_code =
                qrcode_generator::to_svg_to_string(&s, QrCodeEcc::Low, 1024, None::<&str>).unwrap();
            vec![s, qr_code]
        })
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()

    // .into_iter().flatten().collect::<Vec<u8>>()
}
