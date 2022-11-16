use quote::ToTokens;
use syn::punctuated::Punctuated;

pub trait IsEmptyOrTrailing {
    type Punct;

    fn is_empty_or_trailing(&self) -> bool;
}

impl<T: IsEmptyOrTrailing> IsEmptyOrTrailing for &T {
    type Punct = T::Punct;

    #[inline]
    fn is_empty_or_trailing(&self) -> bool {
        T::is_empty_or_trailing(self)
    }
}

impl<T, P> IsEmptyOrTrailing for Punctuated<T, P> {
    type Punct = P;

    #[inline]
    fn is_empty_or_trailing(&self) -> bool {
        self.empty_or_trailing()
    }
}

/// [`ToTokens`] makes sure there is a trailing punct unless empty.
pub struct AutoEmptyOrTrailing<T: IsEmptyOrTrailing>(pub T);

impl<T: IsEmptyOrTrailing> IsEmptyOrTrailing for AutoEmptyOrTrailing<T> {
    type Punct = T::Punct;

    #[inline]
    fn is_empty_or_trailing(&self) -> bool {
        true
    }
}

impl<T: IsEmptyOrTrailing> ToTokens for AutoEmptyOrTrailing<T>
where
    T: ToTokens,
    T::Punct: Default + ToTokens,
{
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let e = self.0.is_empty_or_trailing();
        T::to_tokens(&self.0, tokens);
        if !e {
            let punct = T::Punct::default();
            tokens.extend(punct.into_token_stream());
        }
    }

    #[inline]
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let e = self.0.is_empty_or_trailing();
        let mut tokens = T::to_token_stream(&self.0);
        if !e {
            let punct = T::Punct::default();
            tokens.extend(punct.into_token_stream());
        }
        tokens
    }

    #[inline]
    fn into_token_stream(self) -> proc_macro2::TokenStream
    where
        Self: Sized,
    {
        let e = self.0.is_empty_or_trailing();
        let mut tokens = T::into_token_stream(self.0);
        if !e {
            let punct = T::Punct::default();
            tokens.extend(punct.into_token_stream());
        }
        tokens
    }
}
