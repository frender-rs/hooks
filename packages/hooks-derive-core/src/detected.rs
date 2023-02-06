use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};

pub struct DetectedHooksTokens {
    pub data_expr: TokenStream,
    pub fn_arg_data_pat: TokenStream,
    pub fn_stmts_extract_data: Option<TokenStream>,
}

pub struct DetectedHookCall {
    pub ident: syn::Ident,
    pub expr_call: syn::ExprCall,
}

/// `ty_no_data` defaults to `_`
pub fn detected_hooks_to_tokens(
    mut used_hooks: Vec<DetectedHookCall>,
    hooks_core_path: impl ToTokens,
    expr_no_data: TokenStream,
    ty_no_data: Option<TokenStream>,
    span: Span,
) -> DetectedHooksTokens {
    match used_hooks.len() {
        0 => DetectedHooksTokens {
            data_expr: expr_no_data,
            fn_arg_data_pat: {
                let ty_no_data = ty_no_data.unwrap_or_else(|| quote!(_));
                quote! {_: ::core::pin::Pin<&mut #ty_no_data>}
            },
            fn_stmts_extract_data: None,
        },
        1 => {
            let h = used_hooks.pop().unwrap();
            DetectedHooksTokens {
                data_expr: h.expr_call.into_token_stream(),
                fn_arg_data_pat: h.ident.into_token_stream(),
                fn_stmts_extract_data: None,
            }
        }
        _ => {
            let expr_hooks_data = {
                let mut used_hooks = used_hooks.iter().map(|h| &h.expr_call);
                let first = used_hooks.next().unwrap();
                let second = used_hooks.next().unwrap();

                quote_spanned! { span =>
                    #hooks_core_path ::hook_pair::HookPair::new(#first , #second)
                        #( .chain( #used_hooks ) )*
                }
            };

            let ident_hooks_data = syn::Ident::new("__hooks_data", span);

            let impl_extract_hooks_data = {
                let mut stmts = Vec::with_capacity(used_hooks.len());

                while let Some(used_hook) = used_hooks.pop() {
                    let used_hook_ident = used_hook.ident;
                    let stmt = if !used_hooks.is_empty() {
                        quote_spanned! { span =>
                            let (#ident_hooks_data, #used_hook_ident) = #ident_hooks_data.pin_project();
                        }
                    } else {
                        // This is the first element
                        quote_spanned! { span =>
                            let #used_hook_ident = #ident_hooks_data;
                        }
                    };
                    stmts.push(stmt);
                }

                proc_macro2::TokenStream::from_iter(stmts)
            };

            DetectedHooksTokens {
                data_expr: expr_hooks_data,
                fn_arg_data_pat: ident_hooks_data.into_token_stream(),
                fn_stmts_extract_data: Some(impl_extract_hooks_data),
            }
        }
    }
}
