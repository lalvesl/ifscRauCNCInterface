use std::usize;

use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{
    convert::{FromWasmAbi, ReturnWasmAbi},
    prelude::*,
};
use web_sys::console;

pub fn consoller(text: &str) {
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
    let array = js_array.dyn_into::<js_sys::Array>().unwrap();

    let mut r = Vec::with_capacity(array.length() as usize);
    for i in 0..array.length() {
        let js_value = array.get(i);
        if let Some(js_string) = js_value.as_string() {
            r.push(js_string);
        }
    }
    r
}
