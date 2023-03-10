use std::task::Poll;

#[derive(Debug)]
pub struct PollNextUpdate<P: FnMut(&mut std::task::Context<'_>) -> Poll<bool>>(pub P);

impl<P: FnMut(&mut std::task::Context<'_>) -> Poll<bool>> Unpin for PollNextUpdate<P> {}

hooks_core::impl_hook![
    type For<P: FnMut(&mut std::task::Context<'_>) -> Poll<bool>> = PollNextUpdate<P>;
    fn unmount() {}
    #[inline]
    fn poll_next_update(self, cx: _) {
        self.get_mut().0(cx)
    }
    #[inline(always)]
    fn use_hook(self) {}

    fn update_hook(self, hook: _) {
        hook.get_mut().0 = self.0;
    }
    fn h(self, hook: UninitializedHook<Self>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
];

pub use PollNextUpdate as use_poll_next_update;

use crate::utils::UninitializedHook;
