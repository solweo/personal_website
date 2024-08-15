use std::time::Duration;
use leptos::*;
use leptos::component;
use leptos_dom::helpers::TimeoutHandle;
use leptos_macro::view;

/// Leptos' `<AnimatedShow>` based component. It will show its children when the `when` condition is `true`,
/// and show the fallback when it is `false`, without rerendering every time the condition changes.
/// 
/// The fallback prop is optional and defaults to rendering nothing.
/// 
/// ```
/// use leptos::*;
/// use std::rc::Rc;
/// use std::time::Duration;
///
/// let (count, set_count) = create_signal(0);
/// let on_click = move |_| set_count.update(|count| *count += 1);
///
/// view! {
///   <button on:click=on_click>"Click Me"</button>
///   
///   <AnimatedShowSwitch
///      when=move || count.get() % 2 == 0
///      intro="fadeIn"
///      outro="fadeOut"
///      fallback_intro="fadeIn"
///      fallback_outro="fadeOut"
///      delay=Duration::from_millis(250)
///      fallback=move || view! { 
///         <h1>"`count` is odd"</h1>
///      }
///   >
///      <h1>"`count` is even-numbered"</h1>
///   </AnimatedShowSwitch>
/// };
/// ```
#[cfg_attr(
    any(debug_assertions, feature = "ssr"),
    tracing::instrument(level = "trace", skip_all)
)]
#[component]
pub fn AnimatedShowSwitch<W>(
    /// The children will be shown whenever the condition in the `when` closure returns `true`.
    children: ChildrenFn,
    /// A closure that returns a bool that determines whether this thing runs
    when: W,
    /// Optional CSS class to apply if `when == true`
    #[prop(optional)]
    intro: &'static str,
    /// Optional CSS class to apply if `when == false`
    #[prop(optional)]
    outro: &'static str,
    /// Optional CSS class to apply if `when == false`
    #[prop(optional)]
    fallback_intro: &'static str,
    /// Optional CSS class to apply if `when == true`
    #[prop(optional)]
    fallback_outro: &'static str,
    /// The timeout after which the component will be unmounted if `when == false`
    delay: Duration,
    /// A closure that returns what gets rendered if the when statement is false. By default this is the empty view.
    #[prop(optional, into)]
    fallback: ViewFn,
) -> impl IntoView
where
    W: Fn() -> bool + 'static,
{
    let when = create_memo(move |_| when());
    let show = create_rw_signal(when.get_untracked());

    let handle: StoredValue<Option<TimeoutHandle>> = store_value(None);
    let (cls, fb_cls) = match when.get_untracked() {
        true => (create_rw_signal(intro), create_rw_signal(fallback_outro)),
        false => (create_rw_signal(outro), create_rw_signal(fallback_intro)),
    };

    create_render_effect(move |_| {
        if when.get() {
            // clear any possibly active timer
            if let Some(h) = handle.get_value() {
                h.clear();
            }

            fb_cls.set(fallback_outro);

            handle.set_value(Some(leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    show.set(true);
                    cls.set(intro);
                },
                delay,
            ).expect("set timeout in AnimatedShow")));
        } else {
            // clear any possibly active timer
            if let Some(h) = handle.get_value() {
                h.clear();
            }

            cls.set(outro);
    
            handle.set_value(Some(leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    show.set(false);
                    fb_cls.set(fallback_intro);
                },
                delay,
            ).expect("set timeout in AnimatedShow")));
        }
    });

    on_cleanup(move || {
        if let Some(Some(h)) = handle.try_get_value() {
            h.clear();
        }
    });

    move || match show.get() {
        true => view! { <div class=move || cls.get()>{children()}</div> }.into_view(),
        false => view! { <div class=move || fb_cls.get()>{fallback.run()}</div> }.into_view(),
    }
}
