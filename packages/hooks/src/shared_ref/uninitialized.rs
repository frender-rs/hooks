use super::SharedRef;

#[derive(Debug)]
pub struct SharedRefUninitialized<T>(pub Option<SharedRef<T>>);

impl<T> Default for SharedRefUninitialized<T> {
    #[inline]
    fn default() -> Self {
        Self(None)
    }
}

impl<T> Unpin for SharedRefUninitialized<T> {}

hooks_core::impl_hook![
    type For<T> = SharedRefUninitialized<T>;
    fn unmount() {}
    #[inline(always)]
    fn poll_next_update(self, _cx: _) {
        std::task::Poll::Ready(false)
    }
];
