use proc_macro2::{Span, TokenStream};
use syn::{ext::IdentExt, parse::Parse, spanned::Spanned};

pub mod is_hook {

    pub fn ident(ident: &syn::Ident) -> bool {
        ident.to_string().starts_with("use_")
    }

    pub fn expr_path(path: &syn::ExprPath) -> bool {
        if let Some(last) = path.path.segments.last() {
            ident(&last.ident)
        } else {
            false
        }
    }

    pub fn expr_call_with_path(path: &syn::ExprPath) -> bool {
        expr_path(path)
    }

    pub fn expr_method_call(expr: &syn::ExprMethodCall) -> bool {
        ident(&expr.method)
    }

    pub fn expr_macro(expr: &syn::ExprMacro) -> bool {
        expr.mac
            .path
            .get_ident()
            .map_or(false, |ident| ident == "h")
    }
}

/// attr is `#[not_hook]` or `#![not_hook]`
pub fn attr_is_not_hook(attr: &syn::Attribute) -> bool {
    attr.tokens.is_empty()
        && attr
            .path
            .get_ident()
            .map_or(false, |ident| ident == "not_hook")
}

struct ExprOfStmtMut<'a> {
    expr: &'a mut syn::Expr,
    stmt_attrs: Option<&'a mut Vec<syn::Attribute>>,
}

impl<'a> ExprOfStmtMut<'a> {
    fn try_from(stmt: &'a mut syn::Stmt) -> Option<Self> {
        match stmt {
            syn::Stmt::Local(local) => {
                if let Some((_, expr)) = &mut local.init {
                    Some(Self {
                        expr,
                        stmt_attrs: Some(&mut local.attrs),
                    })
                } else {
                    None
                }
            }
            syn::Stmt::Item(_) => {
                // Items are untouched
                None
            }
            syn::Stmt::Expr(expr) => Some(Self {
                expr,
                stmt_attrs: None,
            }),
            syn::Stmt::Semi(expr, _) => Some(Self {
                expr,
                stmt_attrs: None,
            }),
        }
    }
}

pub struct DetectedHooks {
    pub hooks: Vec<crate::DetectedHook>,
    pub not_hook_attrs: Vec<syn::Attribute>,
}

pub fn detect_hooks<'a>(
    stmts: impl Iterator<Item = &'a mut syn::Stmt>,
    hooks_core_path: &syn::Path,
) -> DetectedHooks {
    let mut used_hooks = vec![];

    let mut mutate = MutateHookExpr::new(|expr| {
        let mut expr_attrs = vec![];
        let mut h_ident = None;
        let mut paren_token = None;
        let mut hook_id = None;

        if let syn::Expr::Macro(m) = expr {
            expr_attrs = std::mem::take(&mut m.attrs);

            h_ident = Some(m.mac.path.get_ident().unwrap().clone());

            paren_token = Some(match &mut m.mac.delimiter {
                syn::MacroDelimiter::Paren(p) => *p,
                syn::MacroDelimiter::Brace(d) => syn::token::Paren(d.span),
                syn::MacroDelimiter::Bracket(d) => syn::token::Paren(d.span),
            });

            let HMacroContent {
                explicit_hook_id,
                expr: actual_expr,
            } = syn::parse2(std::mem::take(&mut m.mac.tokens)).unwrap();

            hook_id = explicit_hook_id.map(|h| h.0);

            *expr = syn::Expr::Verbatim(actual_expr);
        }

        let span = Span::call_site().located_at(expr.span());

        let actual_expr = std::mem::replace(
            expr,
            syn::Expr::Call(syn::ExprCall {
                attrs: expr_attrs,
                func: Box::new(syn::Expr::Path(syn::ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: {
                        let mut p = hooks_core_path.clone();
                        p.segments
                            .push(syn::Ident::new("UpdateHookUninitialized", span).into());
                        p.segments
                            .push(h_ident.unwrap_or_else(|| syn::Ident::new("h", span)).into());
                        p
                    },
                })),
                paren_token: paren_token.unwrap_or_default(),
                args: Default::default(),
            }),
        );

        let hook_id = hook_id.unwrap_or_else(|| {
            let idx = used_hooks.len();
            syn::Ident::new(&format!("__hooks_hook_{idx}"), span)
        });

        if let syn::Expr::Call(syn::ExprCall { args, .. }) = expr {
            args.extend([
                actual_expr,
                syn::Expr::Path(syn::ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: hook_id.clone().into(),
                }),
            ]);
        } else {
            unreachable!()
        };

        used_hooks.push(crate::DetectedHook { ident: hook_id })
    });

    for stmt in stmts {
        if let Some(ExprOfStmtMut { expr, stmt_attrs }) = ExprOfStmtMut::try_from(stmt) {
            if stmt_attrs.map_or(true, |attrs| mutate.not_hook_attrs.might_be_hook(attrs)) {
                mutate.mutate_if_expr_is_hook(expr);
            }
        }
    }

    DetectedHooks {
        not_hook_attrs: mutate.unwrap_not_hook_attrs(),
        hooks: used_hooks,
    }
}

/// tokens inside `h![...]`
struct HMacroContent {
    explicit_hook_id: Option<(syn::Ident, syn::Token![=])>,
    expr: TokenStream,
}

impl Parse for HMacroContent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            explicit_hook_id: if input.peek(syn::Ident::peek_any) && input.peek2(syn::Token![=]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
            expr: input.parse()?,
        })
    }
}

/// attribute must be `#[not_hook]` and `#![not_hook]`
struct NotHookAttrs(Vec<syn::Attribute>);

impl NotHookAttrs {
    /// remove `#[not_hook]` and `#![not_hook]` from attrs
    /// append removed attributes to `append_removed_to`.
    /// return true if no attributes are removed (which means this might be a hook)
    pub fn might_be_hook(&mut self, attrs: &mut Vec<syn::Attribute>) -> bool {
        let mut nothing_is_removed = true;
        attrs.retain_mut(|attr| {
            if attr_is_not_hook(attr) {
                let src = syn::Attribute {
                    pound_token: attr.pound_token,
                    style: attr.style,
                    bracket_token: attr.bracket_token,
                    path: syn::Path {
                        leading_colon: None,
                        segments: Default::default(),
                    },
                    tokens: Default::default(),
                };
                nothing_is_removed = false;
                self.0.push(std::mem::replace(attr, src));
                false
            } else {
                true
            }
        });

        nothing_is_removed
    }
}

pub struct MutateHookExpr<F: FnMut(&mut syn::Expr)> {
    mutate_hook_expr: F,
    not_hook_attrs: NotHookAttrs,
}

impl<F: FnMut(&mut syn::Expr)> MutateHookExpr<F> {
    pub fn new(mutate_hook_expr: F) -> Self {
        Self {
            mutate_hook_expr,
            not_hook_attrs: NotHookAttrs(vec![]),
        }
    }

    pub fn mutate_if_expr_is_hook(&mut self, expr: &mut syn::Expr) {
        macro_rules! process_inner_expressions {
        ($e:ident . $field:ident) => {
            process_inner_expressions! { $e { $field } }
        };
        ($e:ident { $($field:ident),+ $(,)? }) => {
            if self.not_hook_attrs.might_be_hook(&mut $e.attrs) {
                $(
                    self.mutate_if_expr_is_hook(&mut $e.$field);
                )+
            }
        };
    }

        match expr {
            syn::Expr::Array(array) => {
                if self.not_hook_attrs.might_be_hook(&mut array.attrs) {
                    for elem in array.elems.iter_mut() {
                        self.mutate_if_expr_is_hook(elem);
                    }
                }
            }
            syn::Expr::Assign(e) => process_inner_expressions!(e { left, right }),
            syn::Expr::AssignOp(e) => process_inner_expressions!(e { left, right }),
            syn::Expr::Async(_) => {
                // `async {}` is untouched
            }
            syn::Expr::Await(e) => process_inner_expressions!(e.base),
            syn::Expr::Binary(e) => process_inner_expressions!(e { left, right }),
            syn::Expr::Block(_) => {
                // `{}` is untouched because it is not top level
            }
            syn::Expr::Box(e) => process_inner_expressions!(e.expr),
            syn::Expr::Break(_) => {
                // `break` is untouched
                // because there cannot be any break in top level.
            }
            syn::Expr::Call(c) => {
                if self.not_hook_attrs.might_be_hook(&mut c.attrs) {
                    for arg in c.args.iter_mut() {
                        self.mutate_if_expr_is_hook(arg);
                    }

                    if let syn::Expr::Path(path) = &*c.func {
                        if is_hook::expr_call_with_path(path) {
                            (self.mutate_hook_expr)(expr);
                        }
                    } else {
                        self.mutate_if_expr_is_hook(&mut c.func);
                    }
                }
            }
            syn::Expr::Cast(e) => process_inner_expressions!(e.expr),
            syn::Expr::Closure(_) => {
                // `|| {}` is untouched
                // because exprs in the body are not top level
            }
            syn::Expr::Continue(_) => {
                // `continue` is untouched
                // with the same reason as `break`
            }
            syn::Expr::Field(e) => process_inner_expressions!(e.base),
            syn::Expr::ForLoop(e) => process_inner_expressions!(e.expr),
            syn::Expr::Group(e) => process_inner_expressions!(e.expr),
            syn::Expr::If(e) => process_inner_expressions!(e.cond),
            syn::Expr::Index(e) => process_inner_expressions!(e { expr, index }),
            syn::Expr::Let(e) => process_inner_expressions!(e.expr),
            syn::Expr::Lit(_) => {
                // literals are untouched
                // because there is no hook
            }
            syn::Expr::Loop(_) => {
                // `loop {}` is untouched
                // because there are no exprs in top level
            }
            syn::Expr::Macro(m) => {
                if self.not_hook_attrs.might_be_hook(&mut m.attrs) && is_hook::expr_macro(m) {
                    (self.mutate_hook_expr)(expr);
                }
            }
            syn::Expr::Match(e) => process_inner_expressions!(e.expr),
            syn::Expr::MethodCall(m) => {
                if self.not_hook_attrs.might_be_hook(&mut m.attrs) {
                    for arg in m.args.iter_mut() {
                        self.mutate_if_expr_is_hook(arg);
                    }
                    self.mutate_if_expr_is_hook(&mut m.receiver);

                    if is_hook::ident(&m.method) {
                        (self.mutate_hook_expr)(expr);
                    }
                }
            }
            syn::Expr::Paren(e) => process_inner_expressions!(e.expr),
            syn::Expr::Path(_) => {
                // `std::mem::replace` is untouched
                // because there is no function call
            }
            syn::Expr::Range(r) => {
                if (self.not_hook_attrs).might_be_hook(&mut r.attrs) {
                    if let Some(e) = &mut r.from {
                        self.mutate_if_expr_is_hook(e);
                    }
                    if let Some(e) = &mut r.to {
                        self.mutate_if_expr_is_hook(e);
                    }
                }
            }
            syn::Expr::Reference(e) => process_inner_expressions!(e.expr),
            syn::Expr::Repeat(_) => {
                // `[expr; N]` is untouched
                // because the expr is not considered top level
            }
            syn::Expr::Return(e) => {
                if self.not_hook_attrs.might_be_hook(&mut e.attrs) {
                    if let Some(expr) = &mut e.expr {
                        self.mutate_if_expr_is_hook(expr);
                    }
                }
            }
            syn::Expr::Struct(s) => {
                if self.not_hook_attrs.might_be_hook(&mut s.attrs) {
                    for field in s.fields.iter_mut() {
                        process_inner_expressions!(field.expr);
                    }
                    if let Some(e) = &mut s.rest {
                        self.mutate_if_expr_is_hook(e);
                    }
                }
            }
            syn::Expr::Try(e) => process_inner_expressions!(e.expr),
            syn::Expr::TryBlock(_) => {
                // `try {}` is untouched
                // because there are no exprs in top level
            }
            syn::Expr::Tuple(t) => {
                if self.not_hook_attrs.might_be_hook(&mut t.attrs) {
                    for elem in t.elems.iter_mut() {
                        self.mutate_if_expr_is_hook(elem);
                    }
                }
            }
            syn::Expr::Type(e) => process_inner_expressions!(e.expr),
            syn::Expr::Unary(e) => process_inner_expressions!(e.expr),
            syn::Expr::Unsafe(_) => {
                // `unsafe {}` is untouched
                // because there are no exprs in top level
            }
            syn::Expr::Verbatim(_) => {
                // untouched because not interpreted by Syn
            }
            syn::Expr::While(e) => process_inner_expressions!(e.cond),
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

    pub fn unwrap_not_hook_attrs(self) -> Vec<syn::Attribute> {
        self.not_hook_attrs.0
    }
}
