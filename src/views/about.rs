use crate::{
    components::button::Button,
    utils::js_value::{invoke, jsvalue_2_vec_str},
};
use leptos::*;
use urlencoding::encode;

#[component]
pub fn About() -> impl IntoView {
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

    view! {
        <main class="h-full w-full flex justify-center scroll pt-8 text-gifsc text-center font-semibold">
            <div class="flex flex-col">
                <div>
                    <h1 class="text-4xl pt-16" id="edital">Sobre O Projeto</h1>
                    <h1 class="text-2xl m-4" id="edital">{"Edital nº 02/2023/PROPPI - Universal"}</h1>
                </div>
                <div>
                    <h1 class="text-4xl pt-16" id="edital">Manuais</h1>
                    <div class="pt-2 flex flex-wrap justify-center">
                    {
                        move || manuals
                        .get()
                        .iter()
                        .map(|s| view!{<div class="m-4">
                                <a href={ ["/manual/", &encode(s)].join("") }>
                                    <Button text={s.to_string().replace(".pdf", "")} button_type={"thumb".to_string()}></Button>
                                </a>
                            </div>
                            })
                        .collect::<Vec<_>>()
                    }
                    </div>
                </div>

            </div>
        </main>
        <a class="absolute bottom-0 left-0 m-4" href="/">
            <Button text="Voltar".to_string()></Button>
        </a>
    }
}
