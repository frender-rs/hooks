use std::{pin::Pin, task::Poll};

use crate::{Hook, HookPollNextUpdate};

use super::NextUpdate;

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

pub trait HookExt<Args>: Hook<Args> {
    /// A shortcut to call [`Hook::use_hook`] on Unpin hooks.
    #[inline]
    fn use_hook(&mut self, args: Args) -> Self::Value<'_>
    where
        Self: Unpin,
    {
        <Self as Hook<Args>>::use_hook(Pin::new(self), args)
    }

    #[inline]
    fn next_value(&mut self, args: Args) -> crate::NextValue<'_, Self, Args>
    where
        Self: Unpin,
    {
        crate::NextValue::new(Pin::new(self), args)
    }

    #[inline]
    fn next_value_with<F: FnOnce(Pin<&mut Self>) -> Args>(
        &mut self,
        get_args: F,
    ) -> crate::NextValueWith<'_, Self, Args, F>
    where
        Self: Unpin,
    {
        crate::NextValueWith::new(Pin::new(self), get_args)
    }

    #[inline]
    fn next_value_with_default_args(&mut self) -> crate::NextValueWithDefaultArgs<'_, Self, Args>
    where
        Self: Unpin,
        Args: Default,
    {
        crate::NextValueWithDefaultArgs::new(Pin::new(self))
    }
}

impl<Args, H: Hook<Args> + ?Sized> HookExt<Args> for H {}
