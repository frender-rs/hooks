use darling::ToTokens;

#[derive(Clone, Copy)]
pub struct Chain<A, B>(pub A, pub B);

impl<A, B> Chain<A, B> {
    pub fn chain<C>(self, c: C) -> Chain<Chain<A, B>, C> {
        Chain(self, c)
    }
}

impl<A: ToTokens, B: ToTokens> ToTokens for Chain<A, B> {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        A::to_tokens(&self.0, tokens);
        B::to_tokens(&self.1, tokens);
    }

    #[inline]
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let mut tokens = A::to_token_stream(&self.0);
        tokens.extend(B::to_token_stream(&self.1));

        tokens
    }

    #[inline]
    fn into_token_stream(self) -> proc_macro2::TokenStream
    where
        Self: Sized,
    {
        let mut tokens = A::into_token_stream(self.0);
        tokens.extend(B::into_token_stream(self.1));

        tokens
    }
}

macro_rules! chain {
    ($e1:expr, $e2:expr $(, $e:expr)* $(,)?) => {
        $crate::utils::chain::Chain($e1, $e2)
            $(.chain($e))*
    };
}

pub(crate) use chain;
