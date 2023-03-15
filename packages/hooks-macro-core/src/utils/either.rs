use quote::ToTokens;

pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A: ToTokens, B: ToTokens> ToTokens for Either<A, B> {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Either::A(a) => A::to_tokens(a, tokens),
            Either::B(b) => B::to_tokens(b, tokens),
        }
    }

    #[inline]
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Either::A(a) => A::to_token_stream(a),
            Either::B(b) => B::to_token_stream(b),
        }
    }

    #[inline]
    fn into_token_stream(self) -> proc_macro2::TokenStream
    where
        Self: Sized,
    {
        match self {
            Either::A(a) => A::into_token_stream(a),
            Either::B(b) => B::into_token_stream(b),
        }
    }
}
