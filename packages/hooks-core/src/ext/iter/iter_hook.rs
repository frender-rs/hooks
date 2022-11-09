use std::pin::Pin;

use crate::{Hook, HookPollNextUpdate, NextValue, NonLendingHook};

pin_project_lite::pin_project! {
    pub struct IterHook<H> {
        #[pin]
        hook: H,
    }
}

impl<H> IterHook<H> {
    #[inline]
    pub fn new(hook: H) -> Self {
        Self { hook }
    }

    #[inline]
    pub fn into_inner(self) -> H {
        self.hook
    }

    #[inline]
    pub fn pin_project_hook(self: Pin<&mut Self>) -> Pin<&mut H> {
        self.project().hook
    }

    #[inline]
    pub fn next_value(&mut self) -> NextValue<H, ()>
    where
        Self: Unpin,
    {
        NextValue::new(Pin::new(self).pin_project_hook(), ())
    }
}

#[cfg(feature = "futures-core")]
impl<H> futures_core::Stream for IterHook<H>
where
    H: NonLendingHook<()>,
{
    type Item = H::NonGenericValue;

    #[inline]
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut hook = self.pin_project_hook();
        <H as HookPollNextUpdate>::poll_next_update(hook.as_mut(), cx).map(|dynamic| {
            if dynamic {
                let value = <H as Hook<()>>::use_hook(hook, ());
                Some(value)
            } else {
                None
            }
        })
    }
}
