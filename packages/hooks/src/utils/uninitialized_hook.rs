use std::pin::Pin;

use hooks_core::{Hook, HookExt, IntoHook};

#[derive(Debug)]
pub struct UninitializedHook<H>(pub Option<H>);

impl<H: Hook + Unpin> UninitializedHook<H> {
    #[inline]
    pub(crate) fn use_hook(&mut self, get_hook: impl FnOnce() -> H) -> hooks_core::Value![H] {
        self.0.get_or_insert_with(get_hook).use_hook()
    }
    #[inline]
    pub(crate) fn use_into_hook(
        &mut self,
        into_hook: impl IntoHook<Hook = H>,
    ) -> hooks_core::Value![H] {
        self.use_hook(|| into_hook.into_hook())
    }
}

impl<H> Default for UninitializedHook<H> {
    fn default() -> Self {
        Self(None)
    }
}

hooks_core::impl_hook![
    type For<H: Unpin + Hook> = UninitializedHook<H>;

    fn unmount(self) {
        if let Some(hook) = &mut self.get_mut().0 {
            H::unmount(Pin::new(hook))
        }
    }
    fn poll_next_update(self, cx: _) {
        if let Some(hook) = &mut self.get_mut().0 {
            H::poll_next_update(Pin::new(hook), cx)
        } else {
            std::task::Poll::Ready(false)
        }
    }
];
