use axum::extract::FromRef;
use leptos::LeptosOptions;
use crate::article_parser::ArticleParser;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub md_parser: ArticleParser,
    pub reqwest_client: reqwest::Client,
}