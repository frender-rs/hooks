use syn::punctuated::Punctuated;

#[inline]
pub fn process_top_level_ref(ty: &mut syn::Type, mut f: impl FnMut(&mut syn::TypeReference)) {
    match ty {
        syn::Type::Group(g) => process_top_level_ref(&mut g.elem, f),
        syn::Type::Reference(tp) => f(tp),
        _ => {}
    }
}

/// auto fill elided lifetimes.
/// Currently only top-level references are auto filled.
///
/// For example: `&T` -> `&'hooks_auto_lifetime_0 T`
pub fn auto_fill_lifetimes(
    args_lifetimes: &mut Punctuated<syn::GenericParam, syn::Token![,]>,
    args_types: &mut Punctuated<syn::Type, syn::Token![,]>,
) {
    let mut index = 0u8;

    for ty in args_types.iter_mut() {
        process_top_level_ref(ty, |tr| {
            if tr.lifetime.is_none() {
                let lt = format!("'hooks_auto_lifetime_{index}");
                index += 1;

                let lt = syn::Lifetime::new(&lt, tr.and_token.span);
                tr.lifetime = Some(lt.clone());
                args_lifetimes.push(syn::GenericParam::Lifetime(syn::LifetimeDef::new(lt)));
            }
        })
    }
}
