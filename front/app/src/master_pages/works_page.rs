use leptos::*;
use leptos_router::*;

#[component]
pub fn WorksList() -> impl IntoView {
    view! {
        <h3>"Works listing page placeholder"</h3>
        <div>
            <A href="demo1">"Demo 1"</A>
            <A href="demo2">"Demo 2"</A>
        </div>
        <AnimatedOutlet
            outro="fadeOut"
            intro="fadeIn"
        />
    }
}

#[component]
pub fn ExactWork() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let work_content = move || match id().as_str() {
        "demo1" => view! { <DemoWork/> }.into_view(),
        "demo2" => view! { <h3>"Work article page placeholder"</h3> }.into_view(),
        _ => view! { <h3>"Work not found"</h3> }.into_view(),
    };
    
    view! {
        {work_content}
        <AnimatedOutlet/>
    }
}

#[component]
pub fn DemoWork() -> impl IntoView {
    // let mut options = Options::empty();
    // options.insert(Options::ENABLE_TABLES);
    // options.insert(Options::ENABLE_STRIKETHROUGH);
    // options.insert(Options::ENABLE_TASKLISTS);
    // let parser = pulldown_cmark::Parser::new(MARKDOWN_SOURCE);
    // let mut html_output = String::new();
    // pull_mark::push_html(&mut html_output, parser);
    // leptos::logging::log!("{}", html_output);
    
    view! {
        <h3>"Work article page placeholder"</h3>
        // <div inner_html=html_output></div>
    }
}