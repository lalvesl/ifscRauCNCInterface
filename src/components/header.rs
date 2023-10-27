use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="container">
            <div class="row">
                <img src="public/ifsc-vWithoutCampus.png" class="logo tauri" alt="Tauri logo"/>
            </div>
        </header>
    }
}
