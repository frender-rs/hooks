use std::{future::Future, pin::Pin};

use crate::{Hook, HookLifetime};

use super::{get_args::GetArgsFnOnce, inner::NextValueInner};

pub struct NextValueWith<'hook, H: ?Sized, Args, F: FnOnce(Pin<&mut H>) -> Args>(
    NextValueInner<'hook, H, GetArgsFnOnce<F>>,
);

impl<'hook, H: ?Sized, Args, F: FnOnce(Pin<&mut H>) -> Args> Unpin
    for NextValueWith<'hook, H, Args, F>
{
}

impl<'hook, H: ?Sized, Args, F: FnOnce(Pin<&mut H>) -> Args> NextValueWith<'hook, H, Args, F> {
    #[inline]
    pub fn new(hook: Pin<&'hook mut H>, get_args: F) -> Self {
        Self(NextValueInner::new(hook, GetArgsFnOnce(get_args)))
    }
}

impl<'hook, H: ?Sized, Args, F: FnOnce(Pin<&mut H>) -> Args> Future
    for NextValueWith<'hook, H, Args, F>
where
    H: Hook<Args>,
{
    type Output = Option<<H as HookLifetime<'hook, Args>>::Value>;

    #[inline]
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.get_mut().0.impl_poll(cx)
    }
}
