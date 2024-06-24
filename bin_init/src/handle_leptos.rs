use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::Request,
    response::IntoResponse,
};
use leptos::provide_context;
use leptos_axum::handle_server_fns_with_context;
use front::App;
use server::state::AppState;

pub async fn server_fn_handler(
    State(app_state): State<AppState>,
    _path: Path<String>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            log::debug!("Providing MD parser context (server_fn)");

            let AppState {
                md_parser,
                reqwest_client,
                ..
            } = app_state.clone();

            provide_context(md_parser);
            provide_context(reqwest_client);
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    axum::extract::State(option): axum::extract::State<leptos::LeptosOptions>,
    request: Request<AxumBody>,
) -> axum::response::Response {
    let handler = leptos_axum::render_app_async_with_context(
        option.clone(),
        move || {
            log::debug!("Providing MD parser context (router)");
            
            let AppState {
                md_parser,
                reqwest_client,
                ..
            } = app_state.clone();

            provide_context(md_parser);
            provide_context(reqwest_client);
        },
        App,
    );

    handler(request).await.into_response()
}