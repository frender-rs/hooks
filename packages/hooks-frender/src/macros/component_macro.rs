#[macro_export]
macro_rules! component {
    ($vis:vis fn $name:ident ($ctx_arg:ident : (), $props_arg:ident : & $($props_name:ident)? $(:: $props_p:ident)* $(,)?) {$($impl_code:tt)*}) => {
        $crate::builder! {
            $vis struct $name($($props_name)? $(:: $props_p)*);

            mod build_element {
                use super::super::*;
                use $crate::hooks::core as __private_hooks_core;

                #[$crate::hook(args_generics = "'render_ctx", hooks_core_path = "__private_hooks_core")]
                fn impl_render<TypesDef: ?Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                    $ctx_arg: $crate::ContextAndState<'render_ctx, Dom, dyn std::any::Any>,
                    $props_arg: &$($props_name)? $(:: $props_p)* ::Data<TypesDef>,
                ) -> $crate::ContextAndState<'render_ctx, Dom, impl render::RenderState + 'static> {
                    let $ctx_arg = $ctx_arg.downcast_state().unwrap();

                    $($impl_code)*
                }

                pub fn build_element<TypesDef: 'static + ?Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                    super::Building(props): super::Building<TypesDef>,
                ) -> $crate::HookElementWithProps<
                    impl $crate::FnOnceOutputElementHookWithProps<
                            $crate::Dom,
                            $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
                            RenderState = impl $crate::RenderState + 'static,
                        > + Copy
                        + 'static,
                        $($props_name)? $(:: $props_p)* ::Data<TypesDef>,
                > {
                    $crate::HookElementWithProps(impl_render, props)
                }
            }

            pub use build_element::build_element;
        }
    };
}
