use std::fmt;
use std::str::FromStr;
use leptos::*;
use serde::{Deserialize, Serialize};
use crate::api::*;

/// API error
#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorOn {
    // `#[from]` enables automatic conversion
    #[error(transparent)]
    FetchArticle(#[from] fetch_article::Error),
    #[error(transparent)]
    FetchFoo(#[from] fetch_foo::Error),
}

const API_ERROR_STR: &str = "ApiError";

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiError(pub ErrorOn);

impl ApiError {
    pub fn get(&self) -> ErrorOn {
        self.0
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", API_ERROR_STR, self.0.ser().unwrap())
    }
}

impl FromStr for ApiError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

// Could be used in future for more convenient error generation

// extern crate proc_macro;
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput};

// #[proc_macro_derive(GenerateErr)]
// pub fn generate_err_derive(input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as DeriveInput);

//     impl_generate_err(&ast)
// }

// fn impl_generate_err(ast: &DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     let expanded = quote! {
//         impl #name {
//             fn into(self) -> Result<(), super::super::ErrorOn<Self>> {
//                 Err(ErrorOn::from(#name::DummyServerError).into())
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }