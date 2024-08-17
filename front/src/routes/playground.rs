use leptos::*;
use server::api::ErrorOn;

#[component]
pub fn playground() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    
    let async_foo = create_resource(count, server::api::fetch_foo);
    let async_article = create_resource(count, |_| server::api::fetch_article("test_article".to_string()));

    view! {
        <h3>"Playground page placeholder"</h3>

        <button on:click=on_click>"Refresh anchored ones. Already: "{count}"th time"</button>
        
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <h2>"Retrived Data"</h2>
            {move || {
                async_foo.get().map(|data| { match data {
                        Result::Err(v) => {
                            if let ErrorOn::FetchFoo(err) = ErrorOn::from(v) {
                                match err {
                                    server::api::fetch_foo::Error::DummyServerError => view! {
                                        <h3>"Dummy server error was invoked"</h3>
                                        <button on:click=on_click>"Try again"</button>
                                    }.into_view(),
                                }
                            } else {
                                view! {
                                    <h3>"An error occurred that is NOT related to fetching Foo"</h3>
                                    <button on:click=on_click>"Try again"</button>
                                }.into_view()
                            }
                        },
                        Result::Ok(v) => view! {
                            <h3>"Here the data: "{v}</h3>
                        }.into_view(),
                }})
            }}
        </Suspense>

        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <h2>"Retrived Data"</h2>
            {move || {
                async_article.get().map(|data| { match data {
                        Result::Err(v) => {
                            if let ErrorOn::FetchArticle(err) = ErrorOn::from(v) {
                                match err {
                                    server::api::fetch_article::Error::InvalidId => view! {
                                        <h3>"An invalid article ID was sent to the server"</h3>
                                        <button on:click=on_click>"Try again"</button>
                                    }.into_view(),
                                    server::api::fetch_article::Error::NotFound => view! {
                                        <h3>"Server couldn't find the article by the specified ID"</h3>
                                        <button on:click=on_click>"Try again"</button>
                                    }.into_view(),
                                    server::api::fetch_article::Error::FailedToParse => view! {
                                        <h3>"There is something faulty with the article, server failed to handle it"</h3>
                                        <button on:click=on_click>"Try again"</button>
                                    }.into_view(),
                                }
                            } else {
                                view! {
                                    <h3>"An error occurred that is NOT related to fetching Article"</h3>
                                    <button on:click=on_click>"Try again"</button>
                                }.into_view()
                            }
                        },
                        Result::Ok(v) => view! {
                            <h3>"Here the article: "{v.content}</h3>
                        }.into_view(),
                }})
            }}
        </Suspense>
    }
}