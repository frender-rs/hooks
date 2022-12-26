#[macro_export]
macro_rules! def_component {
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident () $body:tt
    ) => {
        $crate::def_component! {
            $(#$attr)*
            $vis fn $name (_ctx: _) $body
        }
    };
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident ($ctx_arg:tt : _ $(,)?) {$($impl_code:tt)*}
    ) => {
        $crate::builder! {
            $(#$attr)*
            $vis struct $name($crate::bg::Empty);

            mod impl_render {
                use super::super::*;
                use $crate::hooks::core as __private_hooks_core;

                #[$crate::hook(args_generics = "'render_ctx", hooks_core_path = "__private_hooks_core")]
                pub fn $name(
                    $ctx_arg: $crate::ContextAndState<'render_ctx, Dom, dyn std::any::Any>,
                ) -> $crate::ContextAndState<'render_ctx, Dom, impl $crate::RenderState + 'static> {
                    let $ctx_arg = $ctx_arg.downcast_state().unwrap();

                    $($impl_code)*
                }
            }

            #[inline]
            pub fn build_element<TypesDef: ?Sized + $crate::bg::Empty::ValidTypes>(
                _: Building<TypesDef>,
            ) -> $crate::HookElement<
                impl $crate::FnOnceOutputElementHook<
                    $crate::Dom,
                    RenderState = impl $crate::RenderState + 'static,
                > + ::core::marker::Copy + 'static
            > {
                $crate::HookElement(impl_render::$name)
            }
        }
    };
    (
        $(#$attr:tt)*
        $vis:vis fn $name:ident ($ctx_arg:ident : _, $props_arg:ident : & $($props_name:ident)? $(:: $props_p:ident)* $(,)?) {$($impl_code:tt)*}
    ) => {
        $crate::builder! {
            $(#$attr)*
            $vis struct $name($($props_name)? $(:: $props_p)*);

            mod impl_render {
                use super::super::*;
                use $crate::hooks::core as __private_hooks_core;

                #[$crate::hook(args_generics = "'render_ctx", hooks_core_path = "__private_hooks_core")]
                pub fn $name<TypesDef: ?Sized + $($props_name)? $(:: $props_p)* ::ValidTypes>(
                    $ctx_arg: $crate::ContextAndState<'render_ctx, Dom, dyn std::any::Any>,
                    $props_arg: &$($props_name)? $(:: $props_p)* ::Data<TypesDef>,
                ) -> $crate::ContextAndState<'render_ctx, Dom, impl $crate::RenderState + 'static> {
                    let $ctx_arg = $ctx_arg.downcast_state().unwrap();

                    $($impl_code)*
                }
            }

            mod build_element {
                use super::super::*;
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
                    $crate::HookElementWithProps(super::impl_render::$name, props)
                }
            }

            pub use build_element::build_element;
        }
    };
}
