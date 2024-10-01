use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use ui_kit::typewritter::TypeWritterFX;

#[component]
pub fn Works() -> impl IntoView {
    view! {
        <Title text="Works record"/>
        <h1><TypeWritterFX words={vec!["Work library".to_string(), "Insight essays".to_string(), "Project publications".to_string()]}/></h1>
        <AnimatedOutlet
            outro=""
            intro=""
        />
    }
}