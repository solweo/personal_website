use leptos::*;
use stylance::import_style as get_css;

use super::WidgetState;

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

#[component]
pub fn PrimaryButton() -> impl IntoView {
    view! {
        <button class=css::primary_button_initial>"Button text"</button>
    }
}

#[component]
pub fn SecondaryButton() -> impl IntoView {
    view! {
        <button class=css::secondary_button_initial>"Button text"</button>
    }
}

#[component]
pub fn FlatButton() -> impl IntoView {
    view! {
        <button class=css::flat_button_initial>"Button text"</button>
    }
}

#[component]
pub fn IconButton() -> impl IntoView {
    view! {
        <button class=css::icon_button_initial>"Icon placeholder"</button>
    }
}