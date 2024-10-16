use leptos::*;
use leptos_meta::*;
use server::api::ErrorOn;
use std::time::Duration;
use addons::AnimatedBoundary;
// use crate::index;
use ui_kit::{style_baseline, Article};

#[component]
pub fn playground() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    
    let async_foo = create_resource(count, server::api::fetch_foo);

    view! {
        <Title text="Playground for code experimentation"/>

        <h3>"Playground page placeholder"</h3>

        <button on:click=on_click>"Refresh anchored ones. Already: "{count}"th time"</button>

        <AnimatedBoundary
            value=async_foo
            intro=style_baseline::fadeIn
            outro=style_baseline::fadeOut
            fallback_intro=style_baseline::fadeIn
            fallback_outro=style_baseline::fadeOut
            delay=Duration::from_millis(250)
            suspense_fallback=move || view! { <p>"Loading..."</p> }
            error_fallback=move |v| {
                if let ErrorOn::FetchFoo(err) = ErrorOn::from(v) {
                    match err {
                        server::api::fetch_foo::Error::DummyServerError => view! {
                            <h3>"Dummy server error was invoked"</h3>
                            <button on:click=on_click>"Try again"</button>
                        }.into_view(),
                    }
                } else {
                    view! {
                        <h3>"An error occurred that is NOT related to fetching Foo"</h3>
                        <button on:click=on_click>"Try again"</button>
                    }.into_view()
                }
            }
            let:data
        >
            <p>"Retrived data: "{data}</p>
        </AnimatedBoundary>

        <Article id={"test_article".to_string()}/>
    }
}