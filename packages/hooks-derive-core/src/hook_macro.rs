use std::borrow::Cow;

use darling::{FromMeta, ToTokens};
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{parse_quote_spanned, spanned::Spanned};

use crate::{
    detect_hooks, detected_hooks_to_tokens,
    utils::{
        chain::Chain,
        either::Either,
        empty_or_trailing::AutoEmptyOrTrailing,
        group::{angled, parened},
        map::map_to_tokens,
        path_or_lit::PathOrLit,
        phantom::{make_phantom_or_ref, PhantomOfTy},
        repeat::Repeat,
        type_generics::TypeGenericsWithoutBraces,
    },
    DetectedHooksTokens,
};

#[cfg_attr(feature = "extra-traits", derive(PartialEq, Eq))]
#[derive(Debug, Default, FromMeta)]
#[non_exhaustive]
#[darling(default)]
pub struct HookArgs {
    /// Defaults to `::hooks::core`
    pub hooks_core_path: Option<PathOrLit<syn::Path>>,

    /// When a hook fn borrows from a lifetime,
    /// this bound might need to be explicitly specified.
    ///
    /// ```compile_fail
    /// # extern crate hooks_dev as hooks;
    /// # use hooks::prelude::*;
    ///
    /// #[hook]
    /// fn use_borrow<'a>(v: &'a str) -> usize {
    ///     v.len()
    /// }
    /// ```
    ///
    /// ```
    /// # extern crate hooks_dev as hooks;
    /// # use hooks::prelude::*;
    /// #[hook(bounds = "'a")]
    /// fn use_borrow<'a>(v: &'a str) -> usize {
    ///     v.len()
    /// }
    /// ```
    ///
    /// This is equivalent to `type Bounds = impl ...` in [`hook_fn!(...);`](hooks_dev::hook_fn);
    ///
    /// ```
    /// hook_fn!(
    ///     type Bounds = impl 'a;
    ///     fn use_borrow<'a>(v: &'a str) -> usize {
    ///         v.len()
    ///     }
    /// );
    /// ```
    pub bounds: Option<syn::punctuated::Punctuated<syn::TypeParamBound, syn::Token![+]>>,
}

impl HookArgs {
    #[inline]
    pub fn transform_item_fn(
        self,
        mut item_fn: syn::ItemFn,
    ) -> (syn::ItemFn, Option<darling::Error>) {
        let error = self.transform_item_fn_in_place(&mut item_fn);
        (item_fn, error)
    }

    pub fn transform_item_fn_in_place(self, item_fn: &mut syn::ItemFn) -> Option<darling::Error> {
        // let mut errors = darling::error::Accumulator::default();

        let hooks_core_path = self.hooks_core_path.map_or_else(
            || syn::Path {
                leading_colon: Some(Default::default()),
                segments: syn::punctuated::Punctuated::from_iter([
                    syn::PathSegment::from(syn::Ident::new("hooks", Span::call_site())),
                    syn::PathSegment::from(syn::Ident::new("core", Span::call_site())),
                ]),
            },
            PathOrLit::unwrap,
        );

        let bounds = self.bounds;

        let sig = &mut item_fn.sig;

        let span_fn_name = sig.ident.span();

        let generics = &sig.generics;

        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let hooks_value_struct_field_ty = map_to_tokens(&generics.params, |params| {
            params.pairs().filter_map(|p| {
                make_phantom_or_ref(p.value()).map(|v| {
                    Chain(
                        v,
                        p.punct()
                            .map_or_else(|| Cow::Owned(Default::default()), |v| Cow::Borrowed(*v)),
                    )
                })
            })
        });

        let mut output_ty: syn::Type = {
            let fn_rt = &mut sig.output;
            let span;
            match fn_rt {
                syn::ReturnType::Default => {
                    span = span_fn_name;
                    let output_ty = syn::Type::Tuple(syn::TypeTuple {
                        paren_token: syn::token::Paren(span),
                        elems: Default::default(),
                    });
                    *fn_rt = syn::ReturnType::Type(
                        syn::Token![->](span),
                        Box::new(syn::Type::Verbatim(utils::UpdateHookUninitialized(
                            &hooks_core_path,
                            span,
                            quote_spanned!(span=> ()),
                            bounds,
                        ))),
                    );

                    output_ty
                }
                syn::ReturnType::Type(ra, ty) => {
                    span = ra.span();
                    let it = utils::UpdateHookUninitialized(&hooks_core_path, span, &**ty, bounds);
                    std::mem::replace(&mut **ty, syn::Type::Verbatim(it))
                }
            }
        };

        // T,
        let fn_type_generics_eot = AutoEmptyOrTrailing(TypeGenericsWithoutBraces(&generics.params));

        // HooksImplTrait0: Debug, HooksImplTrait1: Any,
        //      introduced by impl trait in return position
        let it_impl_generics_eot = extract_impl_trait_as_type_params(&mut output_ty);

        // HooksImplTrait0, HooksImplTrait1,
        let it_type_generics_eot = map_to_tokens(&it_impl_generics_eot, |v| {
            v.iter().map(|pair| Chain(&pair.0.ident, &pair.1))
        });

        // PhantomData<T>, PhantomData<HooksImplTrait0>, PhantomData<HooksImplTrait1>,
        let hook_types_phantoms_eot;
        // <T: Clone, HooksImplTrait0: Debug, HooksImplTrait1: Any,>
        let hook_types_impl_generics;
        // <T, HooksImplTrait0, HooksImplTrait1,>
        let hook_types_type_generics;
        // _, _,
        let it_generics_elided_without_braces_eot;

        if it_impl_generics_eot.is_empty() {
            hook_types_phantoms_eot = Either::A(&hooks_value_struct_field_ty);
            hook_types_impl_generics = Either::A(impl_generics);
            hook_types_type_generics = Either::A(&type_generics);
            it_generics_elided_without_braces_eot = None;
        } else {
            hook_types_phantoms_eot = Either::B(Chain(
                &hooks_value_struct_field_ty,
                map_to_tokens(&it_impl_generics_eot, |v| {
                    v.iter()
                        .map(|pair| Chain(PhantomOfTy(&pair.0.ident), pair.1))
                }),
            ));

            hook_types_impl_generics = Either::B(angled(Chain(
                AutoEmptyOrTrailing(&sig.generics.params),
                map_to_tokens(&it_impl_generics_eot, |v| v.iter()),
            )));

            hook_types_type_generics =
                Either::B(angled(Chain(&fn_type_generics_eot, &it_type_generics_eot)));

            it_generics_elided_without_braces_eot = Some(Repeat(
                Chain(<syn::Token![_]>::default(), <syn::Token![,]>::default()),
                it_impl_generics_eot.len(),
            ));
        };

        // T: Clone,
        // The generics comes from `fn`, so there won't be default types like `<T = i32>`
        let fn_impl_generics_without_braces_eot = AutoEmptyOrTrailing(&sig.generics.params);

        let mut impl_use_hook = std::mem::take(&mut item_fn.block.stmts);

        let used_hooks = detect_hooks(impl_use_hook.iter_mut(), &hooks_core_path);

        let DetectedHooksTokens {
            fn_arg_data_pat: arg_hooks_data,
            fn_stmts_extract_data: impl_extract_hooks_data,
        } = detected_hooks_to_tokens(used_hooks.hooks, &hooks_core_path, sig.fn_token.span);

        item_fn
            .block
            .stmts
            .push(syn::Stmt::Expr(syn::Expr::Verbatim(
                quote_spanned! { span_fn_name =>
                    enum __HooksImplNever {}

                    struct __HooksValueOfThisHook #hook_types_impl_generics
                    #where_clause
                    {
                        __: (
                            __HooksImplNever,
                            #hook_types_phantoms_eot
                        )
                    }

                    impl<
                        'hook,
                        #fn_impl_generics_without_braces_eot
                        #(#it_impl_generics_eot)*
                    > #hooks_core_path::HookValue<'hook> for
                        __HooksValueOfThisHook #hook_types_type_generics
                        #where_clause {
                        type Value = #output_ty;
                    }

                    #hooks_core_path::fn_hook::use_fn_hook::<
                        __HooksValueOfThisHook<
                            #fn_type_generics_eot
                            #it_generics_elided_without_braces_eot
                        >, _, _
                    >
                    (
                        move |#arg_hooks_data| {
                            #impl_extract_hooks_data

                            #(#impl_use_hook)*
                        }
                    )
                },
            )));

        // errors.finish().err()
        None
    }

    pub fn from_punctuated_meta_list(
        meta_list: syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]>,
    ) -> darling::Result<Self> {
        let args: Vec<syn::NestedMeta> = meta_list.into_iter().collect();
        Self::from_list(&args)
    }
}

fn replace_impl_trait_in_type(
    ty: &mut syn::Type,
    f: &mut impl FnMut(&mut syn::TypeImplTrait) -> syn::Type,
) {
    match ty {
        syn::Type::Array(ta) => replace_impl_trait_in_type(&mut ta.elem, f),
        syn::Type::BareFn(_) => {}
        syn::Type::Group(g) => replace_impl_trait_in_type(&mut g.elem, f),
        syn::Type::ImplTrait(it) => {
            // TODO: resolve `impl Trait` in it.bounds
            // f(it.bounds)

            *ty = f(it)
        }
        syn::Type::Infer(_) => {}
        syn::Type::Macro(_) => {}
        syn::Type::Never(_) => {}
        syn::Type::Paren(p) => {
            let is_impl_trait = matches!(&*p.elem, syn::Type::ImplTrait(_));
            replace_impl_trait_in_type(&mut p.elem, f);

            // also remove the paren for (HookImplTrait0)
            if is_impl_trait {
                let new_ty =
                    std::mem::replace(&mut *p.elem, syn::Type::Verbatim(Default::default()));
                *ty = new_ty;
            }
        }
        syn::Type::Path(tp) => {
            if let Some(qself) = &mut tp.qself {
                replace_impl_trait_in_type(&mut qself.ty, f);
            }
            for seg in tp.path.segments.iter_mut() {
                match &mut seg.arguments {
                    syn::PathArguments::None => {}
                    syn::PathArguments::AngleBracketed(a) => {
                        for arg in a.args.iter_mut() {
                            match arg {
                                syn::GenericArgument::Lifetime(_) => {}
                                syn::GenericArgument::Type(ty) => {
                                    replace_impl_trait_in_type(ty, f);
                                }
                                syn::GenericArgument::Const(_) => {}
                                syn::GenericArgument::Binding(b) => {
                                    replace_impl_trait_in_type(&mut b.ty, f);
                                }
                                syn::GenericArgument::Constraint(_) => {}
                            }
                        }
                    }
                    syn::PathArguments::Parenthesized(_) => {
                        // TODO: resolve `impl Trait` in path like `Fn(impl Trait) -> impl Trait`
                    }
                }
            }
            // TODO: resolve `impl Trait` in path like `Struct<impl Trait>`
        }
        syn::Type::Ptr(ptr) => replace_impl_trait_in_type(&mut ptr.elem, f),
        syn::Type::Reference(r) => replace_impl_trait_in_type(&mut r.elem, f),
        syn::Type::Slice(s) => replace_impl_trait_in_type(&mut s.elem, f),
        syn::Type::TraitObject(_) => {
            // TODO: resolve `impl Trait` in to.bounds
            // f(to.bounds)
        }
        syn::Type::Tuple(t) => {
            for elem in t.elems.iter_mut() {
                replace_impl_trait_in_type(elem, f);
            }
        }
        syn::Type::Verbatim(_) => {}
        _ => {}
    }
}

/// The returned Punctuated is guaranteed to be `empty_or_trailing`
fn extract_impl_trait_as_type_params(
    output_ty: &mut syn::Type,
) -> Vec<Chain<syn::TypeParam, syn::Token![,]>> {
    let mut ret = vec![];
    replace_impl_trait_in_type(output_ty, &mut |ty| {
        let id = ret.len();
        let span = ty.impl_token.span;

        let ident = syn::Ident::new(&format!("HooksImplTrait{id}"), span);

        ret.push(Chain(
            syn::TypeParam {
                attrs: vec![],
                ident: ident.clone(),
                colon_token: Some(syn::Token![:](span)),
                bounds: std::mem::take(&mut ty.bounds),
                eq_token: None,
                default: None,
            },
            syn::Token![,](span),
        ));

        syn::Type::Path(syn::TypePath {
            qself: None,
            path: ident.into(),
        })
    });
    ret
}

mod utils {
    use darling::ToTokens;
    use proc_macro2::{Span, TokenStream};
    use quote::quote_spanned;
    use syn::spanned::Spanned;

    use crate::utils::chain::Chain;

    #[allow(non_snake_case)]
    pub fn UpdateHookUninitialized(
        hooks_core_path: &impl ToTokens,
        span: Span,
        value_ty: impl ToTokens,
        bounds: Option<impl ToTokens>,
    ) -> TokenStream {
        let bounds = bounds.map(|bounds| {
            let bounds = bounds.into_token_stream();

            Chain(syn::Token![+](bounds.span()), bounds)
        });

        quote_spanned! {span=>
            impl #hooks_core_path::UpdateHookUninitialized<
                Hook = impl #hooks_core_path::Hook + for<'hook> #hooks_core_path::HookValue<'hook, Value = #value_ty>
                #bounds
            > #bounds
        }
    }
}
