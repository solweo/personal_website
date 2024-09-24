use leptos::*;
use leptos_meta::*;
use ui_kit::Tagline;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Solweo's home page"/>
        <h3>"Home page placeholder"</h3>
        <Tagline/>
    }
}