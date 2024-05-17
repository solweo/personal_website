use crate::error_page::{AppError, DisplayError};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_page;
pub mod master_pages;

use ui_kit::widgets::*;
use master_pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        <Title text="Solweo"/>

        <Meta
            name="description"
            content="Hands-on experience on topics ranging from design to development from Adrian Alekseev aka Solweo"
        />

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
                    <Route path="/works" view=|| view! { <WorksList/> }/>
                    <Route path="/works/:id" view=|| view! { <ExactWork/> }/>
                    <Route path="/ui-kit-preview" view=UiKitPreview/>
                </AnimatedRoutes>
            </main>
        </Router>
    }
}

#[component]
pub fn UiKitPreview() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h3>"UI kit preview"</h3>
        <button on:click=on_click>"Click Me: " {count}</button>
        <Preview/>
        <LoremImpus/>
        <AnimatedOutlet/>
    }
}