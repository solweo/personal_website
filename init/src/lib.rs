#[cfg(feature = "ssr")]
pub mod fileserv;
#[cfg(feature = "ssr")]
pub mod handle_leptos;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use front::App;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
