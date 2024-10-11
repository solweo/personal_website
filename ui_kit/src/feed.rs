use html::Title;
use leptos::*;
use leptos_use::*;
use std::time::Duration;
use itertools::Itertools;
use server::api::{ErrorOn, fetch_preview::ArticlePreview};
use addons::AnimatedBoundary;
use crate::style_baseline;
use stylance::import_style as get_css;

get_css!(feed_css, "./feed.css");

#[component]
pub fn WorksFeed() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    
    let async_article = create_resource(count, |_| server::api::fetch_preview::fetch_preview());

    view! {
        <AnimatedBoundary
            value=async_article
            intro=style_baseline::fadeIn
            outro=style_baseline::fadeOut
            fallback_intro=style_baseline::fadeIn
            fallback_outro=style_baseline::fadeOut
            delay=Duration::from_millis(250)
            suspense_fallback=move || view! { <p>"Loading..."</p> }
            error_fallback=move |_| {
                view! {
                    <h3>"Some error occurred and I don't really care which at that moment"</h3>
                    <button on:click=on_click>"Try again"</button>
                }.into_view()
            }
            let:data
        >
            <FeedContainer articles=data/>
        </AnimatedBoundary>
    }
}

#[component]
fn FeedContainer(articles: Vec<ArticlePreview>) -> impl IntoView {
    let titles_visibility = store_value(Vec::new());
    
    let title_tiles = articles.iter()
        .rev()
        .map(|article| {
            let (id, alias) = (&article.id, &article.aliases[0]);
            let obfuscated_id = match id.len() > 5 {
                true => &format!("{}…{}", &id[..2], &id[id.len() - 2..]),
                false => id,
            };

            let el = create_node_ref::<leptos::html::Div>();
            titles_visibility.update_value(|v| v.push((
                id.clone(), 
                use_element_visibility(el)
            )));

            view! {
                <div class=feed_css::work_tile node_ref=el>
                    <p>{obfuscated_id}</p>
                    <h2>{alias}</h2>
                </div>
                <hr/>
            }
        }).collect_view();

    let first_visible_title = create_memo(move |_| titles_visibility
        .get_value()
        .iter()
        .find_map(|(id, visible)| visible
            .get()
            .then_some(id.clone())));

    let preview_tiles = articles.iter()
        .map(|article| {
            let (id, preview_image_url, atricle_description) = (article.id.clone(), &article.preview, &article.description);
            let wrapped_description = match atricle_description.len() > 80 {
                true => {
                    let constrained_description = &atricle_description[..80];
                    match constrained_description.rfind(' ') {
                        Some(last_word) => &format!("{} …", &constrained_description[..last_word]),
                        None => &format!("{constrained_description}…"),
                    }
                },
                false => atricle_description,
            };
            
            view! {
                <div 
                    class=feed_css::article_preview_tile
                    attr:data-state=move || match first_visible_title
                        .get()
                        .is_some_and(|which_preview_tile_to_show_id| *which_preview_tile_to_show_id == id) {
                            true => "exposed",
                            false => "hidden"
                    }
                >
                    <img src={preview_image_url} class=style_baseline::square_squirrel_mask alt="Aricle's preview image"/>
                    <p>{wrapped_description}</p>
                </div>
            }
        }).collect_view();

    let liason_tiles = articles.iter()
        .flat_map(|article| &article.liasions)
        .unique()
        .sorted()
        .map(|liason| view! {
            <div class=feed_css::liasion_tile>
                <button
                    on:click = move |_| leptos::logging::log!("Hi!")
                >{liason}</button>
            </div>
        }).collect_view();

    view! {
        <div class=feed_css::feed_container>
            <div class=feed_css::preview_carousel> {preview_tiles} </div>
            <div class=feed_css::titles_listing> {title_tiles} </div>
            <div class=feed_css::liasion_selector> {liason_tiles} </div>
        </div>
    }
}

// WorksCarousel
// LiasionSelector
// WorkPreview
// WorkTile