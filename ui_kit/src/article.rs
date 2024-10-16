use leptos::*;
use server::api::ErrorOn;
use std::time::Duration;
use addons::AnimatedBoundary;
use crate::style_baseline;
use crate::typewritter::TypeWritterFX;
use stylance::import_style as get_css;

get_css!(article_css, "./article.css");

#[component]
pub fn Article(
    id: String,
) -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let async_article = create_resource(count, move |_| server::api::fetch_article(id.clone()));
    
    view! {
        <AnimatedBoundary
            value=async_article
            intro=style_baseline::fadeIn
            outro=style_baseline::fadeOut
            fallback_intro=style_baseline::fadeIn
            fallback_outro=style_baseline::fadeOut
            delay=Duration::from_millis(250)
            suspense_fallback=move || view! { <p>"Loading..."</p> }
            error_fallback=move |v| {
                if let ErrorOn::FetchArticle(err) = ErrorOn::from(v) {
                    match err {
                        server::api::fetch_article::Error::InvalidId => view! {
                            <h3>"An invalid article ID was sent to the server"</h3>
                            <button on:click=on_click>"Try again"</button>
                        }.into_view(),
                        server::api::fetch_article::Error::NotFound => view! {
                            <h3>"Server couldn't find the article by the specified ID"</h3>
                            <button on:click=on_click>"Try again"</button>
                        }.into_view(),
                        server::api::fetch_article::Error::FailedToParse => view! {
                            <h3>"There is something faulty with the article, server failed to handle it"</h3>
                            <button on:click=on_click>"Try again"</button>
                        }.into_view(),
                    }
                } else {
                    view! {
                        <h3>"An error occurred that is NOT related to fetching Article"</h3>
                        <button on:click=on_click>"Try again"</button>
                    }.into_view()
                }
            }
            let:data
        >
            <div class=article_css::article_container>
                <div class=article_css::aliases_container>
                    <TypeWritterFX words={data.aliases}/>
                </div>
                <div class=article_css::preview_and_description>
                    <img src={data.preview} class=style_baseline::square_squirrel_mask alt="Aricle's preview image"/>
                    <h1>{data.description}</h1>
                </div>
                <div class=article_css::article_itself inner_html={data.content}></div>
            </div>
        </AnimatedBoundary>
    }
}