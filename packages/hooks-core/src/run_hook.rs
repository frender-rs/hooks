use crate::Hook;

pub trait GetArgsForHook<R: ?Sized, H: Hook<Args> + ?Sized, Args> {
    fn get_args_for_hook(&mut self, runner: &mut R, hook: &mut H) -> Args;
}

impl<F, R: ?Sized, H: Hook<Args> + ?Sized, Args> GetArgsForHook<R, H, Args> for F
where
    F: ?Sized + FnMut(&mut R, &mut H) -> Args,
{
    #[inline]
    fn get_args_for_hook(&mut self, runner: &mut R, hook: &mut H) -> Args {
        self(runner, hook)
    }
}

pub struct CloneArgsForHook<Args: Clone>(pub Args);

impl<R: ?Sized, H: Hook<Args> + ?Sized, Args: Clone> GetArgsForHook<R, H, Args>
    for CloneArgsForHook<Args>
{
    #[inline]
    fn get_args_for_hook(&mut self, _runner: &mut R, _hook: &mut H) -> Args {
        self.0.clone()
    }
}

pub struct DefaultArgsForHook;

impl<R: ?Sized, H: Hook<Args> + ?Sized, Args: Default> GetArgsForHook<R, H, Args>
    for DefaultArgsForHook
{
    #[inline]
    fn get_args_for_hook(&mut self, _runner: &mut R, _hook: &mut H) -> Args {
        Args::default()
    }
}

/// A type implementing `RunHook` is a *hook runner*.
pub trait RunHook<H: Hook<Args>, G: GetArgsForHook<Self, H, Args>, Args> {
    type RunningHook;

    fn run_hook(self, hook: H, get_args: G) -> Self::RunningHook;
}

pub mod default_runner {
    use std::{
        future::Future,
        marker::PhantomData,
        pin::Pin,
        task::{Context, Poll},
    };

    use super::GetArgsForHook;
    use crate::{Hook, HookExt, HookLifetime, HookPollNextUpdateExt};

    #[derive(Debug, Default, Clone, Copy)]
    pub struct Runner;

    pub struct RunningHook<H, G, Args> {
        hook: H,
        get_args: G,
        args: PhantomData<Args>,
    }

    pub struct RunningHookNextValue<'hook, H, G, Args> {
        running_hook: Option<&'hook mut RunningHook<H, G, Args>>,
    }

    impl<'hook, H, G, Args> Future for RunningHookNextValue<'hook, H, G, Args>
    where
        H: Hook<Args> + Unpin,
        G: GetArgsForHook<Runner, H, Args>,
    {
        type Output = Option<<H as HookLifetime<'hook, Args>>::Value>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.get_mut();
            let running_hook = this.running_hook.as_mut().unwrap();

            match HookPollNextUpdateExt::poll_next_update(&mut running_hook.hook, cx) {
                Poll::Ready(dynamic) => {
                    if dynamic {
                        let running_hook = this.running_hook.take().unwrap();
                        let args = running_hook
                            .get_args
                            .get_args_for_hook(&mut Runner, &mut running_hook.hook);
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

    impl<H: Hook<Args>, G, Args> RunningHook<H, G, Args> {
        pub fn next_value(self: &mut Self) -> RunningHookNextValue<'_, H, G, Args> {
            RunningHookNextValue {
                running_hook: Some(self),
            }
        }
    }

    impl<H: Hook<Args>, G: GetArgsForHook<Self, H, Args>, Args> super::RunHook<H, G, Args> for Runner {
        type RunningHook = RunningHook<H, G, Args>;

        fn run_hook(self, hook: H, get_args: G) -> Self::RunningHook {
            RunningHook {
                hook,
                get_args,
                args: PhantomData,
            }
        }
    }
}
