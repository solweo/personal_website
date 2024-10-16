use leptos::*;
use leptos_meta::*;
use ui_kit::WorksFeed;

#[component]
pub fn Works() -> impl IntoView {
    view! {
        <Title text="Works record"/>
        <WorksFeed/>
    }
}