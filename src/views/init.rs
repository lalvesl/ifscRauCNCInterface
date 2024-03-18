use crate::components::button::Button;
use leptos::*;

#[component]
pub fn InitView() -> impl IntoView {
    view! {
        <div class="text-gifsc text-center h-full flex flex-col items-center justify-center">
            <h1 class="my-2 text-5xl font-bold">Fresadora CNC para PCIs</h1>
            <h2 class="my-2 text-3xl font-semibold">Insira um pendrive</h2>
        </div>
        <a href="/about" class="absolute bottom-0 right-0 m-4">
            <Button text="Ajuda & Sobre".to_string()/>
        </a>
    }
}
