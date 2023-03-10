#![deny(clippy::undocumented_unsafe_blocks)]

mod macro_impl;
pub use macro_impl::*;

mod macros;
pub use macros::*;

mod traits;
pub use traits::*;

mod ext;
mod hook_tuple;

pub use ext::*;
pub use hook_tuple::*;

pub mod fn_hook;

pub(crate) mod utils;

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
