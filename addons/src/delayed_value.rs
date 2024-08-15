use std::time::Duration;
use leptos::*;
use leptos::logging::log;
use std::rc::Rc;
use leptos_dom::helpers::TimeoutHandle;

/// Сontainer for delayed value update. Useful for components that use animation, 
/// in particular for those situations when it's required to renew a [`Resource`](leptos_reactive::Resource), 
/// [`Signal`](leptos_reactive::Signal) or [`Memo`](leptos_reactive::Memo) only after a certain period of time has passed.
///
/// ```
/// use leptos::*;
/// use std::rc::Rc;
/// use std::time::Duration;
///
/// let (count, set_count) = create_signal(0);
/// let on_click = move |_| set_count.update(|count| *count += 1);
///
/// let count_delayer_container = DelayedValue::default();
/// let delayed_count = count_delayer_container.queue(Duration::from_secs(1), Rc::new(count));
///
/// view! {
///   <button on:click=on_click>"Click Me"</button>
///   <h1>"Count: " {count}</h1>
///   <h1>"Delayed count: " {delayed_count}</h1>
/// };
/// ```
pub struct DelayedValue {
    prev_view: RwSignal<Option<View>>,
}

impl Default for DelayedValue {
    fn default() -> Self {
        Self {
            prev_view: create_rw_signal(None),
        }
    }
}

impl DelayedValue {
    pub fn queue<V>(self, delay: Duration, new_view: Rc<dyn Fn() -> V>) -> Memo<View>
    where
        V: IntoView + 'static
    {

        let handle: StoredValue<Option<TimeoutHandle>> = store_value(None);

        let new_view = create_memo({
            let orig_view = Rc::clone(&new_view);
            move |_| {
                orig_view().into_view()
            }
        });

        let children = create_memo({
            move |_| {
                log!("children > create_memo > update");
                if let Option::Some(prev_view) = self.prev_view.get() {
                    prev_view
                } else {
                    new_view.get()
                }
            }
        });
        
        create_render_effect(move |_| {
            let new_view = new_view.get();

            if let Some(h) = handle.get_value() {
                h.clear();
            }

            log!("render_effect");
    
            handle.set_value(Some(leptos_dom::helpers::set_timeout_with_handle(
                move || {
                    log!("render_effect > set_value > call");
                    self.prev_view.set(Some(new_view));
                },
                delay,
            ).expect("set timeout in Delayed Value")));
        });

        on_cleanup(move || {
            if let Some(Some(h)) = handle.try_get_value() {
                h.clear();
            }
        });

        children
    }
}