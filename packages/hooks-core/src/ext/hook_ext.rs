use std::{pin::Pin, task::Poll};

use crate::{Hook, HookPollNextUpdate, HookValue};

use super::NextUpdate;

/// Extend [`HookPollNextUpdate`] with convenient methods.
pub trait HookPollNextUpdateExt: HookPollNextUpdate {
    /// A shortcut to call [`HookPollNextUpdate::poll_next_update`] on Unpin hooks.
    #[inline]
    fn poll_next_update(&mut self, cx: &mut std::task::Context<'_>) -> Poll<bool>
    where
        Self: Unpin,
    {
        HookPollNextUpdate::poll_next_update(Pin::new(self), cx)
    }

    /// Get a future which polls [`HookPollNextUpdate::poll_next_update`].
    #[inline]
    fn next_update(&mut self) -> NextUpdate<'_, Self>
    where
        Self: Unpin,
    {
        NextUpdate::new(self)
    }
}

impl<H: HookPollNextUpdate + ?Sized> HookPollNextUpdateExt for H {}

/// Extend [`Hook`](trait@Hook) with convenient methods.
pub trait HookExt: Hook {
    /// A shortcut to call [`Hook::use_hook`] on Unpin hooks.
    #[inline(always)]
    fn use_hook(&mut self) -> <Self as HookValue<'_>>::Value
    where
        Self: Unpin,
    {
        <Self as Hook>::use_hook(Pin::new(self))
    }

    #[inline]
    fn next_value(&mut self) -> super::NextValue<'_, Self>
    where
        Self: Unpin,
    {
        crate::NextValue::new(Pin::new(self))
    }

    #[inline]
    fn into_values(self) -> super::Values<Self>
    where
        Self: Sized,
    {
        super::Values::new(self)
    }

    #[inline]
    fn values(&mut self) -> super::Values<&mut Self>
    where
        Self: Unpin,
    {
        super::Values::new(self)
    }
}

impl<H: Hook + ?Sized> HookExt for H {}
