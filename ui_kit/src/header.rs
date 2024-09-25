use leptos::*;
use leptos_router::*;
use stylance::import_style as get_css;

get_css!(header_css, "./header.css");

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header 
            class=header_css::bottom_floating_header 
        >
            <A 
                class=header_css::header_nav
                href="/"
            >"Solweo"</A>
            <A 
                class=header_css::header_nav
                href="/about"
            >"About"</A>
            <A 
                class=header_css::header_nav
                href="/works"
            >"Works"</A>
            <A 
                class=header_css::header_nav
                href="/playground"
            >"Playground"</A>
            <A 
                class=header_css::header_nav
                href="/"
            >"Contact"</A>
        </header>
    }
}