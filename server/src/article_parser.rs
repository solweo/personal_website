use comrak::{plugins::syntect::SyntectAdapterBuilder, *};

type PreparedParser = dyn Fn(&str) -> String + Send + Sync;
pub struct ArticleParser(std::sync::Arc<Box<PreparedParser>>);

impl Clone for ArticleParser {
    fn clone(&self) -> Self { 
        Self(std::sync::Arc::clone(&self.0))
    }
}

impl std::fmt::Debug for ArticleParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ArticleParser",
        )
    }
}

impl Default for ArticleParser {
    fn default() -> Self {
        log::info!("Bulding MD parser");

        let options = Options {
            extension: ExtensionOptionsBuilder::default()
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
                .unwrap(),
            parse: ParseOptionsBuilder::default()
                .smart(true)
                .default_info_string(Some("rust".to_string()))
                .relaxed_tasklist_matching(true)
                .relaxed_autolinks(true)
                .build()
                .unwrap(),
            ..Options::default()
        };

        let adapter = std::sync::Arc::new((SyntectAdapterBuilder::new().theme("base16-ocean.dark")).build());

        Self(std::sync::Arc::new(Box::new(move |md| {
            log::info!("Running MD parser");

            let mut plugins = Plugins::default();
            plugins.render.codefence_syntax_highlighter = Some(&*adapter);

            markdown_to_html_with_plugins(md, &options, &plugins)
        })))
    }
}

impl ArticleParser {
    pub fn parse(&self, md: &str) -> String {
        (self.0)(md)
    }
}