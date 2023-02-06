mod utils;

pub use utils::path_or_lit::PathOrLit;

pub mod tlpc;

mod detect;
mod detected;
pub use detect::*;
pub use detected::*;

mod hook_macro;

pub use hook_macro::*;

pub use {darling, proc_macro2, quote, syn};
