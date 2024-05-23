use leptos::{server_fn::error::NoCustomError, *};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
// use comrak::{markdown_to_html, Options};
use comrak::{plugins::syntect::SyntectAdapterBuilder, *};

static MARKDOWN_SOURCE: &str = r#"---                                                                            
title: 'Some title'                                                         
description: 'Some description'     
aliases: ['some alias', 'another alias']          
---  

## Code
```rust
fn main() {
    println!("hello world !")
}
```

---

## Math
- $1+1=2$
- $e^{i\pi}+1=0$

$$\int_0^{+\infty}\dfrac{\sin(t)}{t}\,dt=\dfrac{\sqrt{\pi}}{2}$$

## Links and images
![example.org](https://example.org/)

## Style
| unstyled | styled    |
| :-----:  | ------    |
| bold     | **bold**  |
| italics  | *italics* |
| strike   | ~strike~  |

> Hey, I am a quote !

## Lists
1) one
2) two
3) three

- and
- unorderded
- too

Even todo lists:
- [ ] todo
- [x] done
"#;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String,
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

    let builder = SyntectAdapterBuilder::new().theme("base16-ocean.dark");
    // let builder = SyntectAdapterBuilder::new().css();
    let adapter = builder.build();
    let mut plugins = Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let html_output = markdown_to_html_with_plugins(MARKDOWN_SOURCE, &options, &plugins);

    Ok(Article {
        id: "yuvOBfTQ_bw".to_string(),
        title: "Markdown Layout Test".to_string(),
        content: html_output,
    })
}