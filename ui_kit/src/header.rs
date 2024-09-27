use leptos::*;
use leptos_router::*;
use stylance::import_style as get_css;

get_css!(header_css, "./header.css");

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

    view! {
        <div class=header_css::bottom_floating_navigational_header_container>
            <header>
                <A 
                    href="/"
                >"Solweo"</A>
                <button
                    on:click = open_menu
                >"Menu"</button>
                <button
                    on:click = close_either
                >"Close"</button>
                <button
                    on:click = open_contact
                >"Contact"</button>
            </header>
            <nav
                attr:data-state=menu_attr
            >
                <A
                    href="/about"
                >"About"</A>
                <A 
                    href="/works"
                >"Works"</A>
                <A 
                    href="/playground"
                >"Playground"</A>
            </nav>
            <address
                attr:data-state=contact_attr
            >
                <A on:click=close_either href="https://t.me/solweo">"Telegram: @solweo"</A>
                <A on:click=close_either href="mailto:adrian@solweo.tech">"Mail: adrian@solweo.tech"</A>
            </address>
        </div>
    }
}