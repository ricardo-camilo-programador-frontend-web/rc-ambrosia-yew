use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

/// Sets up a sticky scroll observer for step-based scrollytelling.
/// Each `.scroll-step` element gets an `active` class when it enters
/// the middle 40% of the viewport. Only one step is active at a time.
pub fn setup_sticky_scroll() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    let prefers_reduced = window
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false);

    if prefers_reduced {
        if let Ok(elements) = document.query_selector_all(".scroll-step") {
            for i in 0..elements.length() {
                if let Some(node) = elements.get(i) {
                    if let Some(el) = node.dyn_ref::<web_sys::Element>() {
                        let _ = el.class_list().add_1("active");
                    }
                }
            }
        }
        return;
    }

    let callback = Closure::<dyn FnMut(Vec<JsValue>)>::new(move |entries: Vec<JsValue>| {
        for entry_val in entries {
            let entry = IntersectionObserverEntry::from(entry_val);
            let target = entry.target();
            if entry.is_intersecting() {
                let _ = target.class_list().add_1("active");
            } else {
                let _ = target.class_list().remove_1("active");
            }
        }
    });

    let thresholds = js_sys::Array::new();
    thresholds.push(&JsValue::from_f64(0.5));

    let options = IntersectionObserverInit::new();
    options.set_threshold(&thresholds);
    options.set_root_margin("-30% 0px -30% 0px");

    let observer =
        match IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options) {
            Ok(o) => o,
            Err(_) => return,
        };

    if let Ok(elements) = document.query_selector_all(".scroll-step") {
        for i in 0..elements.length() {
            if let Some(node) = elements.get(i) {
                if let Some(el) = node.dyn_ref::<web_sys::Element>() {
                    observer.observe(el);
                }
            }
        }
    }

    callback.forget();
}
