use std::pin::Pin;

pin_project_lite::pin_project![
    #[derive(Debug)]
    pub struct LazyPinned<T> {
        #[pin]
        inner: lazy_pinned::LazyPinned<T>,
    }
];

impl<T> Default for LazyPinned<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T> LazyPinned<T> {
    pub fn pin_project(self: Pin<&mut Self>) -> Pin<&mut lazy_pinned::LazyPinned<T>> {
        self.project().inner
    }
}

hooks_core::impl_hook![
    type For<T> = LazyPinned<T>;
    fn unmount() {}
    #[inline]
    fn poll_next_update(self) {
        ::core::task::Poll::Ready(false)
    }
];

pub struct UseLazyPinned<T>(pub T);
pub use UseLazyPinned as use_lazy_pinned;

hooks_core::impl_hook![
    type For<T> = UseLazyPinned<T>;
    fn into_hook(self) -> super::pinned::Pinned<T> {
        super::pinned::Pinned { inner: self.0 }
    }
    #[inline]
    fn update_hook(self, _hook: _) {}
    #[inline]
    fn h(self, hook: LazyPinned<T>) {
        hook.project().inner.pin_project_or_insert(self.0)
    }
];

pub struct UseLazyPinnedWith<T, F: FnOnce() -> T>(pub F);
pub use UseLazyPinnedWith as use_lazy_pinned_with;

hooks_core::impl_hook![
    type For<T, F> = UseLazyPinnedWith<T, F>
        where __![F: FnOnce() -> T]: __;
    fn into_hook(self) -> super::pinned::Pinned<T> {
        super::pinned::Pinned { inner: self.0() }
    }
    #[inline]
    fn update_hook(self, _hook: _) {}
    #[inline]
    fn h(self, hook: LazyPinned<T>) {
        hook.project().inner.pin_project_or_insert_with(self.0)
    }
];

#[cfg(test)]
mod tests {
    use std::{marker::PhantomPinned, pin::Pin};

    use hooks_core::{HookPollNextUpdateExt, UpdateHookUninitialized};

    #[test]
    fn test_lazy_pinned() {
        let hook = super::LazyPinned::<PhantomPinned>::default();

        futures_lite::pin!(hook);
        futures_lite::future::block_on(async {
            assert!(hook.as_mut().pin_project().0.is_none());
            assert!(!hook.next_update().await);
            let _: Pin<&mut PhantomPinned> = super::use_lazy_pinned(PhantomPinned).h(hook.as_mut());
            assert!(hook.as_mut().pin_project().0.is_some());
            assert!(!hook.next_update().await);
            let _: Pin<&mut PhantomPinned> =
                super::use_lazy_pinned_with(|| unreachable!()).h(hook.as_mut());
            assert!(hook.as_mut().pin_project().0.is_some());
            assert!(!hook.next_update().await);
        })
    }
}
