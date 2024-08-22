use leptos::*;
use leptos_dom::IntoView;
use leptos_macro::component;
use leptos_macro::view;
use leptos_reactive::store_value;

/// Allows you to inline the data loading for an `async` block or server function directly into
/// your view, while simultaneously enabling error handling within a section of the interface.
/// 
/// Reads any passed [`Resource`](leptos_reactive::Resource) whose value is inside `Result<_, _>`.
/// It will show the `suspense_fallback` while `value` is loading. If the value turns out to be 
/// an `Result::Err`, it will display `error_fallback` while also passing it an inner error value.
/// Once all are resolved, it will render the `children`.
/// 
/// Note that the `children` will be rendered initially (in order to capture the fact that
/// those resources are read under the suspense), so you cannot assume that resources have
/// `Some` value in `children`.
/// 
/// ```
/// <CustErrBoundary
///     value=async_foo
///     suspense_fallback=move || view! { <p>"Loading..."</p> }
///     error_fallback=move |v| {
///         if let ErrorOn::FetchFoo(err) = ErrorOn::from(v) {
///             match err {
///                 server::api::fetch_foo::Error::DummyServerError => view! {
///                     <h3>"Dummy server error was invoked"</h3>
///                     <button on:click=on_click>"Try again"</button>
///                 }.into_view(),
///             }
///         } else {
///             view! {
///                 <h3>"An error occurred that is NOT related to fetching Foo"</h3>
///                 <button on:click=on_click>"Try again"</button>
///             }.into_view()
///         }
///     }
///     // the data is bound to whatever variable name you provide
///     // `let:data` syntax equivalent to `children=|data| view! { <p>{data}</p> }`
///     let:data
/// >
///     <p>"Retrived data: "{data}</p>
/// </CustErrBoundary>
/// ```
#[component(transparent)]
pub fn CustErrBoundary<T, E, IVC, ChF, IVE, FlF, RES>(
    value: RES,
    /// Returns a fallback UI that will be shown while `async` [`Resource`](leptos_reactive::Resource)s are still loading. By default this is the empty view.
    #[prop(optional, into)]
    suspense_fallback: ViewFn,
    error_fallback: FlF,
    children: ChF,
) -> impl IntoView
where
    E: std::fmt::Display + 'static,
    IVC: IntoView,
    ChF: Fn(T) -> IVC + 'static,
    IVE: IntoView,
    FlF: Fn(E) -> IVE + 'static,
    RES: SignalGet<Value = Option<Result<T, E>>> + 'static,
{
    let children = store_value(children);
    let error_fallback = store_value(error_fallback);

    let content = StoredValue::new(move || {
        value.get().map(|data| { match data {
            Result::Ok(v) => children.with_value(|view| view(v).into_view()),
            Result::Err(v) => error_fallback.with_value(|view| view(v).into_view()),
        }})
    });

    view! {
        <Suspense fallback=suspense_fallback>
            {content.with_value(|f| f())}
        </Suspense>
    }
}