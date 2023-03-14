use proc_macro2::{Span, TokenStream};
use quote::{quote_spanned, ToTokens};

pub struct DetectedHooksTokens {
    pub fn_arg_data_pat: TokenStream,
    pub fn_stmts_extract_data: Option<TokenStream>,
}

pub struct DetectedHook {
    pub ident: syn::Ident,
}

pub fn detected_hooks_to_tokens(
    mut used_hooks: Vec<DetectedHook>,
    hooks_core_path: impl ToTokens,
    span: Span,
) -> DetectedHooksTokens {
    match used_hooks.len() {
        0 => DetectedHooksTokens {
            fn_arg_data_pat: quote_spanned! {span=>
                _: ::core::pin::Pin<&mut #hooks_core_path::HookTuple::<()>>
            },
            fn_stmts_extract_data: None,
        },
        1 => {
            let DetectedHook { ident } = used_hooks.pop().unwrap();
            DetectedHooksTokens {
                fn_arg_data_pat: quote_spanned! {span=>
                    #ident: ::core::pin::Pin<&mut _>
                },
                fn_stmts_extract_data: None,
            }
        }
        _ => {
            let ident_hooks_data = syn::Ident::new("__hooks_hook_data", span);

            let pat_hook_ids = {
                let used_id = used_hooks.iter().map(|h| &h.ident);
                quote_spanned!(span=> (#(#used_id,)*))
            };

            let used_id = used_hooks.iter().map(|h| &h.ident);

            let impl_extract_hooks_data = quote_spanned! {span=>
                // SAFETY: pin projection
                let #pat_hook_ids = unsafe {
                    let $crate::HookTuple(#pat_hook_ids) = ::core::pin::Pin::get_unchecked_mut(#ident_hooks_data);
                    (#(
                        ::core::pin::Pin::new_unchecked(#used_id),
                    )*)
                };
            };

            DetectedHooksTokens {
                fn_arg_data_pat: quote_spanned! {span=>
                    #ident_hooks_data: ::core::pin::Pin<&mut _>
                },
                fn_stmts_extract_data: Some(impl_extract_hooks_data),
            }
        }
    }
}
