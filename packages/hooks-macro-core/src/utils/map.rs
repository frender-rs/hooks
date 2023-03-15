use quote::ToTokens;

#[derive(Clone, Copy)]
pub struct MapToTokens<T, F>(pub T, pub F);

impl<T, F, Iter> ToTokens for MapToTokens<T, F>
where
    F: Fn(&T) -> Iter,
    Iter: IntoIterator,
    Iter::Item: ToTokens,
{
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let iter = (self.1)(&self.0);
        for i in iter {
            tokens.extend(i.into_token_stream());
        }
    }
}

#[inline]
pub fn map_to_tokens<T, F: Fn(&T) -> Iter, Iter>(value: T, f: F) -> impl ToTokens
where
    Iter: IntoIterator,
    Iter::Item: ToTokens,
{
    MapToTokens(value, f)
}
