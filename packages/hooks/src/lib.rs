#![deny(clippy::undocumented_unsafe_blocks)]

mod effect;
mod pinned;
mod pinned_hook;
mod ref_hook;
mod share_value;
mod shared_state;
mod state;

pub use effect::*;
pub use pinned::*;
pub use pinned_hook::*;
pub use ref_hook::*;
pub use share_value::*;
pub use shared_state::*;
pub use state::*;

pub use hooks_core as core;
pub use hooks_core::{
    AsyncIterableHook, Hook, HookBounds, HookExt, HookLifetime, HookPollNextUpdate,
    HookPollNextUpdateExt,
};

pub use hooks_derive::hook;

pub(crate) mod utils;
