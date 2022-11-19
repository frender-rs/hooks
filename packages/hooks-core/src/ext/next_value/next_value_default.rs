use std::{future::Future, pin::Pin};

use crate::Hook;

use super::{get_args::GetArgsDefault, inner::NextValueInner};

pub struct NextValueWithDefaultArgs<'hook, H: ?Sized, Args>(
    NextValueInner<'hook, H, GetArgsDefault<Args>>,
);

impl<'hook, H: ?Sized, Args> Unpin for NextValueWithDefaultArgs<'hook, H, Args> {}

impl<'hook, H: ?Sized, Args> NextValueWithDefaultArgs<'hook, H, Args> {
    #[inline]
    pub fn new(hook: Pin<&'hook mut H>) -> Self {
        Self(NextValueInner::new(hook, GetArgsDefault::default()))
    }
}

impl<'hook, H: ?Sized, Args> Future for NextValueWithDefaultArgs<'hook, H, Args>
where
    H: Hook<Args>,
    Args: Default,
{
    type Output = Option<<H as Hook<Args>>::Value<'hook>>;

    #[inline]
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().0.impl_poll(cx)
    }
}
