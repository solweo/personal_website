use cfg_if::cfg_if;
use http::status::StatusCode;
use leptos::*;
use thiserror::Error;
use ui_kit::index;

#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// A basic function to display errors served by the error boundaries.
#[component]
pub fn DisplayError(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => create_rw_signal(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    
    let errors = errors.get_untracked();

    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

    // Only the response code for the first error is actually sent from the server
    cfg_if! { if #[cfg(feature="ssr")] {
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }}

    view! {
        <div class=index::error_page_wrapper>
            <h1>"Oops, something went wrong"</h1>
            <For
                each=move || { errors.clone().into_iter().enumerate() }
                key=|(index, _error)| *index
                children=move |error| {
                    let error_code = error.1.status_code();
                    view! {
                        <h2>{error_code.to_string()}</h2>
                    }
                }
            />
        </div>
    }
}
