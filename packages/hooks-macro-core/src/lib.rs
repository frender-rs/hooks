mod captures;
mod utils;

pub use utils::path_or_lit::PathOrLit;

mod detect;
mod detected;
pub use detect::*;
pub use detected::*;

mod hook_macro;

pub use hook_macro::*;

pub use {darling, proc_macro2, quote, syn};
