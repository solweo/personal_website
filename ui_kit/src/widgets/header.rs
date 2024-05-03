use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use stylance::import_style as get_css;

use super::WidgetState;

get_css!(css, "./header.css");

#[component]
pub fn Header() -> impl IntoView {
    let (menu_opened, set_menu_opened) = create_signal(false);

    view! {
        <header 
            class=css::bottom_floating_header
        >
            <Show
                when = move || !menu_opened()
                fallback = || view! { 
                    <button>"CLOSE"</button>
                    <Menu/>
                }
            >
                <A href="/">"SOLWEO"</A>
                <nav>
                    <button
                        on:click = move |_| set_menu_opened.update(|v| *v = true)
                    >"MENU"</button>
                    <h3>"|"</h3>
                    <A href="/works">"WORKS"</A>
                </nav>
            </Show>
        </header>
    }
}

#[component]
fn Menu() -> impl IntoView {
    view! {
        <nav class=css::header_pop_up_menu>
            <A href="/">"Home"</A>
            <A href="/about">"About"</A>
            <A href="/works">"Works"</A>
            <A href="/contact">"Contact"</A>
        </nav>
    }
}