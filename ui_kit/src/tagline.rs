use leptos::*;
use std::time::Duration;
use leptos_dom::helpers::TimeoutHandle;
use stylance::import_style as get_css;

get_css!(tagline_css, "./tagline.css");

#[component]
pub fn Tagline() -> impl IntoView {
    view! {
        <span class=tagline_css::title>"Embark the realms"</span>
        <br/>
        <span class=tagline_css::title>"of "</span>
        <TypeWritterFX/>
        <br/>
        <span class=tagline_css::title>" - thus unleashing"</span>
        <br/>
        <span class=tagline_css::title>"your ingenuity 🌱"</span>
    }
}

#[derive(Clone, Debug)]
enum HoldupCueWaitBefore {
    Start,
    Type,
    Empty,
}

#[component]
fn TypeWritterFX() -> impl IntoView {
    let type_buffer = create_rw_signal(("", HoldupCueWaitBefore::Start));
    let typed = create_rw_signal("");
    let show_stub = create_rw_signal(true);
    
    let start_delay = Duration::from_millis(300);
    let typing_delay = Duration::from_millis(90);
    let empty_delay = Duration::from_millis(900);
    
    let mut result = Vec::new();
    for word in ["tech artistry", "art sorcery", "product wizardy"].iter() {
        for i in 0..=word.len() {
            let cue = if i == 0 {
                HoldupCueWaitBefore::Start
            } else if i < word.len() {
                HoldupCueWaitBefore::Type
            } else {
                HoldupCueWaitBefore::Empty
            };
            result.push((&word[..i], cue));
        }
    }
    let it = store_value(result.into_iter().cycle());
    
    let handle: StoredValue<Option<TimeoutHandle>> = store_value(None);

    create_render_effect(move |_| {
        if let Some(h) = handle.get_value() {
            h.clear();
        }

        let delay = {
            let (line, cue) = type_buffer.get();
            typed.set(line);
            match cue {
                HoldupCueWaitBefore::Start => {show_stub.set(true); start_delay},
                HoldupCueWaitBefore::Type => typing_delay,
                HoldupCueWaitBefore::Empty => {show_stub.set(false); empty_delay},
            }
        };

        handle.set_value(Some(leptos_dom::helpers::set_timeout_with_handle(
            move || {
                it.update_value(|v| type_buffer.set(v.next().unwrap()));
            },
            delay,
        ).expect("set timeout in TypeWritterFX")));
        
    });

    view! {
        <span class=tagline_css::liquid>"[ "{typed}<Stub show=show_stub/>" ]"</span>
    }
}

#[component]
fn Stub<S>(show: S) -> impl IntoView
where
    S: Fn() -> bool + 'static,
{
    let cls = create_memo(move |_| match show() {
        true => tagline_css::stub,
        false => tagline_css::stub_hidden,
    });
    
    view! { <span class=move || cls.get()>"|"</span> }
}