use bg::build;
use hooks_frender::{rsx, Counter, CounterWithInitialValue, Dom};

fn main() {
    wasm_bindgen_futures::spawn_local(async {
        let window = web_sys::window().unwrap();

        let document = window.document().unwrap();
        let current_parent = document.get_element_by_id("main").unwrap();

        Dom::new(document, current_parent)
            .render_get_element(
                || build!(CounterWithInitialValue()).finish_builder(),
                std::future::pending(),
            )
            .await;
    })
}
