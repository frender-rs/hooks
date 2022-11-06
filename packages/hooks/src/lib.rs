#![deny(clippy::undocumented_unsafe_blocks)]

mod effect;
mod lazy_pinned;
mod ref_hook;
mod share_value;
mod shared_state;
mod state;

pub use effect::*;
pub use lazy_pinned::*;
pub use ref_hook::*;
pub use share_value::*;
pub use shared_state::*;
pub use state::*;

pub use hooks_core as core;
pub use hooks_core::{
    Hook, HookBounds, HookExt, HookLifetime, HookPollNextUpdate, HookPollNextUpdateExt, RunHook,
};

pub use hooks_derive::hook;

pub(crate) mod utils;
