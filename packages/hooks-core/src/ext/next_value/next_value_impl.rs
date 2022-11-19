use std::{future::Future, pin::Pin};

use crate::Hook;

use super::{get_args::GetArgsMove, inner::NextValueInner};

pub struct NextValue<'hook, H: ?Sized, Args>(NextValueInner<'hook, H, GetArgsMove<Args>>);

impl<'hook, H: ?Sized, Args> Unpin for NextValue<'hook, H, Args> {}

impl<'hook, H: ?Sized, Args> NextValue<'hook, H, Args> {
    #[inline]
    pub fn new(hook: Pin<&'hook mut H>, args: Args) -> Self {
        Self(NextValueInner::new(hook, GetArgsMove(args)))
    }
}

impl<'hook, H: ?Sized, Args> Future for NextValue<'hook, H, Args>
where
    H: Hook<Args>,
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
