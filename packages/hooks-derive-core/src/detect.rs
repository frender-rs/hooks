use quote::ToTokens;
use syn::spanned::Spanned;

use crate::{
    tlpc::{tlpc, ExprCallPath},
    DetectedHookCall,
};

pub fn expr_path_is_hook(path: &syn::ExprPath) -> bool {
    if let Some(last) = path.path.segments.last() {
        last.ident.to_string().starts_with("use_")
    } else {
        false
    }
}

pub fn mut_expr_of_statement(stmt: &mut syn::Stmt) -> Option<&mut syn::Expr> {
    match stmt {
        syn::Stmt::Local(local) => {
            if let Some((_, expr)) = &mut local.init {
                Some(&mut *expr)
            } else {
                None
            }
        }
        syn::Stmt::Item(_) => {
            // Items are untouched
            None
        }
        syn::Stmt::Expr(expr) => Some(expr),
        syn::Stmt::Semi(expr, _) => Some(expr),
    }
}

pub fn transform_path_call(
    e: ExprCallPath,
    hooks_core_path: impl ToTokens,
    gen_ident: impl FnOnce(&syn::ExprPath) -> syn::Ident,
) -> syn::ExprCall {
    let paren_token = *e.paren_token;

    // ::hooks::core::Hook::<_>::use_hook
    let path_use_hook: syn::ExprPath = syn::parse_quote! {
        #hooks_core_path ::Hook::<_>::use_hook
    };
    let func_path = std::mem::replace(e.func_path, path_use_hook);

    let ident = gen_ident(&func_path);

    let ident = syn::ExprPath {
        attrs: vec![],
        qself: None,
        path: ident.into(),
    };

    let hook_args = std::mem::take(e.args);
    // Hook::<_>::use_hook(ident, (args,))
    e.args.extend([
        syn::Expr::Path(ident),
        syn::Expr::Tuple(syn::ExprTuple {
            attrs: vec![],
            paren_token,
            elems: hook_args,
        }),
    ]);

    syn::ExprCall {
        attrs: vec![],
        func: Box::new(syn::Expr::Path(func_path)),
        paren_token,
        args: Default::default(),
    }
}

pub fn detect_hooks<'a>(
    stmts: impl Iterator<Item = &'a mut syn::Stmt>,
    hooks_core_path: &impl ToTokens,
) -> Vec<crate::DetectedHookCall> {
    let mut used_hooks = vec![];

    let mut mutate_func_path = |e: ExprCallPath| {
        if expr_path_is_hook(e.func_path) {
            let mut hook_ident = None;
            let expr_call = transform_path_call(e, hooks_core_path, |func_path| {
                let idx = used_hooks.len();
                let ident = syn::Ident::new(&format!("__hooks_hook_{idx}"), func_path.span());
                hook_ident = Some(ident.clone());
                ident
            });
            let ident = hook_ident.unwrap();
            used_hooks.push(DetectedHookCall { ident, expr_call })
        }
    };

    for stmt in stmts {
        if let Some(expr) = mut_expr_of_statement(stmt) {
            tlpc(expr, &mut mutate_func_path);
        }
    }

    used_hooks
}
