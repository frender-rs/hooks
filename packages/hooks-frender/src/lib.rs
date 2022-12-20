#![deny(clippy::undocumented_unsafe_blocks)]

mod macros;
mod props;
mod render;

pub use macros::*;
pub use props::*;
pub use render::*;

mod utils;

use bg::{builder, Maybe, Unspecified};

builder! {
    struct CounterWithInitialValueProps {
        initial_value[? i32],
    }
}

#[allow(non_snake_case)]
pub mod CounterWithInitialValue {
    mod reuse {
        use super::super::*;
        pub use CounterWithInitialValueProps::{
            prelude, Building, Types, TypesInitial, ValidTypes,
        };
    }

    pub use reuse::{prelude, Building, Types, TypesInitial, ValidTypes};

    mod struct_data {
        use super::super::*;
        pub struct CounterWithInitialValue<
            TypeDefs: ?::core::marker::Sized + CounterWithInitialValueProps::Types,
        >(pub CounterWithInitialValueProps::Data<TypeDefs>);
    }

    pub use struct_data::CounterWithInitialValue as Data;

    pub type DataInitial = Data<TypesInitial>;

    #[inline]
    pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
        building: Building<TypeDefs>,
    ) -> Data<TypeDefs> {
        use super::*;
        self::Data(CounterWithInitialValueProps::build(building))
    }

    #[inline]
    pub fn valid<TypeDefs: ?::core::marker::Sized + ValidTypes>(
        building: Building<TypeDefs>,
    ) -> Data<TypeDefs> {
        build(building)
    }

    pub mod fn_uninitialized {
        #[inline]
        pub fn CounterWithInitialValue() -> super::Building<super::TypesInitial> {
            use super::super::*;
            CounterWithInitialValueProps()
        }
    }
}

pub use CounterWithInitialValue::fn_uninitialized::CounterWithInitialValue;

impl<TypesDef: 'static + ?Sized + CounterWithInitialValueProps::ValidTypes>
    CounterWithInitialValue::Data<TypesDef>
{
    // impl UpdateRenderState<Dom>
    pub fn finish_builder(
        self,
    ) -> HookElementWithProps<
        impl FnOnceOutputElementHookWithProps<
                Dom,
                CounterWithInitialValueProps::Data<TypesDef>,
                RenderState = impl render::RenderState + 'static,
            > + Copy
            + 'static,
        CounterWithInitialValueProps::Data<TypesDef>,
    > {
        #[hooks::hook(args_generics = "'render_ctx")]
        pub fn use_impl_render<TypesDef: ?Sized + CounterWithInitialValueProps::ValidTypes>(
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

        HookElementWithProps(use_impl_render, self.0)
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
