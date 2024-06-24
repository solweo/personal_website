use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use yaml_front_matter::YamlFrontMatter;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Article {
    pub aliases: Vec<String>,
    pub description: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Metadata {
    aliases: Vec<String>,
    description: String,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FetchArticleError {
    #[error("Invalid post ID")]
    InvalidId,
    #[error("Post not found")]
    NotFound,
    #[error("Server failed to parse post file")]
    FailedToParse,
}

async fn fetch_markdown_from_cdn(id: &str) -> Result<String, reqwest::Error> {
    let url = format!("http://cdn.solweo.tech/{}.md", id);
    expect_context::<reqwest::Client>()
        .get(&url).send().await?
        .text().await
}

#[server]
pub async fn fetch_article(id: String) -> Result<Article, ServerFnError> {
    println!("Called `fetch_article`");

    // fake API delay
    // std::thread::sleep(std::time::Duration::from_millis(1000));
    
    let markdown_source = fetch_markdown_from_cdn(&id).await;

    let result = YamlFrontMatter::parse::<Metadata>(&markdown_source.unwrap()).unwrap();

    let Metadata {
        aliases,
        description,
    } = result.metadata;
    
    let html_output = expect_context::<crate::state::ArticleParser>().parse(&result.content);
    
    Ok(Article {
        // id: "yuvOBfTQ_bw".to_string(),
        aliases,
        description,
        content: html_output,
    })
}