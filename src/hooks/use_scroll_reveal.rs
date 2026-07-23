use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

/// Sets up an IntersectionObserver that adds the `revealed` class to all
/// elements matching `.reveal`, `.reveal-left`, `.reveal-scale` when they
/// enter the viewport. Respects `prefers-reduced-motion`.
pub fn setup_scroll_reveal() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    let selector = ".reveal, .reveal-left, .reveal-scale";

    // Check reduced motion preference
    let prefers_reduced = window
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false);

    if prefers_reduced {
        if let Ok(elements) = document.query_selector_all(selector) {
            for i in 0..elements.length() {
                if let Some(node) = elements.get(i) {
                    if let Some(el) = node.dyn_ref::<web_sys::Element>() {
                        let _ = el.class_list().add_1("revealed");
                    }
                }
            }
        }
        return;
    }

    let callback = Closure::<dyn FnMut(Vec<JsValue>)>::new(move |entries: Vec<JsValue>| {
        for entry_val in entries {
            let entry = IntersectionObserverEntry::from(entry_val);
            if entry.is_intersecting() {
                let target = entry.target();
                let _ = target.class_list().add_1("revealed");
            }
        }
    });

    let thresholds = js_sys::Array::new();
    thresholds.push(&JsValue::from_f64(0.15));

    let options = IntersectionObserverInit::new();
    options.set_threshold(&thresholds);

    let observer =
        match IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options) {
            Ok(o) => o,
            Err(_) => return,
        };

    if let Ok(elements) = document.query_selector_all(selector) {
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
