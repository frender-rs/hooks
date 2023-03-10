#![deny(clippy::undocumented_unsafe_blocks)]

pub mod identity;

mod traits;
pub use traits::*;

mod ext;
mod hook_tuple;
mod macro_impl_hook;

pub use ext::*;
pub use hook_tuple::*;
pub use macro_impl_hook::*;

pub mod fn_hook;

pub(crate) mod utils;

mod v2_macro_impl_hook;
pub use v2_macro_impl_hook::*;

pub mod prelude {
    pub use crate::{
        fn_hook, hook_fn, Hook, HookExt as _, HookPollNextUpdate, HookPollNextUpdateExt as _,
        IntoHook as _, NonLendingHook,
    };
}

#[doc(hidden)]
pub mod prelude_h {
    pub use crate::h;
}
