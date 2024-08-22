use std::time::Duration;
use leptos::*;
use leptos::component;
use leptos_dom::helpers::TimeoutHandle;
use leptos_dom::{DynChild, HydrationCtx};
#[allow(unused)]
use leptos_reactive::SharedContext;
#[cfg(any(feature = "csr", feature = "hydrate"))]
// use leptos_reactive::SignalGet;
// use leptos_reactive::{
//     create_memo, provide_context, SignalGetUntracked, SuspenseContext,
// };
#[cfg(not(any(feature = "csr", feature = "hydrate")))]
use leptos_reactive::{with_owner, Owner};
use std::rc::Rc;

/// Leptos' `<Suspense>` based component 
#[cfg_attr(
    any(debug_assertions, feature = "ssr"),
    tracing::instrument(level = "trace", skip_all)
)]
#[component]
pub fn AnimatedSuspense<V>(
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
    /// Returns a fallback UI that will be shown while `async` [`Resource`](leptos_reactive::Resource)s are still loading. By default this is the empty view.
    #[prop(optional, into)]
    fallback: ViewFn,
    /// Children will be displayed once all `async` [`Resource`](leptos_reactive::Resource)s have resolved.
    children: Rc<dyn Fn() -> V>,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    #[cfg(all(
        feature = "experimental-islands",
        not(any(feature = "csr", feature = "hydrate"))
    ))]
    let no_hydrate = SharedContext::no_hydrate();
    let orig_children = children;
    let context = SuspenseContext::new();

    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    let owner =
        Owner::current().expect("<Suspense/> created with no reactive owner");

    let current_id = HydrationCtx::next_component();

    // provide this SuspenseContext to any resources below it
    // run in a memo so the children are children of this parent
    #[cfg(not(feature = "hydrate"))]
    let children = create_memo({
        let orig_children = Rc::clone(&orig_children);
        move |_| {
            provide_context(context);
            orig_children().into_view()
        }
    });
    #[cfg(feature = "hydrate")]
    let children = create_memo({
        let orig_children = Rc::clone(&orig_children);
        move |_| {
            provide_context(context);
            if SharedContext::fragment_has_local_resources(
                &current_id.to_string(),
            ) {
                HydrationCtx::with_hydration_off({
                    let orig_children = Rc::clone(&orig_children);
                    move || orig_children().into_view()
                })
            } else {
                orig_children().into_view()
            }
        }
    });

    // likewise for the fallback
    let fallback = create_memo({
        move |_| {
            provide_context(context);
            fallback.run()
        }
    });

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    let when = context.ready();
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    let show = create_rw_signal(when.get_untracked());
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    let (cls, fb_cls) = match when.get_untracked() {
        true => (create_rw_signal(intro), create_rw_signal(fallback_outro)),
        false => (create_rw_signal(outro), create_rw_signal(fallback_intro)),
    };
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let handle: StoredValue<Option<TimeoutHandle>> = store_value(None);

        create_render_effect(move |_| {
            if when.get() {
                fb_cls.set(fallback_outro);

                handle.set_value(Some(leptos_dom::helpers::set_timeout_with_handle(
                    move || {
                        show.set(true);
                        cls.set(intro);
                    },
                    delay,
                ).expect("set timeout in AnimatedShow")));
            } else {
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
    }

    let child = DynChild::new({
        move || {
            // pull lazy memo before checking if context is ready
            let children_rendered = children.get_untracked();

            #[cfg(any(feature = "csr", feature = "hydrate"))]
            {
                use leptos::html::div;
                match show.get() {
                    true => div().class(cls.get(), true).child(children_rendered).into_view(),
                    false => div().class(fb_cls.get(), true).child(fallback.get_untracked()).into_view(),
                }
            }
            #[cfg(not(any(feature = "csr", feature = "hydrate")))]
            {
                use leptos_reactive::signal_prelude::*;

                // run the child; we'll probably throw this away, but it will register resource reads
                //let after_original_child = HydrationCtx::peek();

                {
                    // no resources were read under this, so just return the child
                    if context.none_pending() {
                        with_owner(owner, move || {
                            //HydrationCtx::continue_from(current_id);
                            DynChild::new(move || children_rendered.clone())
                                .into_view()
                        })
                    } else if context.has_any_local() {
                        SharedContext::register_local_fragment(
                            current_id.to_string(),
                        );
                        fallback.get_untracked()
                    }
                    // show the fallback, but also prepare to stream HTML
                    else {
                        HydrationCtx::continue_from(current_id);
                        let runtime = leptos_reactive::current_runtime();

                        SharedContext::register_suspense(
                            context,
                            &current_id.to_string(),
                            // out-of-order streaming
                            {
                                let orig_children = Rc::clone(&orig_children);
                                move || {
                                    leptos_reactive::set_current_runtime(
                                        runtime,
                                    );

                                    #[cfg(feature = "experimental-islands")]
                                    let prev_no_hydrate =
                                        SharedContext::no_hydrate();
                                    #[cfg(feature = "experimental-islands")]
                                    {
                                        SharedContext::set_no_hydrate(
                                            no_hydrate,
                                        );
                                    }

                                    let rendered = with_owner(owner, {
                                        move || {
                                            HydrationCtx::continue_from(
                                                current_id,
                                            );
                                            DynChild::new({
                                                move || {
                                                    orig_children().into_view()
                                                }
                                            })
                                            .into_view()
                                            .render_to_string()
                                            .to_string()
                                        }
                                    });

                                    #[cfg(feature = "experimental-islands")]
                                    SharedContext::set_no_hydrate(
                                        prev_no_hydrate,
                                    );

                                    #[allow(clippy::let_and_return)]
                                    rendered
                                }
                            },
                            // in-order streaming
                            {
                                let orig_children = Rc::clone(&orig_children);
                                move || {
                                    leptos_reactive::set_current_runtime(
                                        runtime,
                                    );

                                    #[cfg(feature = "experimental-islands")]
                                    let prev_no_hydrate =
                                        SharedContext::no_hydrate();
                                    #[cfg(feature = "experimental-islands")]
                                    {
                                        SharedContext::set_no_hydrate(
                                            no_hydrate,
                                        );
                                    }

                                    let rendered = with_owner(owner, {
                                        move || {
                                            HydrationCtx::continue_from(
                                                current_id,
                                            );
                                            DynChild::new({
                                                move || {
                                                    orig_children().into_view()
                                                }
                                            })
                                            .into_view()
                                            .into_stream_chunks()
                                        }
                                    });

                                    #[cfg(feature = "experimental-islands")]
                                    SharedContext::set_no_hydrate(
                                        prev_no_hydrate,
                                    );

                                    #[allow(clippy::let_and_return)]
                                    rendered
                                }
                            },
                        );

                        // return the fallback for now, wrapped in fragment identifier
                        fallback.get_untracked()
                    }
                }
            }
        }
    })
    .into_view();
    let core_component = match child {
        leptos_dom::View::CoreComponent(repr) => repr,
        _ => unreachable!(),
    };

    HydrationCtx::continue_from(current_id);
    HydrationCtx::next_component();

    leptos_dom::View::Suspense(current_id, core_component)
}