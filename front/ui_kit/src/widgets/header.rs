use leptos::*;
use leptos_router::*;
use leptos_use::*;
use stylance::import_style as get_css;

use super::index;
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

    let open_menu = move |_| set_menu_opened.update(|v| *v = true);
    let open_contact = move |_| set_contact_opened.update(|v| *v = true);
    let close_either = move |_| {
        set_contact_opened.update(|v| *v = false);
        set_menu_opened.update(|v| *v = false);
    };

    let (scroll_direction, _) = create_signal(TrackScroll::default());
    let scroll_direction = move || {
        scroll_direction.with(|s| s.direction())
    };
    // ^ Can't be reused more than once

    let header_attr = create_memo(move |_| 
        with!(|menu_opened, contact_opened| 
            match (menu_opened, contact_opened, scroll_direction()) {
                (true, _, _) => "expanded",
                (_, true, _) => "expanded",
                (_, _, Direction::Up) => "neutral",
                (_, _, Direction::Down) => "collapsed",
            }
        )
    );

    let menu_attr = create_memo(move |_| 
        menu_opened.with(|v| match *v {
            true => "exposed",
            false => "hidden",
        })
    );

    let contact_attr = create_memo(move |_| 
        contact_opened.with(|v| match *v {
            true => "exposed",
            false => "hidden",
        })
    );

    let close_attr = create_memo(move |_| 
        with!(|menu_opened, contact_opened| 
            if *menu_opened || *contact_opened {
                return "exposed";
            } else {
                return "hidden";
            }
        )
    );

    let bottom_floating_header = move || {
        view! {
            <header
                class=format!("{} {}", 
                    index::small_context,
                    css::bottom_floating_header
                )
                attr:data-state=header_attr
            >
                <A class=format!("{} {}", 
                    index::neutral_clickable, 
                    css::bfh_logo
                )
                    href="/"
                    on:click=close_either
                >"SOLWEO"</A>
                <nav class=css::bfh_nav>
                    <button
                    on:click = open_menu
                    class=index::neutral_clickable
                    >"MENU"</button>
                    <button
                        on:click = open_contact
                        class=index::neutral_clickable
                    >"CONTACT"</button>
                </nav>
            </header>
        }
    };

    let bfh_pop_up_menu = move || {
        view! {
            <nav
                class=format!("{} {}", 
                    index::large_context,
                    css::bfh_pop_up
                )
                attr:data-state=menu_attr
            >
                <A class=index::neutral_clickable on:click=close_either href="/">"Home"</A>
                <A class=index::neutral_clickable on:click=close_either href="/about">"About"</A>
                <A class=index::neutral_clickable on:click=close_either href="/works">"Works"</A>
                <A class=index::neutral_clickable on:click=close_either href="/ui-kit-preview">"Demo"</A>
            </nav>
        }
    };

    let bfh_pop_up_contact = move || {
        view! {
            <nav
                class=format!("{} {}", 
                    index::medium_context,
                    css::bfh_pop_up
                )
                attr:data-state=contact_attr
            >
                <h3>"Contacts:"</h3>
                <A class=index::neutral_clickable on:click=close_either href="https://t.me/solweo">"Telegram: @solweo"</A>
                <A class=index::neutral_clickable on:click=close_either href="mailto:adrian@solweo.tech">"Mail: adrian@solweo.tech"</A>
            </nav>
        }
    };

    let bfh_pop_up_closer = move || {
        view! {
            <div
                class=format!("{} {}", 
                    index::small_context,
                    css::bfh_pop_up_closer
                )
                attr:data-state=close_attr
            >
                <button
                    on:click = close_either
                    class=index::neutral_clickable
                >"CLOSE"</button>
            </div>
        }
    };

    (bottom_floating_header, bfh_pop_up_menu, bfh_pop_up_contact, bfh_pop_up_closer)
}