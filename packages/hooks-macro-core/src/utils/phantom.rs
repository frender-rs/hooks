use darling::ToTokens;
use quote::quote;

use super::{either::Either, group::angled};

pub struct PhantomOfTy<T>(pub T);

impl<T: ToTokens> ToTokens for PhantomOfTy<T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote!(::core::marker::PhantomData).to_tokens(tokens);
        angled(&self.0).to_tokens(tokens);
    }
}

pub struct RefOfLt<T>(pub T);

impl<T: ToTokens> ToTokens for RefOfLt<T> {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        <syn::Token![&]>::default().to_tokens(tokens);
        self.0.to_tokens(tokens);
        tokens.extend(quote!(()));
    }
}

pub fn make_phantom_or_ref(gp: &syn::GenericParam) -> Option<impl ToTokens + '_> {
    match gp {
        syn::GenericParam::Type(tp) => Some(Either::A(PhantomOfTy(&tp.ident))),
        syn::GenericParam::Lifetime(lt) => Some(Either::B(RefOfLt(&lt.lifetime))),
        syn::GenericParam::Const(_) => None,
    }
}
