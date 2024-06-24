use leptos::*;
use leptos_router::*;

use server::api::fetch_article;

#[component]
pub fn About() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let art = create_resource(count, |_| fetch_article("test_article".to_string()));

    view! {
        <h3>"About page placeholder"</h3>

        <Suspense
            fallback = move || view! { <h3>"Articel is Loading"</h3> }
        >
            { match art.get() {
                Option::Some(Result::Err(_)) => Some(
                    view! {
                        <button on:click=on_click>"Try again"</button>
                    }
                ),
                _ => None,
            }}
            <ErrorBoundary fallback=|_errors| {
                view! {
                    <h3>"An error occurred while fetching Article"</h3>
                }
            }>
                {
                    move || {
                    art.get().map(|r|
                        r.map(|v| view! {
                            <h3>"Title: "{&v.aliases[0]}</h3>
                            <h3>"Description: "{v.description}</h3>
                            <h3>"Text:"</h3>
                            <div inner_html=v.content></div>
                        }.into_view())
                    )
                }}
            </ErrorBoundary>
        </Suspense>

        <AnimatedOutlet/>
    }
}