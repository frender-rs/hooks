use std::pin::Pin;

use hooks_core::{Hook, HookExt, IntoHook, UpdateHook};

#[derive(Debug)]
pub struct UninitializedHook<H>(pub Option<H>);

impl<H: Hook + Unpin> UninitializedHook<H> {
    #[inline]
    pub(crate) fn use_into_or_update_hook(
        &mut self,
        into_hook: impl IntoHook<Hook = H> + UpdateHook,
    ) -> hooks_core::Value![H] {
        let hook = match &mut self.0 {
            Some(hook) => {
                into_hook.update_hook(Pin::new(hook));
                hook
            }
            hook @ None => hook.insert(into_hook.into_hook()),
        };
        hook.use_hook()
    }

    pub(crate) fn use_with(
        &mut self,
        into: impl FnOnce() -> H,
        update: impl FnOnce(Pin<&mut H>),
    ) -> hooks_core::Value![H] {
        let hook = match &mut self.0 {
            Some(hook) => {
                update(Pin::new(hook));
                hook
            }
            hook @ None => hook.insert(into()),
        };
        hook.use_hook()
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
