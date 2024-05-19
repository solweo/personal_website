use crate::error_page::{AppError, DisplayError};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::logging::log;

pub mod error_page;
pub mod master_pages;

use ui_kit::widgets::*;
use master_pages::*;

use server::{api::article::*, *};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/ui_kit.css"/>
        <link rel="stylesheet" id="leptos" href="/front.css"/>

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
                    <Route path="/playground" view=Playground/>
                </AnimatedRoutes>
            </main>
        </Router>
    }
}

#[component]
pub fn Playground() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let res = create_resource(count, fetch_foo);
    let art = create_resource(count, fetch_article);

    view! {
        <h3>"UI kit preview"</h3>
        <button on:click=on_click>"Click Me: " {count}</button>

        <Suspense
            fallback = move || view! { <h3>"Loading"</h3> }
        >
            <ErrorBoundary fallback=|errors| {
                view! {<h3>"An error occurred while fetching data"</h3>}
            }>
                {
                    log!("Resource: {:?}", res);
                    move || {
                    res.get().map(|r|
                        r.map(|v| view! {
                            <h3>"Here the data: "{v}</h3>
                        }.into_view())
                    )
                }}
            </ErrorBoundary>
        </Suspense>

        <br/>

        <Suspense
            fallback = move || view! { <h3>"Articel is Loading"</h3> }
        >
            <ErrorBoundary fallback=|errors| {
                view! {<h3>"An error occurred while fetching Article"</h3>}
            }>
                {
                    move || {
                    art.get().map(|r|
                        r.map(|v| view! {
                            <h3>"Id: "{v.id}</h3>
                            <h3>"Title: "{v.title}</h3>
                            <h3>"Text:"</h3>
                            <div inner_html=v.content></div>
                        }.into_view())
                    )
                }}
            </ErrorBoundary>
        </Suspense>

        <br/>
        <Preview/>
        <LoremImpus/>
        <AnimatedOutlet/>
    }
}