use leptos::*;
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[server]
pub async fn fetch_foo(id: i32) -> Result<String, ServerFnError<crate::error::ApiError>> {
    // fake API delay
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    if (id + 1) % 2 == 0 {
        return Err(crate::error::ErrorOn::from(FetchFooError::DummyServerError).into());
    }
    
    Ok(format!("from server {}", id))
}

#[server]
pub async fn fetch_boo(id: i32) -> Result<String, ServerFnError> {
    // fake API delay
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(format!("from server {}", id))
}


#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FetchFooError {
    #[error("Dummy server error was invoked")]
    DummyServerError,
}