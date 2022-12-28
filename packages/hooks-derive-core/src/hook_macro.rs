use std::borrow::Cow;

use darling::FromMeta;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_quote_spanned, spanned::Spanned};

use crate::utils::{
    chain::Chain,
    either::Either,
    empty_or_trailing::AutoEmptyOrTrailing,
    group::{angled, parened},
    map::map_to_tokens,
    path_or_lit::PathOrLit,
    phantom::{make_phantom_or_ref, PhantomOfTy},
    repeat::Repeat,
    type_generics::TypeGenericsWithoutBraces,
};

pub type GenericParams = syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>;

#[cfg_attr(feature = "extra-traits", derive(PartialEq, Eq))]
#[derive(Debug, Default, FromMeta)]
#[non_exhaustive]
#[darling(default)]
pub struct HookArgs {
    /// Defaults to `::hooks::core`
    pub hooks_core_path: Option<PathOrLit<syn::Path>>,

    /// Defaults to tuple of all lifetime generics except `'hook`
    /// and all type generics.
    ///
    /// For example, default bounds of the following hook is
    /// `(&'a (), &'b (), PhantomData<T>)`
    ///
    /// ```
    /// # use std::marker::PhantomData;
    /// # use hooks::{hook, HookBounds};
    ///
    /// #[hook]
    /// fn use_my_hook<'a, 'b, T>() {
    /// }
    ///
    /// fn asserts<'a, 'b, T>() -> impl HookBounds<
    ///     Bounds = (&'a (), &'b (), PhantomData<T>)
    /// > {
    ///     use_my_hook()
    /// }
    ///
    /// # asserts::<()>();
    /// ```
    pub custom_bounds: Option<syn::Type>,

    /// Generic params used only in `Args`.
    /// Currently only lifetimes without bounds are supported.
    /// Defaults to no generics.
    pub args_generics: GenericParams,
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

    pub fn transform_item_fn_in_place(
        mut self,
        item_fn: &mut syn::ItemFn,
    ) -> Option<darling::Error> {
        let mut errors = darling::error::Accumulator::default();

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

        let sig = &mut item_fn.sig;

        let span_fn_name = sig.ident.span();

        // let token_add: syn::Token![+];
        // let lt_hook;

        let (hook_args_pat, mut hook_args_ty) = {
            let hook_args = std::mem::take(&mut sig.inputs);

            let paren_token = syn::token::Paren(span_fn_name);

            let (hook_args_pat, hook_args_ty) = hook_args
                .into_pairs()
                .into_iter()
                .map(|pair| {
                    let (arg, comma) = pair.into_tuple();
                    let comma = comma.unwrap_or_else(|| syn::Token![,](arg.span()));

                    let (pat, ty) = match arg {
                        syn::FnArg::Receiver(syn::Receiver {
                            attrs,
                            reference,
                            mutability,
                            self_token,
                        }) => {
                            // In fact, this branch is not valid
                            // because self cannot appear in closure args.
                            // But we still transform it and
                            // let the compiler complain about it.
                            let self_type = syn::Type::Path(syn::TypePath {
                                qself: None,
                                path: syn::Token![Self](self_token.span).into(),
                            });

                            if let Some((and_token, lifetime)) = reference {
                                let ty = syn::Type::Reference(syn::TypeReference {
                                    and_token,
                                    lifetime,
                                    mutability,
                                    elem: Box::new(self_type),
                                });
                                let pat = syn::Pat::Ident(syn::PatIdent {
                                    attrs,
                                    by_ref: None,
                                    mutability: None,
                                    ident: self_token.into(),
                                    subpat: None,
                                });
                                (pat, ty)
                            } else {
                                (
                                    syn::Pat::Ident(syn::PatIdent {
                                        attrs,
                                        by_ref: None,
                                        mutability,
                                        ident: self_token.into(),
                                        subpat: None,
                                    }),
                                    self_type,
                                )
                            }
                        }
                        syn::FnArg::Typed(pat_ty) => {
                            for attr in pat_ty.attrs {
                                errors.push(
                                    darling::Error::custom(
                                        "arguments of hook cannot have attributes",
                                    )
                                    .with_span(&attr),
                                );
                            }
                            (*pat_ty.pat, *pat_ty.ty)
                        }
                    };

                    (
                        syn::punctuated::Pair::Punctuated(pat, comma),
                        syn::punctuated::Pair::Punctuated(ty, comma),
                    )
                })
                .unzip();

            let hook_args_pat = syn::PatTuple {
                attrs: vec![],
                paren_token,
                elems: hook_args_pat,
            };

            let hook_args_ty = syn::TypeTuple {
                paren_token,
                elems: hook_args_ty,
            };

            (hook_args_pat, hook_args_ty)
        };

        crate::utils::elided_args_generics::auto_fill_lifetimes(
            &mut self.args_generics,
            &mut hook_args_ty.elems,
        );

        let args_lifetimes = &self.args_generics;

        let args_lifetimes_empty = args_lifetimes.is_empty();

        if !args_lifetimes_empty {
            for g in self.args_generics.iter() {
                match g {
                    syn::GenericParam::Lifetime(_) => {}
                    _ => errors.push(
                        darling::Error::custom(
                            "Currently args_generics only supports lifetimes without bounds",
                        )
                        .with_span(&g),
                    ),
                }
            }
        }

        let generics = &sig.generics;

        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let default_hook_bounds_fields_eot = map_to_tokens(&generics.params, |params| {
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

        let hook_bounds = self.custom_bounds.as_ref().map_or_else(
            || Either::A(parened(&default_hook_bounds_fields_eot)),
            |ty| Either::B(ty),
        );

        let mut output_ty: syn::Type = {
            let fn_rt = &mut sig.output;
            let (ra, output_ty) = match std::mem::replace(fn_rt, syn::ReturnType::Default) {
                syn::ReturnType::Default => {
                    let span = fn_rt.span();
                    (
                        syn::Token![->](span),
                        syn::Type::Tuple(syn::TypeTuple {
                            paren_token: syn::token::Paren(span),
                            elems: Default::default(),
                        }),
                    )
                }
                syn::ReturnType::Type(ra, ty) => (ra, *ty),
            };

            let (for_hook, for_lifetimes) = if args_lifetimes_empty {
                (None, None)
            } else {
                (
                    Some(
                        Chain(syn::Token![for](span_fn_name), syn::Token![<](span_fn_name))
                            .chain(args_lifetimes)
                            .chain(syn::Token![>](span_fn_name)),
                    ),
                    Some(Chain(syn::Token![,](span_fn_name), args_lifetimes)),
                )
            };

            let return_ty = parse_quote_spanned! { span_fn_name =>
                impl #for_hook #hooks_core_path ::Hook<#hook_args_ty>
                    + for<'hook #for_lifetimes> #hooks_core_path ::HookLifetime<
                        'hook,
                        #hook_args_ty,
                        &'hook #hook_bounds,
                        Value = #output_ty
                    >
                    + #hooks_core_path ::HookBounds<Bounds = #hook_bounds>
            };

            *fn_rt = syn::ReturnType::Type(ra, return_ty);

            output_ty
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

        // ( PhantomData<T>, PhantomData<HooksImplTrait0>, PhantomData<HooksImplTrait1>, )
        let hook_types_phantom;
        // <T: Clone, HooksImplTrait0: Debug, HooksImplTrait1: Any,>
        let hook_types_impl_generics;
        // <T, HooksImplTrait0, HooksImplTrait1,>
        let hook_types_type_generics;
        // _, _,
        let it_generics_elided_without_braces_eot;

        // where T: SomeOtherTrait, HooksImplTrait0: 'hook,
        // let hook_lifetime_where_clause;
        // TODO: figure out when hook_lifetime_where_clause is needed.

        if it_impl_generics_eot.is_empty() {
            hook_types_phantom = Either::A(&hook_bounds);
            hook_types_impl_generics = Either::A(impl_generics);
            hook_types_type_generics = Either::A(&type_generics);
            it_generics_elided_without_braces_eot = None;
            // hook_lifetime_where_clause = Either::A(where_clause);
        } else {
            hook_types_phantom = Either::B(parened(Chain(
                &default_hook_bounds_fields_eot,
                map_to_tokens(&it_impl_generics_eot, |v| {
                    v.iter()
                        .map(|pair| Chain(PhantomOfTy(&pair.0.ident), pair.1))
                }),
            )));

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

            // token_add = <syn::Token![+]>::default();

            // lt_hook = syn::Lifetime {
            //     apostrophe: Span::call_site(),
            //     ident: syn::Ident::new("hook", Span::call_site()),
            // };

            // let it_where_predicates_eot = map_to_tokens(&it_impl_generics_eot, |data| {
            //     data.iter()
            //         .map(|tp| chain![&tp.0, &token_add, &lt_hook, &tp.1,])
            // });

            // hook_lifetime_where_clause = Either::B({
            //     match where_clause {
            //         None => Chain(Default::default(), Either::A(it_where_predicates_eot)),
            //         Some(where_clause) => Chain(
            //             where_clause.where_token,
            //             Either::B(Chain(it_where_predicates_eot, &where_clause.predicates)),
            //         ),
            //     }
            // });
        };

        // T: Clone,
        // The generics comes from `fn`, so there won't be default types like `<T = i32>`
        let fn_impl_generics_without_braces_eot = AutoEmptyOrTrailing(&sig.generics.params);

        let mut used_hooks = vec![];
        let mut used_hooks_idents = vec![];

        let mut mutate_func_path = |e: ExprCallPath| {
            if expr_path_is_hook(e.func_path) {
                let paren_token = *e.paren_token;

                let idx = used_hooks_idents.len();
                let ident = syn::Ident::new(&format!("__hooks_hook_{}", idx), e.func_path.span());

                used_hooks_idents.push(ident.clone());

                // ::hooks::core::Hook::<_>::use_hook
                let path_use_hook: syn::ExprPath = syn::parse_quote! {
                    #hooks_core_path ::Hook::<_>::use_hook
                };
                let func_path = std::mem::replace(e.func_path, path_use_hook);

                let ident = syn::ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: ident.into(),
                };
                if e.args.is_empty() {
                    e.args.push(syn::Expr::Path(ident));
                    e.args.push(syn::Expr::Tuple(syn::ExprTuple {
                        attrs: vec![],
                        paren_token,
                        elems: Default::default(),
                    }));
                } else {
                    let mut hook_args = std::mem::take(e.args);

                    e.args.push(syn::Expr::Path(ident));

                    if !hook_args.trailing_punct() {
                        hook_args.push_punct(syn::Token![,](hook_args.span()));
                    }
                    let hook_args = syn::Expr::Tuple(syn::ExprTuple {
                        attrs: vec![],
                        paren_token,
                        elems: hook_args,
                    });
                    e.args.push(hook_args);
                }

                used_hooks.push(syn::ExprCall {
                    attrs: vec![],
                    func: Box::new(syn::Expr::Path(func_path)),
                    paren_token,
                    args: Default::default(),
                });
            }
        };

        let mut impl_use_hook = std::mem::take(&mut item_fn.block.stmts);
        for stmt in impl_use_hook.iter_mut() {
            let expr = match stmt {
                syn::Stmt::Local(local) => {
                    if let Some((_, expr)) = &mut local.init {
                        &mut *expr
                    } else {
                        continue;
                    }
                }
                syn::Stmt::Item(_) => {
                    // Items are untouched
                    continue;
                }
                syn::Stmt::Expr(expr) => expr,
                syn::Stmt::Semi(expr, _) => expr,
            };

            tlpc(expr, &mut mutate_func_path);
        }

        let impl_poll_next_update = if used_hooks.is_empty() {
            quote_spanned! { span_fn_name =>
                #hooks_core_path ::fn_hook::poll_next_update_ready_false
            }
        } else {
            quote_spanned! { span_fn_name =>
                #hooks_core_path ::HookPollNextUpdate::poll_next_update
            }
        };

        let (expr_hooks_data, arg_hooks_data, impl_extract_hooks_data) = match used_hooks.len() {
            0 => (quote! {()}, quote! {_: ::core::pin::Pin<&mut ()>}, None),
            1 => (
                used_hooks.pop().unwrap().into_token_stream(),
                used_hooks_idents.pop().unwrap().into_token_stream(),
                None,
            ),
            _ => {
                let expr_hooks_data = {
                    let mut first2 = used_hooks.drain(0..=1);
                    let first = first2.next().unwrap();
                    let second = first2.next().unwrap();
                    drop(first2);

                    let used_hooks = used_hooks.into_iter();

                    quote_spanned! { span_fn_name =>
                        #hooks_core_path ::hook_pair::HookPair::new(#first , #second)
                            #( .chain( #used_hooks ) )*
                    }
                };

                let ident_hooks_data = syn::Ident::new("__hooks_data", span_fn_name);

                let impl_extract_hooks_data = {
                    let mut stmts = Vec::with_capacity(used_hooks_idents.len());

                    while let Some(used_hook_ident) = used_hooks_idents.pop() {
                        let stmt = if !used_hooks_idents.is_empty() {
                            quote_spanned! { span_fn_name =>
                                let (#ident_hooks_data, #used_hook_ident) = #ident_hooks_data.pin_project();
                            }
                        } else {
                            // This is the first element
                            quote_spanned! { span_fn_name =>
                                let #used_hook_ident = #ident_hooks_data;
                            }
                        };
                        stmts.push(stmt);
                    }

                    proc_macro2::TokenStream::from_iter(stmts)
                };

                (
                    expr_hooks_data,
                    ident_hooks_data.into_token_stream(),
                    Some(impl_extract_hooks_data),
                )
            }
        };

        let (args_generics_for_hook_lifetime_eot, stmt_ret) = if args_lifetimes_empty {
            let stmt_ret: syn::Expr = parse_quote_spanned! { span_fn_name =>
                #hooks_core_path ::fn_hook::new_fn_hook::<
                    #hook_args_ty,
                    _,
                    __HookTypes <#fn_type_generics_eot  #it_generics_elided_without_braces_eot>
                >(
                    #expr_hooks_data,
                    #impl_poll_next_update,
                    |#arg_hooks_data, #hook_args_pat : #hook_args_ty| {
                        #impl_extract_hooks_data

                        #(#impl_use_hook)*
                    }
                )
            };

            (None, stmt_ret)
        } else {
            let stmt_ret: syn::Expr = parse_quote_spanned! { span_fn_name =>
                {
                    #[inline]
                    fn _hooks_def_fn_hook<
                        #fn_impl_generics_without_braces_eot
                        #(#it_impl_generics_eot)*
                        __HooksData,
                        __HooksPoll: ::core::ops::Fn(::core::pin::Pin<&mut __HooksData>, &mut ::core::task::Context) -> ::core::task::Poll<::core::primitive::bool>,
                        __HooksUseHook: for<'hook, #args_lifetimes> ::core::ops::Fn(::core::pin::Pin<&'hook mut __HooksData>, #hook_args_ty) -> #output_ty,
                    >(
                        hooks_data: __HooksData,
                        hooks_poll: __HooksPoll,
                        hooks_use_hook: __HooksUseHook
                    ) -> #hooks_core_path ::fn_hook::FnHook::<__HooksData, __HooksPoll, __HooksUseHook, __HookTypes #hook_types_type_generics> #where_clause {
                        #hooks_core_path ::fn_hook::FnHook::<__HooksData, __HooksPoll, __HooksUseHook, __HookTypes #hook_types_type_generics>::new(
                            hooks_data,
                            hooks_poll,
                            hooks_use_hook
                        )
                    }

                    _hooks_def_fn_hook::<
                        #fn_type_generics_eot
                        #it_generics_elided_without_braces_eot
                        _, _, _
                    >(
                        #expr_hooks_data,
                        #impl_poll_next_update,
                        |#arg_hooks_data, #hook_args_pat| {
                            #impl_extract_hooks_data

                            #(#impl_use_hook)*
                        },
                    )
                }
            };

            (Some(AutoEmptyOrTrailing(self.args_generics)), stmt_ret)
        };

        item_fn.block.stmts = parse_quote_spanned! { span_fn_name =>
            struct __HookTypes #hook_types_impl_generics #where_clause {
                __: ::core::marker::PhantomData< #hook_types_phantom >
            }

            impl #hook_types_impl_generics #hooks_core_path ::HookBounds for __HookTypes #hook_types_type_generics #where_clause {
                type Bounds = #hook_bounds;
            }

            impl <
                'hook,
                #args_generics_for_hook_lifetime_eot
                #fn_impl_generics_without_braces_eot
                #(#it_impl_generics_eot)*
            > #hooks_core_path ::HookLifetime<'hook, #hook_args_ty, &'hook #hook_bounds>
                for __HookTypes #hook_types_type_generics #where_clause
            {
                type Value = #output_ty;
            }
        };

        item_fn.block.stmts.push(syn::Stmt::Expr(stmt_ret));

        errors.finish().err()
    }

    pub fn from_punctuated_meta_list(
        meta_list: syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]>,
    ) -> darling::Result<Self> {
        let args: Vec<syn::NestedMeta> = meta_list.into_iter().collect();
        Self::from_list(&args)
    }

    pub fn with_args_generics(mut self, args_generics: GenericParams) -> Self {
        self.args_generics = args_generics;
        self
    }
}

struct ExprCallPath<'a> {
    func_path: &'a mut syn::ExprPath,
    paren_token: &'a mut syn::token::Paren,
    args: &'a mut syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
}

/// Mutate Top-Level-Path-Calls
fn tlpc(expr: &mut syn::Expr, mutate_func_path: &mut impl FnMut(ExprCallPath)) {
    match expr {
        syn::Expr::Array(array) => {
            for elem in array.elems.iter_mut() {
                tlpc(elem, mutate_func_path);
            }
        }
        syn::Expr::Assign(a) => tlpc(&mut a.right, mutate_func_path),
        syn::Expr::AssignOp(a) => tlpc(&mut a.right, mutate_func_path),
        syn::Expr::Async(_) => {
            // `async {}` is untouched
        }
        syn::Expr::Await(_) => {
            // `fut.await` is untouched
            // because hook closure is not async and
            // there cannot be any await exprs in top level.
        }
        syn::Expr::Binary(b) => {
            tlpc(&mut b.left, mutate_func_path);
            tlpc(&mut b.right, mutate_func_path);
        }
        syn::Expr::Block(_) => {
            // `{}` is untouched because it is not top level
        }
        syn::Expr::Box(b) => tlpc(&mut b.expr, mutate_func_path),
        syn::Expr::Break(_) => {
            // `break` is untouched
            // because there cannot be any break in top level.
        }
        syn::Expr::Call(c) => {
            for arg in c.args.iter_mut() {
                tlpc(arg, mutate_func_path);
            }

            if let syn::Expr::Path(func_path) = &mut *c.func {
                mutate_func_path(ExprCallPath {
                    func_path,
                    paren_token: &mut c.paren_token,
                    args: &mut c.args,
                });
            } else {
                tlpc(&mut c.func, mutate_func_path);
            }
        }
        syn::Expr::Cast(c) => tlpc(&mut c.expr, mutate_func_path),
        syn::Expr::Closure(_) => {
            // `|| {}` is untouched
            // because exprs in the body are not top level
        }
        syn::Expr::Continue(_) => {
            // `continue` is untouched
            // with the same reason as `break`
        }
        syn::Expr::Field(f) => tlpc(&mut f.base, mutate_func_path),
        syn::Expr::ForLoop(f) => tlpc(&mut f.expr, mutate_func_path),
        syn::Expr::Group(g) => tlpc(&mut g.expr, mutate_func_path),
        syn::Expr::If(i) => tlpc(&mut i.cond, mutate_func_path),
        syn::Expr::Index(i) => {
            tlpc(&mut i.expr, mutate_func_path);
            tlpc(&mut i.index, mutate_func_path);
        }
        syn::Expr::Let(l) => tlpc(&mut l.expr, mutate_func_path),
        syn::Expr::Lit(_) => {
            // literals are untouched
            // because there is no function call
        }
        syn::Expr::Loop(_) => {
            // `loop {}` is untouched
            // because there are no exprs in top level
        }
        syn::Expr::Macro(_) => {
            // macros are untouched
        }
        syn::Expr::Match(m) => tlpc(&mut m.expr, mutate_func_path),
        syn::Expr::MethodCall(m) => {
            for arg in m.args.iter_mut() {
                tlpc(arg, mutate_func_path);
            }
            tlpc(&mut m.receiver, mutate_func_path);
        }
        syn::Expr::Paren(p) => tlpc(&mut p.expr, mutate_func_path),
        syn::Expr::Path(_) => {
            // `std::mem::replace` is untouched
            // because there is no function call
        }
        syn::Expr::Range(r) => {
            if let Some(e) = &mut r.from {
                tlpc(e, mutate_func_path);
            }
            if let Some(e) = &mut r.to {
                tlpc(e, mutate_func_path);
            }
        }
        syn::Expr::Reference(r) => {
            tlpc(&mut r.expr, mutate_func_path);
        }
        syn::Expr::Repeat(_) => {
            // `[expr; N]` is untouched
            // because the expr is not considered top level
        }
        syn::Expr::Return(r) => {
            if let Some(e) = &mut r.expr {
                tlpc(e, mutate_func_path);
            }
        }
        syn::Expr::Struct(s) => {
            for field in s.fields.iter_mut() {
                tlpc(&mut field.expr, mutate_func_path);
            }
            if let Some(e) = &mut s.rest {
                tlpc(e, mutate_func_path);
            }
        }
        syn::Expr::Try(t) => tlpc(&mut t.expr, mutate_func_path),
        syn::Expr::TryBlock(_) => {
            // `try {}` is untouched
            // because there are no exprs in top level
        }
        syn::Expr::Tuple(t) => {
            for elem in t.elems.iter_mut() {
                tlpc(elem, mutate_func_path);
            }
        }
        syn::Expr::Type(t) => tlpc(&mut t.expr, mutate_func_path),
        syn::Expr::Unary(u) => tlpc(&mut u.expr, mutate_func_path),
        syn::Expr::Unsafe(_) => {
            // `unsafe {}` is untouched
            // because there are no exprs in top level
        }
        syn::Expr::Verbatim(_) => {
            // untouched because not interpreted by Syn
        }
        syn::Expr::While(w) => tlpc(&mut w.cond, mutate_func_path),
        syn::Expr::Yield(_) => {
            // `yield` is untouched
            // with the same reason as `break`
        }
        _ => {
            // unknown exprs are untouched
            // Adding new variants or changing behavior of current variants
            // would be a BREAKING CHANGE
        }
    }
}

fn expr_path_is_hook(path: &syn::ExprPath) -> bool {
    if let Some(last) = path.path.segments.last() {
        last.ident.to_string().starts_with("use_")
    } else {
        false
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
