use crate::{sealed, Hook, HookLifetime};

pub trait RunHookBounds<H: ?Sized> {
    type Bounds: ?Sized;
}

pub trait GetArgsForHook<R: RunHook<H> + ?Sized, H: Hook + ?Sized> {
    fn get_args_for_hook<'call>(
        &'call mut self,
        runner: &'call mut R,
        hook: &mut H,
    ) -> <H as HookLifetime<'call>>::Args;
}

impl<F, R: RunHook<H> + ?Sized, H: Hook + ?Sized> GetArgsForHook<R, H> for F
where
    F: ?Sized + for<'call> FnMut(&'call mut R, &mut H) -> <H as HookLifetime<'call>>::Args,
{
    // #[inline]
    fn get_args_for_hook<'call>(
        &'call mut self,
        runner: &'call mut R,
        hook: &mut H,
    ) -> <H as HookLifetime<'call>>::Args {
        self(runner, hook)
    }
}

pub trait RunHookLifetime<
    'hook,
    H: Hook + ?Sized,
    ImplicitBounds: sealed::RunHookLifetimeBounds<'hook, Self, H> = &'hook <Self as RunHookBounds<
        H,
    >>::Bounds,
>: RunHookBounds<H>
{
    type RunningHook;
}

/// A type implementing `RunHook` is a *hook runner*.
pub trait RunHook<H: Hook + ?Sized>: for<'hook> RunHookLifetime<'hook, H> {
    fn run_hook<'hook>(
        &'hook mut self,
        hook: &'hook mut H,
        get_args: &'hook mut dyn GetArgsForHook<Self, H>,
    ) -> <Self as RunHookLifetime<'hook, H>>::RunningHook;
}

pub mod default_runner {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };

    use crate::{Hook, HookExt, HookLifetime, RunHookBounds};

    pub struct Runner;

    pub struct RunningHook<'run, H: ?Sized> {
        runner: &'run mut Runner,
        hook: &'run mut H,
        get_args: &'run mut dyn super::GetArgsForHook<Runner, H>,
    }

    pub struct RunningHookNextValue<'hook, 'run, H: ?Sized> {
        running_hook: Option<&'hook mut RunningHook<'run, H>>,
    }

    impl<'hook, 'run, H> Future for RunningHookNextValue<'hook, 'run, H>
    where
        H: Hook + ?Sized + Unpin,
    {
        type Output = Option<<H as HookLifetime<'hook>>::Value>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.get_mut();
            let running_hook = this.running_hook.as_mut().unwrap();

            match HookExt::poll_next_update(running_hook.hook, cx) {
                Poll::Ready(dynamic) => {
                    if dynamic {
                        let running_hook = this.running_hook.take().unwrap();
                        let args = running_hook
                            .get_args
                            .get_args_for_hook(running_hook.runner, running_hook.hook);
                        let v = running_hook.hook.use_hook(args);
                        Poll::Ready(Some(v))
                    } else {
                        Poll::Ready(None)
                    }
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }

    impl<'run, H: ?Sized> RunningHook<'run, H> {
        pub fn next_value(self: &mut Self) -> RunningHookNextValue<'_, 'run, H> {
            RunningHookNextValue {
                running_hook: Some(self),
            }
        }
    }

    impl<H: ?Sized> RunHookBounds<H> for Runner {
        type Bounds = H;
    }

    impl<'hook, H: Hook + ?Sized> super::RunHookLifetime<'hook, H> for Runner {
        type RunningHook = RunningHook<'hook, H>;
    }

    impl<H: Hook + ?Sized> super::RunHook<H> for Runner {
        fn run_hook<'hook>(
            &'hook mut self,
            hook: &'hook mut H,
            get_args: &'hook mut dyn crate::GetArgsForHook<Self, H>,
        ) -> <Self as crate::RunHookLifetime<'hook, H>>::RunningHook {
            RunningHook {
                runner: self,
                hook,
                get_args,
            }
        }
    }
}
