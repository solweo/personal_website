use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use ui_kit::{typewritter::TypeWritterFX, WorksFeed};

#[component]
pub fn Works() -> impl IntoView {
    view! {
        <Title text="Works record"/>
        <WorksFeed/>
    }
}