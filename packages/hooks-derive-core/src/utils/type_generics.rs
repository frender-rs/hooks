use quote::ToTokens;

/// [`syn::TypeGenerics`] without `< >`
pub struct TypeGenericsWithoutBraces<'a>(
    pub &'a syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>,
);

impl<'a> ToTokens for TypeGenericsWithoutBraces<'a> {
    /// See [`syn::TypeGenerics::to_tokens`] for the original code.
    ///
    /// [`syn::TypeGenerics::to_tokens`]: https://docs.rs/syn/1.0.103/src/syn/generics.rs.html#1126-1170
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.0.is_empty() {
            return;
        }

        // Print lifetimes before types and consts, regardless of their
        // order in self.params.
        //
        // TODO: ordering rules for const parameters vs type parameters have
        // not been settled yet. https://github.com/rust-lang/rust/issues/44580
        let mut trailing_or_empty = true;
        for param in self.0.pairs() {
            if let syn::GenericParam::Lifetime(def) = *param.value() {
                // Leave off the lifetime bounds and attributes
                def.lifetime.to_tokens(tokens);
                param.punct().to_tokens(tokens);
                trailing_or_empty = param.punct().is_some();
            }
        }
        for param in self.0.pairs() {
            if let syn::GenericParam::Lifetime(_) = **param.value() {
                continue;
            }
            if !trailing_or_empty {
                <syn::Token![,]>::default().to_tokens(tokens);
                trailing_or_empty = true;
            }
            match *param.value() {
                syn::GenericParam::Lifetime(_) => unreachable!(),
                syn::GenericParam::Type(param) => {
                    // Leave off the type parameter defaults
                    param.ident.to_tokens(tokens);
                }
                syn::GenericParam::Const(param) => {
                    // Leave off the const parameter defaults
                    param.ident.to_tokens(tokens);
                }
            }
            param.punct().to_tokens(tokens);
        }
    }
}

impl<'a> super::empty_or_trailing::IsEmptyOrTrailing for TypeGenericsWithoutBraces<'a> {
    type Punct = syn::Token![,];

    #[inline]
    fn is_empty_or_trailing(&self) -> bool {
        self.0.empty_or_trailing()
    }
}
