pub mod error_page;
pub mod master_pages;
pub mod animated_show_switch;
pub mod animated_suspense;
pub mod delayed_value;

use std::time::Duration;
use std::rc::Rc;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::logging::log as client_log;
use ui_kit::widgets::*;
use server::{api::*, error::ErrorOn};

use crate::{
    error_page::{AppError, DisplayError},
    animated_show_switch::AnimatedShowSwitch,
    animated_suspense::AnimatedSuspense,
    delayed_value::DelayedValue,
    master_pages::*,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/ui_kit.css"/>
        <link id="leptos" rel="stylesheet" href="/front.css"/>

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
    let on_click = move |_| set_count.update(|count| *count += 1); // <- res.refetch()

    let async_data = create_resource(count, fetch_boo);

    let count_delayer_container = DelayedValue::default();
    let delayed_count = count_delayer_container.queue(Duration::from_millis(1000), Rc::new(count));

    let actual_data = create_rw_signal(String::new());
    let delayed_actual_data_container = DelayedValue::default();
    let delayed_actual_data = delayed_actual_data_container.queue(Duration::from_millis(250), Rc::new(actual_data));
    
    view! {
        <h3>"UI kit preview"</h3>
        <button on:click=on_click>"Click Me: " {count}</button>

        <h1>"Count: " {count}</h1>
        <h1>"Delayed count: " {delayed_count}</h1>

        <AnimatedSuspense
            intro="fadeIn"
            outro="fadeOut"
            fallback_intro="fadeIn"
            fallback_outro="fadeOut"
            delay=Duration::from_millis(250)
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <h5>"Retrived Data"</h5>
            {move || {
                async_data.and_then(|d| {
                    actual_data.set(d.clone());
                    view! { <p>"A Data: "{delayed_actual_data}</p> }
                })
            }}
            // {move || {
            //     async_data.get()
            //         .map(|a| view! { <p>"A Data: "{a}</p> })
            // }}
        </AnimatedSuspense>

        // <AnimatedShowSwitch
        //     when=async_data.loading()
        //     intro="fadeIn"
        //     outro="fadeOut"
        //     fallback_intro="fadeIn"
        //     fallback_outro="fadeOut"
        //     delay=Duration::from_millis(250)
        //     fallback=move || view! { 
        //         // <Suspense>
        //             <h5>"Retrived Data"</h5>
        //             {move || {
        //                 async_data.get()
        //                     .map(|a| view! { <p>"A Data: "{a}</p> })
        //             }}
        //         // </Suspense>
        //         // <h5>"Retrived Data"</h5>
        //     }
        // >
        //     <p>"Loading..."</p>
        // </AnimatedShowSwitch>

        // <br/>
        <Preview/>
        // <LoremImpus/>
        <AnimatedOutlet/>
    }
}