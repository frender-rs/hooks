use std::{pin::Pin, task::Poll};

use super::NextUpdate;
use crate::{Hook, HookLifetime, HookPollNextUpdate};

pub trait HookPollNextUpdateExt: HookPollNextUpdate {
    /// A shortcut to call [`Hook::poll_next_update`] on Unpin hooks.
    #[inline]
    fn poll_next_update(&mut self, cx: &mut std::task::Context<'_>) -> Poll<bool>
    where
        Self: Unpin,
    {
        HookPollNextUpdate::poll_next_update(Pin::new(self), cx)
    }

    /// Get a future which polls [`Hook::poll_next_update`].
    #[inline]
    fn next_update(&mut self) -> NextUpdate<'_, Self>
    where
        Self: Unpin,
    {
        NextUpdate::new(self)
    }
}

impl<H: HookPollNextUpdate> HookPollNextUpdateExt for H {}

pub trait HookExt<Args>: Hook<Args> {
    /// A shortcut to call [`Hook::use_hook`] on Unpin hooks.
    #[inline]
    fn use_hook<'hook>(&'hook mut self, args: Args) -> <Self as HookLifetime<'hook, Args>>::Value
    where
        Self: Unpin,
    {
        <Self as Hook<Args>>::use_hook(Pin::new(self), args)
    }

    // fn run_with_args<'hook, A: FnMut(&mut Self) -> <Self as HookLifetime<'hook>>::Args>(
    //     &'hook mut self,
    //     get_args: A,
    // ) -> HookRun<'hook, Self, A> {
    //     HookRun {
    //         hook: self,
    //         get_args,
    //     }
    // }
}

impl<Args, H: Hook<Args>> HookExt<Args> for H {}
