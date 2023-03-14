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

#[cfg(feature = "use_debug")]
pub mod debug;

#[cfg(feature = "use_default_pinned")]
pub mod pinned;

#[cfg(feature = "use_lazy_pinned")]
pub mod lazy_pinned;

#[cfg(feature = "use_lazy_pinned_hook")]
pub mod lazy_pinned_hook;

#[cfg(feature = "use_poll_next_update")]
pub mod poll_next_update;

#[cfg(feature = "ShareValue")]
mod share_value;
#[cfg(feature = "ShareValue")]
pub use share_value::ShareValue;

#[cfg(feature = "use_shared_ref")]
pub mod shared_ref;
#[cfg(feature = "use_shared_ref")]
pub use shared_ref::SharedRef;

#[cfg(feature = "use_shared_state")]
pub mod shared_state;

#[cfg(feature = "use_state")]
pub mod state;

pub mod prelude {
    pub use hooks_core::prelude::*;

    #[cfg(feature = "derive")]
    pub use hooks_derive::hook;

    #[cfg(feature = "use_debug")]
    pub use crate::debug::use_debug;
    #[cfg(feature = "use_effect")]
    pub use crate::effect::{
        use_effect, use_effect_on_next_poll, use_effect_once, use_effect_once_with, use_effect_with,
    };
    #[cfg(feature = "use_mut")]
    pub use crate::hook_mut::{use_mut, use_mut_default, use_mut_with};
    #[cfg(feature = "use_lazy_pinned")]
    pub use crate::lazy_pinned::{use_lazy_pinned, use_lazy_pinned_with};
    #[cfg(feature = "use_lazy_pinned_hook")]
    pub use crate::lazy_pinned_hook::use_lazy_pinned_hook;
    #[cfg(feature = "use_memo")]
    pub use crate::memo::use_memo;
    #[cfg(feature = "use_default_pinned")]
    pub use crate::pinned::use_default_pinned;
    #[cfg(feature = "use_poll_next_update")]
    pub use crate::poll_next_update::use_poll_next_update;
    #[cfg(feature = "use_shared_ref")]
    pub use crate::shared_ref::{use_shared_ref, use_shared_ref_with};
    #[cfg(feature = "use_shared_state")]
    pub use crate::shared_state::{
        use_shared_state, use_shared_state_eq, use_shared_state_eq_with, use_shared_state_with,
    };
    #[cfg(feature = "use_state")]
    pub use crate::state::{
        use_state, use_state_default, use_state_default_n, use_state_eq, use_state_eq_default,
        use_state_eq_default_n, use_state_eq_n, use_state_eq_n_with, use_state_eq_with,
        use_state_n, use_state_n_with, use_state_with,
    };

    #[cfg(feature = "ShareValue")]
    pub use crate::ShareValue;
}

pub use hooks_core as core;
pub use hooks_core::{
    hook_fn, impl_hook, Hook, HookExt, HookPollNextUpdate, HookPollNextUpdateExt, IntoHook,
    UpdateHook, UpdateHookUninitialized,
};
pub use prelude::*;

pub(crate) mod utils;
