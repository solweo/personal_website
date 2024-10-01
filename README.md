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

Call `just dev`

## Deploying to the server

My setup includes [NGINX](https://nginx.org/) to handle multiple domains, where specifically www.solweo.tech is proxied to a port raised by Leptos.

Call `just serve` to serve it at `site-addr` (Look into leptos' metadata at `Cargo.toml`) in release mode.

If you're curious about my particular set-up, see below.

### NGINX configuration file

Configuration file at `/etc/nginx/conf.d/solweo.tech.conf`:

```conf
server {
  listen *:80 default_server;
  listen [::]:80 default_server;

  server_name solweo.tech www.solweo.tech;

  location / {
    proxy_pass http://{site-addr}; # Ex: `127.0.0.1:3000`
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection 'upgrade';
    proxy_set_header Host $host;
    proxy_cache_bypass $http_upgrade;
  }
}
```

Launching system service via `systemctl start nginx`.

### Leptos app as a system service

System's service configuration file at `/etc/systemd/system/my-site.service`:

```service
[Unit]
Description=Personal website service
After=multi-user.target
[Service]
Type=simple
Restart=always
WorkingDirectory=/var/www/solweo.tech/
ExecStart=just serve
[Install]
WantedBy=multi-user.target
```

And towards the end there are only 3 steps:

1. Updating system daemon: `sudo systemctl daemon-reload`;
2. Enabling service: `systemctl enable my-site.service`;
3. Starting service: `systemctl start my-site.service`.
