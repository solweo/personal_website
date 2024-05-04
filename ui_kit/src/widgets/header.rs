use leptos::{html::{button, h3, header, nav}, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::*;
use stylance::import_style as get_css;
use web_sys::MouseEvent;

use super::WidgetState;

get_css!(css, "./header.css");

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

    let lurking_nav = move || {
        nav()
            .class(css::hidden_nav, move || {
                menu_opened() || contact_opened() || match sd() {
                    Direction::Up => false,
                    Direction::Down => true,
                }
            })
            .child(view! {
                <button
                    on:click = open_menu
                >"MENU"</button>
                <h3>"|"</h3>
                <button
                    on:click = open_contact
                >"CONTACT"</button>
            })
    };

    let pop_up_menu = move || {
        nav()
            .class(css::header_pop_up_menu, true)
            .class(css::header_pop_up_menu_hidden, move || {
                !menu_opened()
            })
            .child(view! {
                <A on:click=close_either href="/">"Home"</A>
                <A on:click=close_either href="/about">"About"</A>
                <A on:click=close_either href="/works">"Works"</A>
            })
    };

    let pop_up_contact = move || {
        nav()
            .class(css::header_pop_up_contact, true)
            .class(css::header_pop_up_contact_hidden, move || {
                !contact_opened()
            })
            .child(view! {
                <h3>"Contacts:"</h3>
                <A on:click=close_either href="https://t.me/solweo">"Telegram: @solweo"</A>
                <A on:click=close_either href="mailto:adrian@solweo.tech">"Mail: adrian@solweo.tech"</A>
            })
    };

    let lurking_close = move || {
        button()
            .class(css::close_hidden, move || {
                !menu_opened() && !contact_opened()
            })
            .on(ev::click, close_either)
            .child("CLOSE")
    };

    view! {
        <header 
            class=css::bottom_floating_header
        >
            <A href="/">"SOLWEO"</A>
            {lurking_close}
            {lurking_nav}
            {pop_up_menu}
            {pop_up_contact}
        </header>
    }
}