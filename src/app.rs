use std::usize;

use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{
    convert::{FromWasmAbi, ReturnWasmAbi},
    prelude::*,
};

use crate::{components::header::Header, utils::js_value::*, views::init::InitView};

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
//     async fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    // spawn_local(async move {
    //     let new_msg = invoke("list_docs").await;
    //     jsvalue_2_vec_str(new_msg).iter().for_each(|d| consoller(d));
    // });

    view! {
        <div id="imgbgd"></div>
        <div id="imgbgdblur"></div>
        <Header/>
        <Router>
            <Routes>
                <Route path="" view=InitView/>
                <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </Router>
    }
}
