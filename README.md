# Personal website

This app was:

- written in [Rust](https://www.rust-lang.org/),
- using the [Leptos](https://leptos.dev/) framework,
- styled with [PostCSS](https://postcss.org/),
- served by [Axum](https://github.com/tokio-rs/axum) via Leptos SSR.

## Prerequisites

This project depend on the following tools. Please install them as needed.

- [Rust](https://www.rust-lang.org/)
- Nightly Rust
  - Run `rustup toolchain install nightly`
  - Run `rustup target add wasm32-unknown-unknown`
- [Cargo Make](https://sagiegurari.github.io/cargo-make/)
  - Run `cargo install --force cargo-make`
- Command Line Interface (CLI) for `wasm-bindgen`
  - Run `cargo install -f wasm-bindgen-cli`
- CLI for `stylance`
  - Run `cargo install stylance-cli`
- "[Just](https://github.com/casey/just)" Command runner 
  - Run `cargo install just`
- [Node.js](https://nodejs.org/)
- [PostCSS](https://postcss.org/)
  - Run `npm install -g postcss-cli`
  - `npm install -g <REQUIRED_PLUGIN_NAME>`, where the necessary plug-ins are: `postcss-mixins`, `autoprefixer`, `postcss-selector-matches`, `postcss-selector-not`, `postcss-custom-media`, `postcss-media-minmax`, `postcss-nesting`, `postcss-custom-selectors`

## Running the project locally in dev mode

Run `just dev`

## Deploying to the cloud