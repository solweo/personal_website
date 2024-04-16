use leptos::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1>"Personal website"</h1>
    }
}