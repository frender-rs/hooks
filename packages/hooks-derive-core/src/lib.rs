mod utils;

pub use utils::path_or_lit::PathOrLit;

mod hook_macro;

pub use hook_macro::*;

pub use {darling, proc_macro2, quote, syn};
