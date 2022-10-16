use std::{pin::Pin, task::Poll};

use super::NextUpdate;
use crate::{Hook, HookLifetime};

pub trait HookExt: Hook {
    /// A shortcut to call [`Hook::poll_next_update`] on Unpin hooks.
    #[inline]
    fn poll_next_update(&mut self, cx: &mut std::task::Context<'_>) -> Poll<bool>
    where
        Self: Unpin,
    {
        Hook::poll_next_update(Pin::new(self), cx)
    }

    /// A shortcut to call [`Hook::use_hook`] on Unpin hooks.
    #[inline]
    fn use_hook<'hook>(
        &'hook mut self,
        args: <Self as HookLifetime<'hook>>::Args,
    ) -> <Self as HookLifetime<'hook>>::Value
    where
        Self: Unpin,
    {
        <Self as Hook>::use_hook(Pin::new(self), args)
    }

    /// Get a future which polls [`Hook::poll_next_update`].
    #[inline]
    fn next_update(&mut self) -> NextUpdate<'_, Self>
    where
        Self: Unpin,
    {
        NextUpdate::new(self)
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

impl<H: Hook + ?Sized> HookExt for H {}
