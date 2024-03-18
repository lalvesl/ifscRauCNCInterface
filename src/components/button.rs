use leptos::*;

#[component]
pub fn Button(
    #[prop(default = "Click".to_string())] text: String,
    #[prop(default = "default".to_string())] button_type: String,
) -> impl IntoView {
    let common_class = "rounded text-2xl p-4 hover:outline hover:outline-gifsc";

    match button_type.as_str() {
        "default" => {
            view! {<button type="button" class={[common_class, "bg-gifsc text-white"].join(" ")}>{text}</button>}
        }
        "thumb" => {
            view! {<button type="button" class={[common_class,"bg-gifsc bg-opacity-5 outline-black outline-opacity-50 border-2  text-gifsc"].join(" ")}>{text}</button>}
        }
        _ => view! {<button>error</button>},
    }

    // view! {<button type="button" class="bg-gifsc rounded text-white text-2xl p-4 hover:outline outline-gifsc">{text}</button>}
}
