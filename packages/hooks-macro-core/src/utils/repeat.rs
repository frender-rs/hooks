use quote::ToTokens;

#[derive(Clone, Copy)]
pub struct Repeat<V>(pub V, pub usize);

impl<V: ToTokens> ToTokens for Repeat<V> {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut times = self.1;

        while times > 0 {
            times -= 1;
            self.0.to_tokens(tokens);
        }
    }
}
