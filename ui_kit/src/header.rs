use leptos::*;
use leptos_router::*;
use leptos_use::*;
use stylance::import_style as get_css;

get_css!(header_css, "./header.css");

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    Up,
    Down,
}

#[derive(Clone, Copy)]
struct TrackScroll {
    y1: Signal<f64>,
    y0: StoredValue<f64>,
}

impl Default for TrackScroll {
    fn default() -> Self {
        let (_, y1) = use_window_scroll();
        Self {
            y1,
            y0: store_value(y1.get_untracked()),
        }
    }
}

impl TrackScroll {
    fn direction(&self) -> Direction {
        let (y1, y0) = (self.y1, self.y0);
        let delta = with!(|y1, y0| y1 - y0);
        y0.update_value(|v| *v = y1.get_untracked());
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
    let scroll_direction = create_memo({
        let scroll_direction = TrackScroll::default();
        move |_| { scroll_direction.direction() }
    });

    let open_menu = move |_| set_menu_opened.update(|v| *v = true);
    let open_contact = move |_| set_contact_opened.update(|v| *v = true);
    let close_either = move |_| {
        set_contact_opened.update(|v| *v = false);
        set_menu_opened.update(|v| *v = false);
    };

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

    view! {
        <div class=header_css::bottom_floating_navigational_header_container>
            <header
                attr:data-state=header_attr
            >
                <button
                    class=header_css::close_btn_in_header
                    on:click = close_either
                >"Close"</button>
                <A 
                    class=header_css::logo_in_header
                    href="/"
                    on:click=close_either
                >"Solweo"</A>
                <button
                    on:click = open_menu
                >"Menu"</button>
                
                <button
                    on:click = open_contact
                >"Contact"</button>
            </header>
            <nav
                attr:data-state=menu_attr
            >
                <A
                    href="/about"
                    on:click = close_either
                >"About"</A>
                <A 
                    href="/works"
                    on:click = close_either
                >"Works"</A>
                <A 
                    href="/playground"
                    on:click = close_either
                >"Playground"</A>
            </nav>
            <address
                attr:data-state=contact_attr
            >
                <A
                    href="https://t.me/solweo"
                    on:click=close_either
                >"Telegram: @solweo"</A>
                <A 
                    href="mailto:adrian@solweo.tech"
                    on:click=close_either
                >"Mail: adrian@solweo.tech"</A>
            </address>
        </div>
    }
}