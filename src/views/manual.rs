use crate::{
    components::button::Button,
    utils::js_value::{consoller, invoke, jsvalue_2_vec_str},
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Manual() -> impl IntoView {
    let (manuals, set_manuals) = create_signal::<Vec<String>>(Vec::<String>::with_capacity(100));
    // manuals.set(vec!["123".to_string(), "321".to_string()]);
    spawn_local(async move {
        let new_msg = invoke("list_docs").await;
        // jsvalue_2_vec_str(new_msg).iter().for_each(|d| consoller(d));
        set_manuals.update(|old_vec| {
            jsvalue_2_vec_str(new_msg)
                .into_iter()
                .for_each(|s| old_vec.push(s));
        })
    });
    
    let params = use_params_map();
    let manual = params.with(|params| params.get("id").cloned().unwrap_or_default());

    consoller(manual.as_str());
    view! {
        <main class="h-full w-full flex justify-center scroll pt-8 text-gifsc text-center font-semibold">
            <div class="flex flex-col">
                <h1 class="text-4xl pt-16">{manual.replace(".pdf", "")}</h1>
            </div>
        </main>
        <a class="absolute bottom-0 left-0 m-4" href="/about">
            <Button text="Voltar".to_string()></Button>
        </a>
    }
}
