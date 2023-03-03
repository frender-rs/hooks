#![deny(clippy::undocumented_unsafe_blocks)]

mod ext;
mod hook;
mod macro_impl_hook;

pub use ext::*;
pub use hook::*;
pub use macro_impl_hook::*;

pub mod fn_hook;
pub mod hook_pair;

pub mod erased_hook;
pub use erased_hook::DynNonLendingHook;

pub(crate) mod utils;

mod v2_macro_impl_hook;
pub use v2_macro_impl_hook::*;

#[doc(hidden)]
pub mod prelude_h {
    pub use crate::h;
}
