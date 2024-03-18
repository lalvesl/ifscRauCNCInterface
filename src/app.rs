use std::usize;

use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{
    convert::{FromWasmAbi, ReturnWasmAbi},
    prelude::*,
};

use crate::{components::header::Header, utils::js_value::*, views::{about::About, init::InitView}};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="imgbgd"></div>
        <div id="imgbgdblur"></div>
        <Header/>
        <Router>
            <Routes>
                <Route path="/" view=InitView/>
                <Route path="/about" view=About/>
                <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </Router>
    }
}
