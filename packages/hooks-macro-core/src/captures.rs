use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};

fn iter_map_with_len<R, T, I: Iterator<Item = T>>(
    mut iter: I,
    empty: impl FnOnce() -> R,
    exactly_one: impl FnOnce(T) -> R,
    many: impl FnOnce(std::iter::Chain<std::array::IntoIter<T, 2>, I>) -> R,
) -> R {
    if let Some(first) = iter.next() {
        if let Some(second) = iter.next() {
            many([first, second].into_iter().chain(iter))
        } else {
            exactly_one(first)
        }
    } else {
        empty()
    }
}

fn ty_to_capture_lifetime(lt: &syn::Lifetime) -> TokenStream {
    quote_spanned!(lt.span() => &#lt () )
}

pub(crate) fn capture_lifetimes<'a>(
    lifetimes: impl Iterator<Item = &'a syn::Lifetime>,
    captures_trait_path: impl ToTokens,
) -> Option<TokenStream> {
    let captures = iter_map_with_len(
        lifetimes,
        || None,
        |lt| Some(ty_to_capture_lifetime(lt)),
        |lifetimes| {
            let types = lifetimes.map(ty_to_capture_lifetime);
            Some(quote!((#(#types,)*)))
        },
    );

    captures.map(|captures| {
        quote! {
            #captures_trait_path
            <#captures>
        }
    })
}
