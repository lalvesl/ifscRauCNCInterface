use leptos::*;

#[component]
pub fn InitView() -> impl IntoView {
    view! {
        <main class="init-main">
            <h1>Fresadora CNC para PCIs</h1>
            <h2>Insira um pendrive</h2>
        </main>
        <h3 id="edital">Edital nº 02/2023/PROPPI - Universal</h3>
    }
}
