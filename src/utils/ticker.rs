use std::time::Duration;

use leptos::{
    create_effect, leptos_dom::helpers::IntervalHandle, set_interval_with_handle, MaybeSignal,
    SignalGet,
};

fn ticker<T, F>(interval_millis: T, f: F, once: bool)
where
    F: Fn() + Clone + 'static,
    T: Into<MaybeSignal<u64>> + 'static,
{
    let interval_millis = interval_millis.into();
    create_effect(move |prev_handle: Option<IntervalHandle>| {
        if once {
            if let Some(prev_handle) = prev_handle {
                prev_handle.clear();
            };
        }

        set_interval_with_handle(f.clone(), Duration::from_millis(interval_millis.get()))
            .expect("could not create interval")
    });
}

pub fn use_interval<T, F>(interval_millis: T, f: F)
where
    F: Fn() + Clone + 'static,
    T: Into<MaybeSignal<u64>> + 'static,
{
    ticker(interval_millis, f, false)
}

pub fn use_timeout<T, F>(interval_millis: T, f: F)
where
    F: Fn() + Clone + 'static,
    T: Into<MaybeSignal<u64>> + 'static,
{
    ticker(interval_millis, f, true)
}
