use leptos::{server_fn::error::NoCustomError, *};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use comrak::{plugins::syntect::SyntectAdapterBuilder, *};
use yaml_front_matter::YamlFrontMatter;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Metadata {
    title: String,
    description: String,
    aliases: Vec<String>,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArticleError {
    #[error("Invalid post ID.")]
    InvalidId,
    #[error("Post not found.")]
    PostNotFound,
    #[error("Server error.")]
    ServerError,
}

async fn fetch_markdown_from_cdn(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}

#[server]
pub async fn fetch_article(id: i32) -> Result<Article, ServerFnError> {
    println!("Called `fetch_article`");
    
    let extension = ExtensionOptionsBuilder::default()
        .strikethrough(true)
        .tagfilter(true)
        .table(true)
        .autolink(true)
        .tasklist(true)
        .superscript(true)
        .header_ids(Some("user-content-".to_string()))
        .footnotes(true)
        .description_lists(true)
        .front_matter_delimiter(Some("---".to_owned()))
        .multiline_block_quotes(true)
        .math_dollars(true)
        .math_code(true)
        .wikilinks_title_after_pipe(true)
        .wikilinks_title_before_pipe(true)
        .build()
        .unwrap();

    let parse = ParseOptionsBuilder::default()
        .smart(true)
        .default_info_string(Some("rust".to_string()))
        .relaxed_tasklist_matching(true)
        .relaxed_autolinks(true)
        .build()
        .unwrap();

    let options = Options {
        extension,
        parse,
        ..Options::default()
    };

    let markdown_source = fetch_markdown_from_cdn("http://cdn.solweo.tech/test_article.md").await;
    println!("MD: {:?}", markdown_source);

    let result = YamlFrontMatter::parse::<Metadata>(&markdown_source.unwrap()).unwrap();

    let Metadata {
        title,
        description,
        aliases,
    } = result.metadata;

    let builder = SyntectAdapterBuilder::new().theme("base16-ocean.dark");
    // let builder = SyntectAdapterBuilder::new().css();
    let adapter = builder.build();
    let mut plugins = Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let html_output = markdown_to_html_with_plugins(&result.content, &options, &plugins);

    Ok(Article {
        id: "yuvOBfTQ_bw".to_string(),
        title,
        content: html_output,
    })
}