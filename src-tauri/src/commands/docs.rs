use base64::{engine::general_purpose, Engine as _};
use qrcode_generator::QrCodeEcc;
use std::fs;
use urlencoding::encode;

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
            let qr_code = qrcode_generator::to_svg_to_string(
                [
                    "https://cdn.jsdelivr.net/gh/lalvesl/ifscRauCNCInterfaceManuals/",
                    &encode(&s),
                ]
                .join(""),
                QrCodeEcc::Low,
                300,
                None::<&str>,
            )
            .unwrap()
            .replace("fill=\"#FFF\"", "fill-opacity=\"0\"");
            let mut file_path = resource_path.clone();
            file_path.push(s.clone());
            let file_data = fs::read(file_path).unwrap_or_default();
            let base_str = general_purpose::STANDARD.encode(&file_data);
            vec![s, qr_code, base_str]
        })
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()

    // .into_iter().flatten().collect::<Vec<u8>>()
}
