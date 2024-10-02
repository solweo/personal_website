use leptos::*;
use serde::{Deserialize, Serialize};
use crate::api::ApiError;
use crate::api::fetch_article::Metadata;
cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::api::ErrorOn;
    use yaml_front_matter::YamlFrontMatter;
}}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct ArticlePreview {
    pub id: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub preview: String,
    pub liasions: Vec<String>,
}

#[server]
pub async fn fetch_preview() -> Result<Vec<ArticlePreview>, ServerFnError<ApiError>> {
    println!("Called `fetch_preview`");

    let ids = vec!["test_article".to_string(), "about_article".to_string()];

    let mut meta_bodies = Vec::new();

    for id in ids {
        let response = reqwest::get(format!("http://cdn.solweo.tech/{}.md", id)).await;
        if let Ok(response) = response { 
            let md_body = response.text().await.unwrap();
            let result = YamlFrontMatter::parse::<Metadata>(&md_body).unwrap();

            let Metadata {
                aliases,
                description,
                preview, 
                liasions,
            } = result.metadata;

            meta_bodies.push(ArticlePreview {
                id,
                aliases,
                description,
                preview,
                liasions,
            });
        };
    }

    Ok(meta_bodies)
}