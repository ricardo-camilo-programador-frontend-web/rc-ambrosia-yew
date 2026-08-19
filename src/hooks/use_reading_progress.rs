use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// Updates the reading progress bar based on scroll position.
/// Uses requestAnimationFrame loop for smooth, throttled updates.
/// Persists progress to localStorage.
pub fn setup_reading_progress() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    let progress_bar = match document.get_element_by_id("reading-progress") {
        Some(el) => el,
        None => return,
    };

    let win = window.clone();
    let bar = progress_bar.clone();

    let callback = Closure::<dyn FnMut()>::new(move || {
        let scroll_y = win.scroll_y().unwrap_or(0.0);
        let scroll_height = win
            .inner_height()
            .ok()
            .and_then(|h| h.as_f64())
            .unwrap_or(800.0);

        let doc_height = match win.document() {
            Some(doc) => {
                let body = doc.body().map(|b| b.scroll_height() as f64).unwrap_or(1.0);
                if body > scroll_height {
                    body
                } else {
                    1.0
                }
            }
            None => 1.0,
        };

        let max_scroll = (doc_height - scroll_height).max(1.0);
        let progress = ((scroll_y / max_scroll) * 100.0).clamp(0.0, 100.0);

        let _ = bar.set_attribute("style", &format!("width: {:.0}%", progress));

        if let Ok(Some(storage)) = win.local_storage() {
            let _ = storage.set_item("ambrosia-reading-progress", &format!("{:.0}", progress));
        }
    });

    // Use setInterval via window for throttled scroll updates (every 100ms)
    let cb_ref = callback.as_ref().unchecked_ref::<js_sys::Function>();
    let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(cb_ref, 100);
    callback.forget();
}
