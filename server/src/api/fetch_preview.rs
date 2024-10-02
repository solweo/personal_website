use leptos::*;
use serde::{Deserialize, Serialize};
use crate::api::ApiError;
use crate::api::fetch_article::Metadata;
cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::api::ErrorOn;
    use yaml_front_matter::YamlFrontMatter;
    use crate::article_parser;
}}

#[server]
pub async fn fetch_preview() -> Result<Vec<Metadata>, ServerFnError<ApiError>> {
    println!("Called `fetch_preview`");

    let ids = vec!["test_article".to_string(), "about_article".to_string()];


    todo!()
}