use rc_ambrosia_yew::app::App;

fn app_root() -> web_sys::Element {
    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("document");
    if let Some(root) = document.get_element_by_id("app") {
        return root;
    }
    document.body().expect("body").into()
}

fn main() {
    // Mount inside #app so the study-project script on <body> is not wiped.
    yew::Renderer::<App>::with_root(app_root()).render();
}
