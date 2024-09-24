use leptos::*;
use serde::{Deserialize, Serialize};
use crate::api::ApiError;
cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::api::ErrorOn;
    use yaml_front_matter::YamlFrontMatter;
    use crate::article_parser;
}}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Article {
    pub aliases: Vec<String>,
    pub description: String,
    pub preview: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Metadata {
    aliases: Vec<String>,
    description: String,
    preview: String,
}

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    #[error("Invalid article ID")]
    InvalidId,
    #[error("Article not found")]
    NotFound,
    #[error("Server failed to parse article file")]
    FailedToParse,
}

#[cfg(feature = "ssr")]
async fn fetch_markdown_from_cdn(id: &str) -> Result<String, reqwest::Error> {
    let url = format!("http://cdn.solweo.tech/{}.md", id);
    expect_context::<reqwest::Client>()
        .get(&url).send().await?
        .text().await
}

#[server]
pub async fn fetch_article(id: String) -> Result<Article, ServerFnError<ApiError>> {
    println!("Called `fetch_article`");

    // fake API delay
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    // Swapping to test ID
    // let id = "test_article".to_string();

    let markdown_source = match fetch_markdown_from_cdn(&id).await {
        Ok(v) => v,
        Err(_) => return Err(ErrorOn::from(Error::InvalidId).into()),
    };

    let result = match YamlFrontMatter::parse::<Metadata>(&markdown_source) {
        Ok(v) => v,
        Err(_) => return Err(ErrorOn::from(Error::FailedToParse).into()),
    };

    let Metadata {
        aliases,
        description,
        preview, 
    } = result.metadata;
    
    let html_output = expect_context::<article_parser::ArticleParser>().parse(&result.content);
    
    Ok(Article {
        // id: "yuvOBfTQ_bw".to_string(),
        aliases,
        description,
        preview,
        content: html_output,
    })
}