pub mod api;

use leptos::{server_fn::error::NoCustomError, *};

#[server]
pub async fn fetch_foo(id: i32) -> Result<String, ServerFnError> {
    println!("Called `fetch_foo`");

    // fake API delay
    // std::thread::sleep(std::time::Duration::from_millis(1250));

    if (id + 1) % 2 == 0 {
        return Err(ServerFnError::<NoCustomError>::Registration("woopsy".to_string()));
    }
    
    Ok("from server".to_string())
}