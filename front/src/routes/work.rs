use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use ui_kit::Article;

#[component]
pub fn Work() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    view! {
        <Title text="Work page"/>
        <Article id={id()}/>
    }
}