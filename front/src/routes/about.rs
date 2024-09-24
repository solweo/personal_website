use leptos::*;
use leptos_meta::*;
use ui_kit::Article;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="Article about me"/>
        <Article id="about_article"/>
    }
}