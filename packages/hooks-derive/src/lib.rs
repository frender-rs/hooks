use proc_macro::TokenStream;

use hooks_derive_core::{
    darling::FromMeta,
    proc_macro2,
    quote::ToTokens,
    syn::{parse_macro_input, AttributeArgs, ItemFn},
};

// TODO: move to a new crate
#[proc_macro_attribute]
pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
    use hooks_derive_core::syn;
    let attr_args = parse_macro_input!(args as AttributeArgs);

    syn::Macro {
        path: syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::from_iter([
                // TODO: change to `::frender`
                syn::PathSegment::from(<syn::Token![crate]>::default()),
                syn::PathSegment::from(syn::Ident::new(
                    "def_component",
                    proc_macro2::Span::call_site(),
                )),
            ]),
        },
        bang_token: Default::default(),
        delimiter: syn::MacroDelimiter::Brace(Default::default()),
        tokens: input.into(),
    }
    .to_token_stream()
    .into()
}

#[proc_macro_attribute]
pub fn hook(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);

    let args = match ::hooks_derive_core::HookArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let item_fn = parse_macro_input!(input as ItemFn);

    let (item_fn, error) = args.transform_item_fn(item_fn);

    let item_fn = item_fn.into_token_stream();

    if let Some(error) = error {
        proc_macro2::TokenStream::from_iter([item_fn, error.write_errors()])
    } else {
        item_fn
    }
    .into()
}
