use qrcode_generator::QrCodeEcc;
use std::fs;
use urlencoding::encode;

#[tauri::command]
pub fn list_docs(handle: tauri::AppHandle) -> Vec<String> {
    let resource_path = handle
        .path_resolver()
        .resolve_resource("manuals")
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
        .map(|s| {
            let mut file_path = resource_path.clone();
            file_path.push(s.clone());
            let file = fs::read(file_path).unwrap();
            let file_data = String::from_utf8_lossy(&file);
            let file_meta_data = file_data.split("\n").collect::<Vec<_>>();
            let doc_name = file_meta_data[0].to_string();
            let qr_code = qrcode_generator::to_svg_to_string(
                [
                    "https://cdn.jsdelivr.net/gh/lalvesl/ifscRauCNCInterfaceManuals/",
                    &encode(&doc_name),
                    ".pdf",
                ]
                .join(""),
                QrCodeEcc::Low,
                300,
                None::<&str>,
            )
            .unwrap()
            .replace("fill=\"#FFF\"", "fill-opacity=\"0\"");
            vec![
                doc_name,
                qr_code,
                file_meta_data[1..]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("//"),
            ]
        })
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()

    // .into_iter().flatten().collect::<Vec<u8>>()
}
