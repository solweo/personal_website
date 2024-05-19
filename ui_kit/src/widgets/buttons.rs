use leptos::*;
use stylance::import_style as get_css;

get_css!(css, "./buttons.css");

#[component]
pub fn NeumorphicButton() -> impl IntoView {
    view! {
        <div class="toggle">
            <input type="checkbox"/>
            <span class="button"></span>
            <span class="label">+</span>
        </div>
    }
}