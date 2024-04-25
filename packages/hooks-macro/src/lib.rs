use proc_macro::TokenStream;

use hooks_macro_core::{
    darling::{self, ast::NestedMeta, FromMeta},
    proc_macro2,
    quote::ToTokens,
    syn::{parse_macro_input, ItemFn},
};

#[proc_macro_attribute]
pub fn hook(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };

    let args = match ::hooks_macro_core::HookArgs::from_list(&attr_args) {
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
