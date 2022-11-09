use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::{Hook, HookLifetime, HookPollNextUpdate};

use super::get_args::GetArgs;

struct HookAndGetArgs<'hook, H: ?Sized, G> {
    hook: Pin<&'hook mut H>,
    get_args: G,
}

pub struct NextValueInner<'hook, H: ?Sized, G> {
    inner: Option<HookAndGetArgs<'hook, H, G>>,
}

impl<'hook, H: ?Sized, G> Unpin for NextValueInner<'hook, H, G> {}

impl<'hook, H: ?Sized, G> NextValueInner<'hook, H, G> {
    #[inline]
    pub fn new(hook: Pin<&'hook mut H>, get_args: G) -> Self {
        Self {
            inner: Some(HookAndGetArgs { hook, get_args }),
        }
    }

    pub fn impl_poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<<H as HookLifetime<'hook, G::Args>>::Value>>
    where
        G: GetArgs<H>,
        H: Hook<G::Args>,
    {
        let inner = self.inner.as_mut().unwrap();
        let hook = inner.hook.as_mut();

        <H as HookPollNextUpdate>::poll_next_update(hook, cx).map(|dynamic| {
            let mut inner = self.inner.take().unwrap();
            if dynamic {
                let args = inner.get_args.get_args(inner.hook.as_mut());
                let value = <H as Hook<G::Args>>::use_hook(inner.hook, args);
                Some(value)
            } else {
                None
            }
        })
    }
}
