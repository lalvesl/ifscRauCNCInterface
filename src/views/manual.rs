use crate::{
    components::button::Button,
    utils::js_value::{consoller, invoke, jsvalue_2_vec_str},
};
use leptos::{html, *};
use leptos_router::*;

#[component]
pub fn Manual() -> impl IntoView {
    let (manuals, set_manuals) = create_signal(vec![] as Vec<String>);
    let params = use_params_map();
    let manual = params.with_untracked(|params| params.get("id").cloned().unwrap_or_default());
    let manual_ = manual.clone();
    let (index, set_index) = create_signal(0 as usize);
    spawn_local(async move {
        let new_msg = invoke("list_docs").await;
        // jsvalue_2_vec_str(new_msg).iter().for_each(|d| consoller(d));
        set_manuals.update(|old_vec| {
            jsvalue_2_vec_str(new_msg)
                .into_iter()
                .for_each(|s| old_vec.push(s));
        });
        set_index.set(
            manuals
                .get_untracked()
                .iter()
                .position(|s| s.ends_with(manual_.as_str()))
                .unwrap_or_default() as usize,
        );
    });
    consoller(manual.as_str());
    let on_error = String::new();
    let on_error_ = String::new();

    let document_id = "manual_pdf";

    view! {
        <main class="h-full w-full flex justify-center scroll pt-8 text-gifsc text-center font-semibold">
            <div class="flex flex-col items-center h-full overflow-y-scroll">
                <h1 class="text-4xl pt-16">{manual.replace(".pdf", "")}</h1>
                    {  move || html::div().inner_html(manuals.get().get(index.get()+1).unwrap_or_else(||&on_error).clone())}
                    <iframe 
                    src=move || ["data:application/pdf;base64,", &manuals.get().get(index.get()+2).unwrap_or_else(||&on_error_).clone()].join("")
                    id="manual_pdf"
                    height="100%" width="100%">
                    </iframe>
                    <script>
                        iframe = document.getElementById("manual_pdf");
                        {"//setTimeout(()=>{
                            // iframe.contentDocument || iframe.contentWindow.document;
                            // console.log(iframe.body.scrollHeight);
                            setTimeout(()=>{
                                console.log(document.getElementById('viewer').scrollHeight);
                                iframe.style.height = document.getElementById('viewer').scrollHeight;
                            }, 10000);"} 
                    </script>
                    // {
                    //     move || html::iframe()
                    //         .set_src(&["data:application/pdf;base64,", &manuals.get().get(index.get()+2).unwrap_or_else(||&on_error_).clone()].join(""))
                    // }
            </div>
        </main>
        <a class="absolute bottom-0 left-0 m-4" href="/about">
            <Button text="Voltar".to_string()></Button>
        </a>
    }
}
