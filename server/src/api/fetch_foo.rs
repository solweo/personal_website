use leptos::*;
use serde::{Deserialize, Serialize};
use crate::api::{ApiError, ErrorOn};

#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    #[error("Dummy server error was invoked")]
    DummyServerError,
}

#[server]
pub async fn fetch_foo(id: i32) -> Result<String, ServerFnError<ApiError>> {
    // fake API delay
    leptos::logging::log!("Calling fetch_foo; id: {}", id);
    std::thread::sleep(std::time::Duration::from_millis(700));
    
    if (id + 1) % 2 == 0 {
        return Err(ErrorOn::from(Error::DummyServerError).into());
    }
    
    Ok(format!("from server {}", id))
}