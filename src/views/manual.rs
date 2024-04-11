use crate::{
    components::button::Button,
    utils::js_value::{invoke, jsvalue_2_vec_str},
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn manual() -> impl IntoView {
    let params = use_params_map();

    let (qr_code, set_qr_code) = create_signal("".to_string());

    let (manual_imgs, set_manual_imgs) = create_signal(vec![] as Vec<String>);

    let (manual_name, _) =
        create_signal(params.with_untracked(|params| params.get("id").cloned().unwrap()));

    spawn_local(async move {
        let new_msg = invoke("list_docs").await;

        let mut data: Vec<String> = vec![];

        jsvalue_2_vec_str(new_msg)
            .into_iter()
            .for_each(|s| data.push(s));

        let index = data
            .iter()
            .position(|s| s.starts_with(manual_name.get().as_str()))
            .unwrap_or_default();

        set_qr_code.set(data[index + 1].clone());

        set_manual_imgs.set(
            data[index + 2]
                .split("//")
                .map(|s| s.to_string())
                .filter(|s| !s.eq(""))
                .collect::<Vec<String>>(),
        );
    });

    view! {
        <main class="h-full w-full flex justify-center scroll pt-8 text-gifsc text-center font-semibold">
            <div class="flex flex-col items-center h-full overflow-y-scroll">
                <h1 class="text-4xl pt-16">{manual_name.get().replace(".pdf", "")}</h1>
                    {move || html::div().inner_html(qr_code.get())}
                    {move || manual_imgs.get().iter().map(|path| {
                        view! {
                            <div class="mx-24">
                                <img src={path} alt="a"/>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
            </div>
        </main>
        <a class="absolute bottom-0 left-0 m-4" href="/about">
            <Button text="Voltar".to_string()></Button>
        </a>
    }
}
