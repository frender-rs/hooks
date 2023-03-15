#![allow(dead_code)]

use hooks_macro_core::{quote::ToTokens, syn};

pub fn pretty_item_fn(item_fn: syn::ItemFn) -> String {
    // re-parse code due to usage of Expr::Verbatim and Type::Verbatim
    // TODO: remove Verbatim in hooks-macro-core or support formatting Verbatim
    let item_fn: syn::ItemFn = syn::parse2(item_fn.into_token_stream()).unwrap();
    prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![item_fn.into()],
    })
}

pub fn hook_uninitialized_default<H: hooks::UpdateHookUninitialized>(_: H) -> H::Uninitialized {
    Default::default()
}

pub type HookUninitialized<H> = <H as hooks::UpdateHookUninitialized>::Uninitialized;

macro_rules! hook_macro {
    ($($tt:tt)*) => {
        $crate::utils::impl_hook_macro! { $($tt)* }

        $($tt)*
    };
}

macro_rules! impl_hook_macro {
    (#[hook $(( $($meta:tt)* ))? ] fn $fn_name:ident $($tt:tt)+ ) => {
        #[allow(non_camel_case_types)]
        struct $fn_name {}
        impl $fn_name {
            fn hook_impl_code() -> String {
                $crate::utils::pretty_item_fn(Self::hook_macro_output())
            }

            fn hook_args() -> ::hooks_macro_core::HookArgs {
                ::hooks_macro_core::HookArgs::from_punctuated_meta_list(
                    ::hooks_macro_core::syn::parse_quote! {
                        $( $($meta)* )?
                    }
                ).unwrap()
            }

            fn hook_macro_output() -> ::hooks_macro_core::syn::ItemFn {
                let (target, error) = Self::hook_args().transform_item_fn(
                    ::hooks_macro_core::syn::parse_quote! {
                        fn $fn_name $($tt)+
                    }
                );

                error.ok_or(()).unwrap_err();

                target
            }
        }

        ::insta::assert_display_snapshot!(
            ::core::stringify!($fn_name),
            $fn_name::hook_impl_code()
        );
    };
}

pub(crate) use {hook_macro, impl_hook_macro};

macro_rules! assert_return_ty {
    ($e:expr => $ty:ty) => {{
        fn assert_impl() -> $ty {
            $e
        }

        assert_impl()
    }};
}

pub(crate) use assert_return_ty;
