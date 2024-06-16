#![forbid(unsafe_code)]
#![cfg_attr(all(feature = "futures-core", feature = "all", feature = "proc-macro"), doc = include_str!("../README.md"))]

#[cfg(feature = "use_reused")]
pub mod reused;

#[cfg(feature = "use_effect")]
pub mod effect;

#[cfg(feature = "use_effect")]
pub use effect::effect_fn;

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

#[cfg(feature = "Signal")]
mod signal;
#[cfg(feature = "Signal")]
pub use signal::{use_signal, Signal, SignalHook, UseSignal};

#[cfg(feature = "use_shared_ref")]
pub mod shared_ref;
#[cfg(feature = "use_shared_ref")]
pub use shared_ref::SharedRef;

#[cfg(feature = "use_shared_state")]
pub mod shared_state;

#[cfg(feature = "use_state_with_updater")]
mod state_with_updater;
#[cfg(feature = "use_state_with_updater")]
pub use state_with_updater::{StateUpdater, StateWithUpdater};

#[cfg(feature = "UpdateState")]
pub use state_with_updater::{IntoUpdateStateResult, UpdateState};

#[cfg(feature = "use_shared_update_state")]
pub use state_with_updater::{SharedUpdateState, UseSharedUpdateState, UseSharedUpdateStateWith};

#[cfg(feature = "use_shared_call")]
pub use state_with_updater::{Call, SharedCall, UseSharedCall, UseSharedCallWith};
#[cfg(feature = "use_shared_reducer")]
pub use state_with_updater::{
    Reduce, Reducer, SharedReduce, SharedReducer, UseSharedReduce, UseSharedReduceWith,
    UseSharedReducer, UseSharedReducerWith,
};
#[cfg(feature = "use_shared_set")]
pub use state_with_updater::{Set, SharedSet, UseSharedSet, UseSharedSetWith};
#[cfg(feature = "use_shared_toggle")]
pub use state_with_updater::{SharedToggle, Toggle, UseSharedToggle, UseSharedToggleWith};

#[cfg(feature = "use_uninitialized_hook")]
pub mod uninitialized_hook;

mod signal_eq;
pub use signal_eq::SignalEq;

pub mod prelude {
    pub use hooks_core::prelude::*;

    #[cfg(feature = "proc-macro")]
    pub use hooks_macro::hook;

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
    #[cfg(feature = "use_reused")]
    pub use crate::reused::{use_reused, ReusableHookExt as _};
    #[cfg(feature = "use_shared_ref")]
    pub use crate::shared_ref::{use_shared_ref, use_shared_ref_with};
    #[cfg(feature = "use_shared_state")]
    pub use crate::shared_state::{
        use_shared_state, use_shared_state_eq, use_shared_state_eq_with, use_shared_state_with,
    };

    #[cfg(feature = "use_state_with_updater")]
    pub use crate::state_with_updater::{use_state_with_updater, use_state_with_updater_with};

    #[cfg(feature = "use_shared_update_state")]
    pub use crate::state_with_updater::{use_shared_update_state, use_shared_update_state_with};

    #[cfg(feature = "use_shared_call")]
    pub use crate::state_with_updater::{use_shared_call, use_shared_call_with};
    #[cfg(feature = "use_shared_reducer")]
    pub use crate::state_with_updater::{
        use_shared_reduce, use_shared_reduce_with, use_shared_reducer, use_shared_reducer_with,
    };
    #[cfg(feature = "use_shared_set")]
    pub use crate::state_with_updater::{use_shared_set, use_shared_set_with};
    #[cfg(feature = "use_shared_toggle")]
    pub use crate::state_with_updater::{use_shared_toggle, use_shared_toggle_with};

    #[cfg(feature = "use_uninitialized_hook")]
    pub use crate::uninitialized_hook::use_uninitialized_hook;

    #[cfg(feature = "ShareValue")]
    pub use crate::ShareValue;

    #[cfg(feature = "Signal")]
    pub use crate::{use_signal, Signal};
}

pub use hooks_core as core;
pub use hooks_core::{
    hook_fn, impl_hook, Hook, HookExt, HookPollNextUpdate, HookPollNextUpdateExt, HookUnmount,
    HookValue, IntoHook, UpdateHook, UpdateHookUninitialized, Value,
};
pub use prelude::*;

pub(crate) mod utils;

mod state_owner;
