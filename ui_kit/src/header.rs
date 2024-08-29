use leptos::*;
use stylance::import_style as get_css;

get_css!(tagline_css, "./header.css");

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <h3>"Header placeholder"</h3>
    }
}