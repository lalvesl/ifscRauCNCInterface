use leptos::*;

use crate::components::header::Header;

#[component]
pub fn InitView() -> impl IntoView {
    view! {
        <Header/>
        <main class="init-main">
            <h1>IFSC-Rau CNC PCB Router</h1>
            <h2>Insira um pendrive</h2>
        </main>
    }
}
