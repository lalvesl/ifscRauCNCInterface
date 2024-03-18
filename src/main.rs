mod app;
mod components;
mod views;
mod utils;

use app::App;

use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
