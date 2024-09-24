use leptos::*;
use std::time::Duration;
use leptos_dom::helpers::TimeoutHandle;
use stylance::import_style as get_css;

get_css!(typewritter_css, "./typewritter.css");

#[derive(Clone, Debug)]
enum HoldupCueWaitBefore {
    Start,
    Type,
    Empty,
}

#[component]
pub fn OldTypeWritterFX(words: Vec<&'static str>) -> impl IntoView {
    let type_buffer = create_rw_signal(("", HoldupCueWaitBefore::Start));
    let typed = create_rw_signal("");
    let show_stub = create_rw_signal(true);
    
    let start_delay = Duration::from_millis(300);
    let typing_delay = Duration::from_millis(90);
    let empty_delay = Duration::from_millis(900);
    
    let mut result = Vec::new();
    // for word in ["tech artistry", "art sorcery", "product wizardy"].iter() {
    for word in words.iter() {
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
        <span class=typewritter_css::liquid>"[ "{typed}<Stub show=show_stub/>" ]"</span>
    }
}

#[component]
pub fn TypeWritterFX(words: Vec<String>) -> impl IntoView {
    let type_buffer = create_rw_signal((String::new(), HoldupCueWaitBefore::Start));
    let typed = create_rw_signal(String::new());
    let show_stub = create_rw_signal(true);
    
    let start_delay = Duration::from_millis(300);
    let typing_delay = Duration::from_millis(90);
    let empty_delay = Duration::from_millis(900);
    
    let mut result = Vec::new();
    for word in words.iter() {
        for i in 0..=word.len() {
            let cue = if i == 0 {
                HoldupCueWaitBefore::Start
            } else if i < word.len() {
                HoldupCueWaitBefore::Type
            } else {
                HoldupCueWaitBefore::Empty
            };
            result.push((word[..i].to_string(), cue));
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
        <span class=typewritter_css::liquid>"[ "{typed}<Stub show=show_stub/>" ]"</span>
    }
}

#[component]
pub fn ModTypeWritterFX(words: Vec<String>) -> impl IntoView {
    let type_buffer = create_rw_signal((String::new(), HoldupCueWaitBefore::Start));
    let typed = create_rw_signal(String::new());
    let show_stub = create_rw_signal(true);
    
    let start_delay = Duration::from_millis(300);
    let typing_delay = Duration::from_millis(90);
    let empty_delay = Duration::from_millis(900);
    
    let mut result = Vec::new();
    // for word in ["tech artistry", "art sorcery", "product wizardy"].iter() {

    // let a = words.iter().map(|w| w.as_str().clone())

    // let words = words.iter().map(String::as_str).collect::<Vec<_>>();
    // let words = words.clone();
    // use leptos_reactive::store_value;
    // let words = store_value(words.clone());
    // let words = words.iter().cloned().collect::<Vec<_>>();

    for word in words.iter() {
        for i in 0..=word.len() {
            let cue = if i == 0 {
                HoldupCueWaitBefore::Start
            } else if i < word.len() {
                HoldupCueWaitBefore::Type
            } else {
                HoldupCueWaitBefore::Empty
            };
            result.push((word[..i].to_string(), cue));
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
        <h1><span class=typewritter_css::liquid>"[ "{typed}<Stub show=show_stub/>" ]"</span></h1> 
    }
}

#[component]
fn Stub<S>(show: S) -> impl IntoView
where
    S: Fn() -> bool + 'static,
{
    let cls = create_memo(move |_| match show() {
        true => typewritter_css::stub,
        false => typewritter_css::stub_hidden,
    });
    
    view! { <span class=move || cls.get()>"|"</span> }
}