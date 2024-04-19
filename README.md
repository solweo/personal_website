# Personal website

Required installations:

- `cargo install trunk`
- `cargo install stylance-cli`
- `npm install -g postcss-cli`
- `npm install -g <REQUIRED_PLUGIN_NAME>`, where the necessary plug-ins are: `autoprefixer`, `postcss-selector-matches`, `postcss-selector-not`, `postcss-custom-media`, `postcss-media-minmax`, `postcss-nesting`, `postcss-custom-selectors`

## Generating necessary style files

1. Hash each CSS class and consolidate all the individual files into a single one: `stylance --watch <MANIFEST_DIR>`.
2. To resolve PostCSS features into eventual style file: `postcss ./target/stylance-output/bundle.css --use autoprefixer --use postcss-selector-matches --use postcss-selector-not --use postcss-custom-media --use postcss-media-minmax --use postcss-nesting --use postcss-custom-selectors --no-map --output ./target/postcss-output/bundle.css --watch --verbose`.

## Building and serving this site

- Run `trunk serve --open` or `cargo leptos watch` to build and open this site.
