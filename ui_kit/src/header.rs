use leptos::*;
use stylance::import_style as get_css;

get_css!(header_css, "./header.css");

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <p>"Header placeholder"</p>
    }
}