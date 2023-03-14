use crate::detect;

pub struct ExprCallPath<'a> {
    pub func_path: &'a mut syn::ExprPath,
    pub paren_token: &'a mut syn::token::Paren,
    pub args: &'a mut syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
}
