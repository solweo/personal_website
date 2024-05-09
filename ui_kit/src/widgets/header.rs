use leptos::{html::{button, h1, h3, header, nav}, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::*;
use stylance::import_style as get_css;
use web_sys::MouseEvent;

use super::WidgetState;

get_css!(css, "./header.css");
get_css!(index, "../index.css");

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Clone, Copy)]
struct TrackScroll {
    y1: Signal<f64>,
    y0: RwSignal<f64>,
}

impl Default for TrackScroll {
    fn default() -> Self {
        let (_, y1) = use_window_scroll();
        Self {
            y1,
            y0: create_rw_signal::<f64>(y1.get_untracked()),
        }
    }
}

impl TrackScroll {
    fn direction(&self) -> Direction {
        let (y1, y0) = (self.y1, self.y0);
        let delta = with!(|y1, y0| y1 - y0);
        y0.update_untracked(|v| *v = y1.get_untracked());
        if delta > 0.0 {
            return  Direction::Down;
        }
        Direction::Up
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let (menu_opened, set_menu_opened) = create_signal(false);
    let (contact_opened, set_contact_opened) = create_signal(false);

    let open_menu = move |_: MouseEvent| set_menu_opened.update(|v| *v = true);
    let open_contact = move |_: MouseEvent| set_contact_opened.update(|v| *v = true);
    let close_either = move |_: MouseEvent| {
        set_contact_opened.update(|v| *v = false);
        set_menu_opened.update(|v| *v = false);
    };

    let (sd, _) = create_signal(TrackScroll::default());
    let sd = move || {
        sd.with(|s| s.direction())
    };
    // ^ Can't be reused more than once

    /*

    let pop_up_menu = move || {
        nav()
            .class(css::bfh_pop_up, true)
            .class(css::bfh_pop_up_hidden, move || {
                !menu_opened()
            })
            .child(view! {
                <A on:click=close_either href="/">"Home"</A>
                <A on:click=close_either href="/about">"About"</A>
                <A on:click=close_either href="/works">"Works"</A>
                <A on:click=close_either href="/ui-kit-preview">"Demo"</A>
            })
    };

    let pop_up_contact = move || {
        nav()
            .class(css::bfh_pop_up, true)
            .class(css::bfh_pop_up_hidden, move || {
                !contact_opened()
            })
            .child(view! {
                <h3>"Contacts:"</h3>
                <A on:click=close_either href="https://t.me/solweo">"Telegram: @solweo"</A>
                <A on:click=close_either href="mailto:adrian@solweo.tech">"Mail: adrian@solweo.tech"</A>
            })
    };

    */
    
    // Rethinking

    let pop_up_menu = move || {
        nav()
            .class(css::re_pop_up, true)
            .child(view! {
                <A class=index::neutral_clickable on:click=close_either href="/">"Home"</A>
                <A class=index::neutral_clickable on:click=close_either href="/about">"About"</A>
                <A class=index::neutral_clickable on:click=close_either href="/works">"Works"</A>
                <A class=index::neutral_clickable on:click=close_either href="/ui-kit-preview">"Demo"</A>
            })
    };
    
    header()
        .class(index::medium_context, true)
        .class(css::re_header_baseline, true)
        .dyn_classes(move || {
            let a = match (menu_opened(), contact_opened(), sd()) {
                // (mo, co, sd)
                (true, _, _) => css::re_header_expanded,
                (_, true, _) => css::re_header_expanded,
                (_, _, Direction::Up) => css::re_header_neutral,
                (_, _, Direction::Down) => css::re_header_collapsed,
            };
            vec![css::re_header_baseline, a]
        })
        .child((
            h1().class(index::neutral_clickable, true).class(css::re_logo, true).child("SOLWEO"),
            view! {
                <button
                    on:click = close_either
                    class=format!("{} {}", 
                    index::neutral_clickable,
                    css::re_close
                )
                >"CLOSE"</button>
            },
            nav()
                .class(css::re_nav, true)
                .child(view! {
                    <button
                    on:click = open_menu
                    class=index::neutral_clickable
                    >"MENU"</button>
                    <button
                        on:click = open_contact
                        class=index::neutral_clickable
                    >"CONTACT"</button>
                }),
            pop_up_menu
        ))
}