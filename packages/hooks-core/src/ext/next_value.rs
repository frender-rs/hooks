use std::{future::Future, pin::Pin};

use crate::{Hook, HookPollNextUpdate, HookValue};

/// A future which outputs the next value of the hook.
/// See [`HookExt::next_value`](crate::HookExt::next_value).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct NextValue<'hook, H: ?Sized> {
    hook: Option<Pin<&'hook mut H>>,
}

impl<'hook, H: ?Sized> Unpin for NextValue<'hook, H> {}

impl<'hook, H: ?Sized> NextValue<'hook, H> {
    #[inline]
    pub fn new(hook: Pin<&'hook mut H>) -> Self {
        Self { hook: Some(hook) }
    }
}

impl<'hook, H: ?Sized> Future for NextValue<'hook, H>
where
    H: Hook,
{
    type Output = Option<<H as HookValue<'hook>>::Value>;

    #[inline]
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = &mut self.get_mut().hook;
        let hook = this
            .as_mut()
            .expect("NextValue should not be polled again after ready");
        let hook = hook.as_mut();

        <H as HookPollNextUpdate>::poll_next_update(hook, cx).map(|dynamic| {
            let hook = this.take().unwrap();
            if dynamic {
                let value = <H as Hook>::use_hook(hook);
                Some(value)
            } else {
                None
            }
        })
    }
}
