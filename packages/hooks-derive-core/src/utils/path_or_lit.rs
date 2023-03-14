use darling::{FromMeta, ToTokens};

#[derive(Debug, PartialEq, Eq)]
pub enum PathOrLit<V> {
    Path(V),
    Lit(V),
}

impl<V> PathOrLit<V> {
    #[inline]
    pub fn as_inner(&self) -> &V {
        match self {
            PathOrLit::Path(v) => v,
            PathOrLit::Lit(v) => v,
        }
    }

    #[inline]
    pub fn unwrap(self) -> V {
        match self {
            PathOrLit::Path(v) => v,
            PathOrLit::Lit(v) => v,
        }
    }
}

impl<V: FromMeta + From<syn::Path>> FromMeta for PathOrLit<V> {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        if items.is_empty() {
            return Err(darling::Error::too_few_items(1));
        } else if items.len() > 1 {
            return Err(darling::Error::too_many_items(1));
        }
        let nmi = &items[0];

        match nmi {
            syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                Ok(PathOrLit::Path(path.clone().into()))
            }
            syn::NestedMeta::Lit(value) => Self::from_value(value),
            _ => Err(darling::Error::unexpected_type("non-word").with_span(nmi)),
        }
    }

    #[inline]
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        V::from_value(value).map(Self::Lit)
    }
}

impl<V: ToTokens> ToTokens for PathOrLit<V> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.as_inner().to_tokens(tokens)
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.as_inner().to_token_stream()
    }

    fn into_token_stream(self) -> proc_macro2::TokenStream
    where
        Self: Sized,
    {
        self.unwrap().into_token_stream()
    }
}
