use leptos::*;
// use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <h3>"Contacts page placeholder"</h3>
        <h3>"Contacts:"</h3>
        <A href="https://t.me/solweo">"Telegram: @solweo"</A>
        <A href="mailto:adrian@solweo.tech">"Mail: adrian@solweo.tech"</A>
        <Outlet/>
    }
}