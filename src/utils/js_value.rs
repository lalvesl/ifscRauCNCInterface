use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[cfg(build = "debug")]
pub fn consoller(text: &str) {
    use serde_wasm_bindgen::to_value;
    use web_sys::console;
    console::log_1(&(to_value(&GreetArgs { name: text })).unwrap())
}

#[derive(Serialize, Deserialize)]
pub struct GreetArgs<'a> {
    name: &'a str,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str) -> JsValue;
}

pub fn jsvalue_2_vec_str(js_array: JsValue) -> Vec<String> {
    js_array
        .dyn_into::<js_sys::Array>()
        .unwrap()
        .iter()
        .map(|v| v.as_string().unwrap_or_else(|| String::from("")))
        .collect::<Vec<String>>()
}
