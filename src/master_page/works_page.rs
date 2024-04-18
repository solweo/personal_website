use leptos::*;
// use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn WorksList() -> impl IntoView {
    view! {
        <h3>"Works listing page placeholder"</h3>
        <div>
            <A href="demo1">"Demo 1"</A>
            <A href="demo2">"Demo 2"</A>
        </div>
        <Outlet/>
    }
}

#[component]
pub fn WorkInfo() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "demo1" => "Demo 1",
        "demo2" => "Demo 2",
        _ => "Work not found",
    };

    view! {
        <h3>{name}</h3>
        <h3>"Work article page placeholder"</h3>
        <Outlet/>
    }
}