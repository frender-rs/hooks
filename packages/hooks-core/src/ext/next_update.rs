use std::{future::Future, pin::Pin, task::Poll};

use crate::HookPollNextUpdate;

pub struct NextUpdate<'a, H: ?Sized> {
    hook: &'a mut H,
}

impl<'a, H: ?Sized> NextUpdate<'a, H> {
    pub fn new(hook: &'a mut H) -> Self {
        Self { hook }
    }
}

impl<'a, H: HookPollNextUpdate + Unpin + ?Sized> Future for NextUpdate<'a, H> {
    type Output = bool;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let hook = &mut self.get_mut().hook;
        Pin::new(hook).poll_next_update(cx)
    }
}
