use crate::api::*;

use std::fmt;
use std::str::FromStr;
use leptos::*;
use serde::{Deserialize, Serialize};

pub trait Summary {}

/// API error
#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorOn {
    // `#[from]` enables automatic conversion
    #[error(transparent)]
    FetchArticle(#[from] FetchArticleError),
    #[error(transparent)]
    FetchFoo(#[from] FetchFooError),
}

const API_ERROR_STR: &str = "ApiError";

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiError(pub ErrorOn);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", API_ERROR_STR, self.0.ser().unwrap())
    }
}

impl FromStr for ApiError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        leptos::logging::log!("Calling from_str: {}", s);

        s.split_once('|')
            .and_then(|(left, right)| {
                match left {
                    API_ERROR_STR => { match ErrorOn::de(right) {
                            Ok(v) => Some(ApiError(v)),
                            Err(_) => None,
                    }},
                    _ => None
                }
        }).ok_or(())
    }
}

impl From<ErrorOn> for ServerFnError<ApiError> {
    fn from(err: ErrorOn) -> Self {
        ServerFnError::from(ApiError(err))
    }
}

impl From<ServerFnError<ApiError>> for ErrorOn {
    fn from(err: ServerFnError<ApiError>) -> Self {
        match err {
            ServerFnError::WrappedServerError(api_error) => api_error.0,
            _ => unreachable!(),
        }
    }
}

impl ApiError {
    pub fn get(&self) -> ErrorOn {
        self.0
    }
}