use leptos::*;
use leptos_router::*;

use crate::{
    components::header::Header,
    views::{about::About, init::InitView},
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="imgbgd"></div>
        <div id="imgbgdblur"></div>
        <Header/>
        <Router>
            <Routes>
                <Route path="/" view=InitView/>
                <Route path="/about" view=About/>
                <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </Router>
    }
}
