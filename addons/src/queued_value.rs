use leptos::*;
use std::time::Duration;
use leptos_dom::helpers::TimeoutHandle;

/// Сontainer for delayed value update. Useful for components that use animation, 
/// in particular for those situations when it's required to renew a [`Resource`](leptos_reactive::Resource), 
/// [`Signal`](leptos_reactive::Signal) or [`Memo`](leptos_reactive::Memo) only after a certain period of time has passed.
///
/// ```
/// use leptos::*;
/// use std::time::Duration;
///
/// let (count, set_count) = create_signal(0);
/// let on_click = move |_| set_count.update(|count| *count += 1);
///
/// let slug = SluggishValue::generate(String::new(), Duration::from_millis(250));
///
/// view! {
///   <button on:click=on_click>"Click Me"</button>
///   <h1>"Count: " {count}</h1>
///   <h1>"Delayed count: " {slug.with(|v| v.current())}</h1>
/// };
/// ```
#[allow(dead_code)]
pub struct SluggishValue<S>
where
    S: PartialEq + Clone + 'static,
{
    current: RwSignal<S>,
    queued: RwSignal<Option<S>>,
    handle: StoredValue<Option<TimeoutHandle>>,
}

impl<S> SluggishValue<S>
where
    S: PartialEq + Clone + 'static,
{
    pub fn current(&self) -> S {
        self.current.get()
    }

    pub fn queued(&self) -> Option<S> {
        self.queued.get()
    }

    pub fn queue(&self, value: S) {
        self.queued.set(Some(value));
    }

    pub fn generate(initial_value: S, delay: Duration) -> RwSignal<SluggishValue<S>> {
        let current = create_rw_signal(initial_value);
        let queued = create_rw_signal(Option::<S>::None);

        let handle: StoredValue<Option<TimeoutHandle>> = store_value(None);

        create_render_effect(move |_| {
            if let Some(new_value) = queued.get() {
                if let Some(h) = handle.get_value() {
                    h.clear();
                }

                queued.set_untracked(None);
        
                handle.set_value(Some(leptos_dom::helpers::set_timeout_with_handle(
                    move || {
                        current.set(new_value);
                    },
                    delay,
                ).expect("set timeout in SluggishValue")));
            }
        });

        create_rw_signal(SluggishValue {
            current,
            queued,
            handle,
        })
    }
}