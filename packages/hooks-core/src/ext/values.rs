use std::pin::Pin;

use super::NextValue;

pin_project_lite::pin_project! {
    pub struct Values<H> {
        #[pin]
        hook: H,
    }
}

impl<H> Values<H> {
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
    pub fn next_value(self: Pin<&mut Self>) -> NextValue<H> {
        NextValue::new(self.pin_project_hook())
    }

    // #[inline]
    // pub fn
}

#[cfg(feature = "futures-core")]
impl<H> futures_core::Stream for Values<H>
where
    H: crate::NonLendingHook,
{
    type Item = H::NonGenericValue;

    #[inline]
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut hook = self.pin_project_hook();
        <H as crate::HookPollNextUpdate>::poll_next_update(hook.as_mut(), cx).map(|dynamic| {
            if dynamic {
                let value = <H as crate::NonLendingHook>::use_non_lending_hook(hook);
                Some(value)
            } else {
                None
            }
        })
    }
}
