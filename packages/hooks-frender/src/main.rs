#![deny(clippy::undocumented_unsafe_blocks)]

use render::{ContextAndState, Dom, EndBuilderAsSelf, NextNodePosition};

use crate::render::{button::button, EndBuilder};

mod props;
mod render;

mod utils;

pub struct Counter {}

impl Counter {
    #[hooks::hook(args_generics = "'render_ctx")]
    pub fn use_impl_render(
        ctx: ContextAndState<'render_ctx, Dom, dyn std::any::Any>,
    ) -> ContextAndState<'render_ctx, Dom, impl render::RenderState + 'static> {
        let ctx = ctx.downcast_state().unwrap();

        let (state, updater) = hooks::use_state(0);

        let updater = updater.clone();

        web_sys::console::log_1(&"render".into());

        ctx.render((
            if *state % 3 == 0 { None } else { Some("help ") },
            // "please",
            button()
                .on_click(move |_: &_| {
                    web_sys::console::log_1(&"on_click".into());
                    updater.replace_with_fn_pointer(|v| *v + 1);
                })
                .children(format!("state = {}", state))
                .end_builder(),
            if *state % 2 == 0 {
                Some("state is even")
            } else {
                None
            },
            if *state % 2 == 1 {
                Some(format!("{state} is odd"))
            } else {
                None
            },
            " Last",
            // Box::new(ElementFnOnce(Counter)),
            // rsx!(<button
            //         on_click={move |_: &_| {
            //             web_sys::console::log_1(&"on_click".into());
            //             updater.replace_with_fn_pointer(|v| *v + 1);
            //         }}
            //         children={format!("state = {}", state)}
            // />
            // ),
        ))
    }
}

pub fn Counter() -> impl render::UpdateRenderState<Dom> + Copy + EndBuilderAsSelf {
    render::HookElement(Counter::use_impl_render)
}

fn main() {
    wasm_bindgen_futures::spawn_local(async {
        let window = web_sys::window().unwrap();

        let document = window.document().unwrap();
        let current_parent = document.get_element_by_id("main").unwrap();

        Dom::new(document, current_parent)
            .render_element(rsx!(<Counter />), std::future::pending())
            .await;
    })
}
