#![deny(clippy::undocumented_unsafe_blocks)]
#![doc = include_str!("../README.md")]

mod effect;
mod hook_mut;
mod memo;
mod pinned;
mod pinned_hook;
mod ref_hook;
mod share_value;
mod shared_state;
mod state;

pub use effect::*;
pub use hook_mut::*;
pub use memo::*;
pub use pinned::*;
pub use pinned_hook::*;
pub use ref_hook::*;
pub use share_value::*;
pub use shared_state::*;
pub use state::*;

pub use hooks_core as core;
pub use hooks_core::{
    dyn_hook, impl_hook, AsyncIterableHook, DynNonLendingHook, Hook, HookBounds, HookExt,
    HookLifetime, HookPollNextUpdate, HookPollNextUpdateExt,
};

pub use hooks_derive::hook;

pub(crate) mod utils;
