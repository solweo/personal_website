pub mod error_page;
pub mod routes;

use error_page::{AppError, DisplayError};
use routes::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use stylance::import_style as get_css;
get_css!(pub index, "./index.css");

// use ui_kit::style_baseline;
use ui_kit::Header;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        
        <Stylesheet id="leptos" href="/ui_kit.css"/>
        <link id="leptos" rel="stylesheet" href="/front.css"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <DisplayError outside_errors/> }.into_view()
        }>
            <Header/>
            <h1>"Personal website"</h1>
            <main>
                <AnimatedRoutes
                    outro="slideOut"
                    intro="slideIn"
                    outro_back="slideOutBack"
                    intro_back="slideInBack"
                >
                    <Route path="/" view=Home/>
                    <Route path="/about" view=About/>
                    <Route path="/works" view=|| view! { <Works/> }/>
                    <Route path="/works/:id" view=|| view! { <Work/> }/>
                    <Route path="/playground" view=Playground/>
                </AnimatedRoutes>
            </main>
        </Router>
    }
}