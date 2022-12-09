#![deny(clippy::undocumented_unsafe_blocks)]

use render::{ContextAndState, Dom};

use crate::render::button::{button, Button};

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
            "help",
            "please",
            Button(
                button()
                    .children(format!("state = {}", state))
                    .on_click(move |_: &_| {
                        web_sys::console::log_1(&"on_click".into());
                        updater.replace_with_fn_pointer(|v| *v + 1);
                    }),
            ),
        ))
    }
}

fn Counter() -> impl render::UpdateRenderState<Dom> {
    render::HookElement(Counter::use_impl_render)
}

// pub use hook_component::*;
// pub use hook_element::*;
// pub use pin_macro::*;

// use async_executor::Task;
// use dom::{render_element, Dom, RenderState, UpdateRenderState};

// use std::{any::Any, pin::Pin};

// use gloo::events::EventListener;
// use hooks::{hook, memo_with, DataAndDep, Hook, HookExt, HookLifetime};
// use wasm_bindgen::UnwrapThrowExt;

// use crate::dom::{AnyContext, Context};

// // pub use render_to::*;

// // use crate::dom::element::{Text, TextProps};

// pub mod dom;

// struct CounterInner {}

// impl UpdateRenderState<Dom<'_>> for CounterInner {
//     type State = ();

//     fn update_render_state(self, ctx: Context<Self::State>) {
//         let document = ctx.document.clone();
//         let current_parent = ctx.current_parent.clone();

//         let fut = Self::use_render().into_render(document, current_parent, std::future::pending());
//         wasm_bindgen_futures::spawn_local(fut);
//     }
// }

// impl CounterInner {
//     #[hook(args_generics = "'ctx")]
//     fn use_impl_render(ctx: AnyContext<'ctx>) -> Context<'ctx, impl RenderState + 'static> {
//         let mut ctx = ctx.downcast_into().unwrap();

//         let (state, updater) = hooks::use_state(0);

//         let updater = updater.clone();

//         ctx.render(
//             dom::element::button()
//                 .children(format!("state = {}", state))
//                 .on_click(move |_: &_| updater.replace_with_fn_box(|v| *v + 1)),
//         );

//         ctx
//     }

//     fn use_element() -> HookComponent<
//         impl for<'ctx> Hook<(AnyContext<'ctx>,), Value = Context<'ctx, impl RenderState + 'static>>,
//     > {
//         HookComponent::new(Self::use_impl_render())
//     }

//     #[inline]
//     fn use_render() -> HookElement<
//         impl for<'c> Hook<(AnyContext<'c>,), Value = Context<'c, impl RenderState + 'static>>,
//     > {
//         HookElement::new(CounterInner::use_impl_render())
//     }
// }

// #[inline]
// pub fn CounterInner() -> impl UpdateRenderState {
//     HookFn(CounterInner::use_render)
// }

// struct Counter {}

// impl Counter {
//     #[hook(args_generics = "'ctx")]
//     fn use_impl_render(ctx: AnyContext<'ctx>) -> Context<'ctx, impl RenderState + 'static> {
//         let mut ctx = ctx.downcast_into().unwrap();

//         let render_id = hooks::use_mut_default::<i32>();

//         let counter = CounterInner::use_element();
//         // CounterInner::use_render(Context {
//         //     document: ctx.document,
//         //     current_parent: ctx.current_parent,
//         //     current_rendered: &mut current_rendered.1,
//         // });

//         web_sys::console::log_1(&format!("render_id = {}", render_id).into());

//         ctx.render((
//             // CounterInner(),
//             counter,
//             if *render_id % 2 == 0 {
//                 web_sys::console::log_1(&"render some".into());
//                 Some(format!("render_id = {}", render_id))
//             } else {
//                 None
//             },
//         ));

//         *render_id += 1;

//         ctx
//     }

//     fn use_element() -> HookComponent<
//         impl for<'ctx> Hook<(AnyContext<'ctx>,), Value = Context<'ctx, impl RenderState + 'static>>,
//     > {
//         HookComponent::new(Self::use_impl_render())
//     }
// }

// #[inline]
// pub fn Counter() -> impl UpdateRenderState {
//     HookFn(|| HookElement::new(CounterInner::use_impl_render()))
// }

// fn main() {
//     wasm_bindgen_futures::spawn_local(async {
//         let window = web_sys::window().unwrap_throw();
//         let document = window.document().unwrap_throw();

//         let current_parent = document.get_element_by_id("main").unwrap_throw();

//         // let el = Counter::use_element();

//         // el.into_render(&document, &current_parent).await;
//         render_element(CounterInner, &document, &current_parent).await;
//         // (el);
//     })
// }

fn main() {
    wasm_bindgen_futures::spawn_local(async {
        let window = web_sys::window().unwrap();

        let document = window.document().unwrap();
        let current_parent = document.get_element_by_id("main").unwrap();

        Dom {
            document,
            current_parent: current_parent.into(),
        }
        .render_element(Counter, std::future::pending())
        .await;
    })
}
