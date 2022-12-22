use hooks_frender::{element, Counter, CounterWithInitialValue, Dom};

fn main() {
    wasm_bindgen_futures::spawn_local(async {
        let window = web_sys::window().unwrap();

        let document = window.document().unwrap();
        let current_parent = document.get_element_by_id("main").unwrap();

        Dom::new(document, current_parent)
            .render_get_element(
                || {
                    (
                        element!(CounterWithInitialValue().initial_value(8)),
                        element!(Counter()),
                        element!(CounterWithInitialValue().initial_value(-8)),
                    )
                },
                std::future::pending(),
            )
            .await;
    })
}
