#![deny(clippy::undocumented_unsafe_blocks)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "use_effect")]
pub mod effect;

#[cfg(feature = "use_effect")]
pub use effect::{effect_fn, get_new_dep_and_effect};

#[cfg(feature = "use_mut")]
pub mod hook_mut;

#[cfg(feature = "use_memo")]
pub mod memo;

#[cfg(feature = "use_pinned")]
pub mod pinned;

#[cfg(feature = "use_pinned_hook")]
pub mod pinned_hook;

#[cfg(feature = "use_ref")]
pub mod ref_hook;

#[cfg(feature = "ShareValue")]
pub mod share_value;

#[cfg(feature = "use_shared_state")]
pub mod shared_state;

#[cfg(feature = "use_state")]
mod state;

pub mod prelude {
    #[cfg(feature = "use_effect")]
    pub use crate::effect::{
        use_effect, use_effect_on_next_poll, use_effect_once, use_effect_once_with, use_effect_with,
    };
    pub use hooks_core::prelude::*;
}

pub use hooks_core as core;
pub use hooks_core::{
    hook_fn, impl_hook, Hook, HookExt, HookPollNextUpdate, HookPollNextUpdateExt, IntoHook,
    UpdateHook, UpdateHookUninitialized,
};
pub use prelude::*;

#[cfg(feature = "derive")]
pub use hooks_derive::hook;

pub(crate) mod utils;
