use proc_macro2::{Delimiter, Group};
use quote::ToTokens;

use super::chain::chain;

#[inline]
pub fn angled(v: impl ToTokens) -> impl ToTokens {
    chain![
        //
        <syn::Token![<]>::default(),
        v,
        <syn::Token![>]>::default(),
    ]
}

#[derive(Clone, Copy)]
pub struct Grouped<T>(pub Delimiter, pub T);

impl<T: ToTokens> ToTokens for Grouped<T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ts = self.1.to_token_stream();
        Group::new(self.0, ts).to_tokens(tokens)
    }

    #[inline]
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let ts = self.1.to_token_stream();
        Group::new(self.0, ts).to_token_stream()
    }

    #[inline]
    fn into_token_stream(self) -> proc_macro2::TokenStream
    where
        Self: Sized,
    {
        let ts = self.1.into_token_stream();
        Group::new(self.0, ts).into_token_stream()
    }
}

#[inline]
pub fn parened<T>(v: T) -> Grouped<T> {
    Grouped(Delimiter::Parenthesis, v)
}
