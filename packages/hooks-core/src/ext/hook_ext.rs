use std::{pin::Pin, task::Poll};

use super::NextUpdate;
use crate::{
    default_runner, CloneArgsForHook, DefaultArgsForHook, GetArgsForHook, Hook, HookLifetime,
    HookPollNextUpdate, RunHook,
};

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

    #[inline]
    fn run_by_runner_with_get_args<
        'run,
        R: RunHook<&'run mut Self, G, Args>,
        G: GetArgsForHook<R, &'run mut Self, Args>,
    >(
        &'run mut self,
        runner: R,
        get_args: G,
    ) -> R::RunningHook
    where
        Self: Unpin + 'run,
    {
        R::run_hook(runner, self, get_args)
    }

    #[inline]
    fn run_by_runner_with_clone_args<
        'run,
        R: RunHook<&'run mut Self, CloneArgsForHook<Args>, Args>,
    >(
        &'run mut self,
        runner: R,
        args: Args,
    ) -> R::RunningHook
    where
        Self: Unpin + 'run,
        Args: Clone,
    {
        self.run_by_runner_with_get_args(runner, CloneArgsForHook(args))
    }

    #[inline]
    fn run_by_runner_with_default_args<'run, R: RunHook<&'run mut Self, DefaultArgsForHook, Args>>(
        &'run mut self,
        runner: R,
    ) -> R::RunningHook
    where
        Self: Unpin + 'run,
        Args: Default,
    {
        self.run_by_runner_with_get_args(runner, DefaultArgsForHook)
    }

    #[inline]
    fn run_with_get_args<'run, G: GetArgsForHook<default_runner::Runner, &'run mut Self, Args>>(
        &'run mut self,
        get_args: G,
    ) -> default_runner::RunningHook<&'run mut Self, G, Args>
    where
        Self: Unpin + 'run,
    {
        self.run_by_runner_with_get_args(default_runner::Runner, get_args)
    }

    #[inline]
    fn run_with_clone_args<'run>(
        &'run mut self,
        args: Args,
    ) -> default_runner::RunningHook<&'run mut Self, CloneArgsForHook<Args>, Args>
    where
        Self: Unpin + 'run,
        Args: Clone,
    {
        self.run_with_get_args(CloneArgsForHook(args))
    }

    #[inline]
    fn run_with_default_args<'run>(
        &'run mut self,
    ) -> default_runner::RunningHook<&'run mut Self, DefaultArgsForHook, Args>
    where
        Self: Unpin + 'run,
        Args: Default,
    {
        self.run_with_get_args(DefaultArgsForHook)
    }

    #[inline]
    fn into_run_by_runner_with_get_args<
        R: RunHook<Self, G, Args>,
        G: GetArgsForHook<R, Self, Args>,
    >(
        self,
        runner: R,
        get_args: G,
    ) -> R::RunningHook
    where
        Self: Sized,
    {
        R::run_hook(runner, self, get_args)
    }

    #[inline]
    fn into_run_by_runner_with_clone_args<R: RunHook<Self, CloneArgsForHook<Args>, Args>>(
        self,
        runner: R,
        args: Args,
    ) -> R::RunningHook
    where
        Self: Sized,
        Args: Clone,
    {
        self.into_run_by_runner_with_get_args(runner, CloneArgsForHook(args))
    }

    #[inline]
    fn into_run_by_runner_with_default_args<R: RunHook<Self, DefaultArgsForHook, Args>>(
        self,
        runner: R,
    ) -> R::RunningHook
    where
        Self: Sized,
        Args: Default,
    {
        self.into_run_by_runner_with_get_args(runner, DefaultArgsForHook)
    }

    #[inline]
    fn into_run_with_get_args<G: GetArgsForHook<default_runner::Runner, Self, Args>>(
        self,
        get_args: G,
    ) -> default_runner::RunningHook<Self, G, Args>
    where
        Self: Sized,
    {
        self.into_run_by_runner_with_get_args(default_runner::Runner, get_args)
    }

    #[inline]
    fn into_run_with_clone_args(
        self,
        args: Args,
    ) -> default_runner::RunningHook<Self, CloneArgsForHook<Args>, Args>
    where
        Self: Sized,
        Args: Clone,
    {
        self.into_run_with_get_args(CloneArgsForHook(args))
    }

    #[inline]
    fn into_run_with_default_args(
        self,
    ) -> default_runner::RunningHook<Self, DefaultArgsForHook, Args>
    where
        Self: Sized,
        Args: Default,
    {
        self.into_run_with_get_args(DefaultArgsForHook)
    }
}

impl<Args, H: Hook<Args> + ?Sized> HookExt<Args> for H {}
