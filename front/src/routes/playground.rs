use leptos::*;
use leptos_meta::*;
use server::api::ErrorOn;
use std::time::Duration;
use addons::{AnimatedSuspense, AnimatedBoundary};
use crate::index;
use ui_kit::{style_baseline, Article};

#[component]
pub fn playground() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    
    let async_foo = create_resource(count, server::api::fetch_foo);
    let async_article = create_resource(count, |_| server::api::fetch_article("test_article".to_string()));

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

        <Article/>
        
        // <AnimatedBoundary
        //     value=async_article
        //     intro=index::fadeIn
        //     outro=index::fadeOut
        //     fallback_intro=index::fadeIn
        //     fallback_outro=index::fadeOut
        //     delay=Duration::from_millis(250)
        //     suspense_fallback=move || view! { <p>"Loading..."</p> }
        //     error_fallback=move |v| {
        //         if let ErrorOn::FetchArticle(err) = ErrorOn::from(v) {
        //             match err {
        //                 server::api::fetch_article::Error::InvalidId => view! {
        //                     <h3>"An invalid article ID was sent to the server"</h3>
        //                     <button on:click=on_click>"Try again"</button>
        //                 }.into_view(),
        //                 server::api::fetch_article::Error::NotFound => view! {
        //                     <h3>"Server couldn't find the article by the specified ID"</h3>
        //                     <button on:click=on_click>"Try again"</button>
        //                 }.into_view(),
        //                 server::api::fetch_article::Error::FailedToParse => view! {
        //                     <h3>"There is something faulty with the article, server failed to handle it"</h3>
        //                     <button on:click=on_click>"Try again"</button>
        //                 }.into_view(),
        //             }
        //         } else {
        //             view! {
        //                 <h3>"An error occurred that is NOT related to fetching Article"</h3>
        //                 <button on:click=on_click>"Try again"</button>
        //             }.into_view()
        //         }
        //     }
        //     let:data
        // >
        //     <h3>"Here the article: "</h3>
        //     <div inner_html={data.content}></div>
        // </AnimatedBoundary>
    }
}