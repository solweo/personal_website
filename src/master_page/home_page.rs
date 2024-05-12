use leptos::*;
// use leptos_meta::*;
use leptos_router::*;

use stylance::import_style as get_css;

use super::super::BRAND_KEY_MESSAGE as BKM;

get_css!(index, "../../ui_kit/src/index.css");

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class=index::large_context>
            <h1 class=index::title>{BKM}</h1>
        </div>
        <Outlet/>
    }
}