#![deny(clippy::undocumented_unsafe_blocks)]

mod macros;
mod props;
mod render;

pub use macros::*;
pub use props::*;
pub use render::*;

mod utils;

pub use bg;

use bg::{builder, Maybe};

builder! {
    struct CounterWithInitialValueProps {
        initial_value[? i32],
    }
}

builder! {
    pub struct CounterWithInitialValue(CounterWithInitialValueProps);

    mod build_element {
        use super::super::*;

        pub fn build_element<TypesDef: 'static + ?Sized + CounterWithInitialValueProps::ValidTypes>(
            super::Building(props): super::Building<TypesDef>,
        ) -> crate::HookElementWithProps<
            impl crate::FnOnceOutputElementHookWithProps<
                    crate::Dom,
                    CounterWithInitialValueProps::Data<TypesDef>,
                    RenderState = impl crate::RenderState + 'static,
                > + Copy
                + 'static,
            CounterWithInitialValueProps::Data<TypesDef>,
        > {
            // impl UpdateRenderState<Dom>
            {
                crate::HookElementWithProps(super::DataInitial::use_impl_render, props)
            }
        }
    }

    pub use build_element::build_element;
}

impl CounterWithInitialValue::DataInitial {
    #[hooks::hook(args_generics = "'render_ctx")]
    fn use_impl_render<TypesDef: ?Sized + CounterWithInitialValueProps::ValidTypes>(
        ctx: ContextAndState<'render_ctx, Dom, dyn std::any::Any>,
        props: &CounterWithInitialValueProps::Data<TypesDef>,
    ) -> ContextAndState<'render_ctx, Dom, impl render::RenderState + 'static> {
        let ctx = ctx.downcast_state().unwrap();

        let (state, updater) =
            hooks::use_state_with(|| props.initial_value.as_some().copied().unwrap_or(4));

        let updater = updater.clone();

        web_sys::console::log_1(&"render".into());

        ctx.render((
            if *state % 3 == 0 { None } else { Some("help ") },
            // "please",
            render::button()
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

pub trait Counter {}

pub fn Counter() -> impl render::UpdateRenderState<Dom> + Copy {
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

    render::HookElement(use_impl_render)
}
