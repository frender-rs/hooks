pub struct ExprCallPath<'a> {
    pub func_path: &'a mut syn::ExprPath,
    pub paren_token: &'a mut syn::token::Paren,
    pub args: &'a mut syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
}

/// Mutate Top-Level-Path-Calls
pub fn tlpc(expr: &mut syn::Expr, mutate_func_path: &mut impl FnMut(ExprCallPath)) {
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
