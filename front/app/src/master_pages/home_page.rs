use leptos::*;
use leptos_router::*;
use ui_kit::index;

pub const BRAND_KEY_MESSAGE: &str = "Embark the realms of tech artistry, art sorcery and product wizardy - thus unleashing your ingenuity!";

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class=index::large_context>
            <h1 class=index::title>{BRAND_KEY_MESSAGE}</h1>
        </div>
        <AnimatedOutlet/>
    }
}