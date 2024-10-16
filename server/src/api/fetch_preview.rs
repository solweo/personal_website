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

    // Just dummy data
    meta_bodies.append(&mut vec![
        ArticlePreview {
            id: "akd1f32".to_string(),
            aliases: vec!["How to make your protocol".to_string()],
            description: "some description".to_string(),
            preview: "http://cdn.solweo.tech/assets/church.jpg".to_string(),
            liasions: vec!["layout system".to_string()],
        },
        ArticlePreview {
            id: "ae02agp".to_string(),
            aliases: vec!["Life in Cyprus (+ photos)".to_string()],
            description: "some description".to_string(),
            preview: "http://cdn.solweo.tech/assets/discourse.jpg".to_string(),
            liasions: vec!["layout system".to_string()],
        },
        ArticlePreview {
            id: "2fmi8ht".to_string(),
            aliases: vec!["Future of web engines and web itself".to_string()],
            description: "some description".to_string(),
            preview: "http://cdn.solweo.tech/assets/dreming.jpg".to_string(),
            liasions: vec!["layout system".to_string()],
        },
        ArticlePreview {
            id: "e14ar2".to_string(),
            aliases: vec!["Implementing SSR in Leptos".to_string()],
            description: "some description".to_string(),
            preview: "http://cdn.solweo.tech/assets/eating.jpg".to_string(),
            liasions: vec!["layout system".to_string()],
        },
        ArticlePreview {
            id: "ga56sd".to_string(),
            aliases: vec!["Taking notes is only underset of memex system".to_string()],
            description: "some description".to_string(),
            preview: "http://cdn.solweo.tech/assets/flower.jpg".to_string(),
            liasions: vec!["layout system".to_string()],
        }
    ]);

    for id in ids {
        let response = reqwest::get(format!("http://cdn.solweo.tech/work/{}/en.md", id)).await;
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