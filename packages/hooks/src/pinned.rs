use std::{marker::PhantomData, pin::Pin};

pin_project_lite::pin_project![
    #[derive(Debug, Default)]
    pub struct Pinned<T> {
        #[pin]
        pub inner: T,
    }
];

impl<T> Pinned<T> {
    #[inline]
    pub fn pin_project_inner(self: Pin<&mut Self>) -> Pin<&mut T> {
        self.project().inner
    }
}

hooks_core::impl_hook![
    type For<T> = Pinned<T>;

    fn unmount() {}

    #[inline]
    fn poll_next_update(self) {
        std::task::Poll::Ready(false)
    }
    #[inline]
    fn use_hook(self) -> Pin<&'hook mut T> {
        self.pin_project_inner()
    }
];

pub struct UseDefaultPinned<T: Default>(PhantomData<T>);

hooks_core::impl_hook![
    type For<T: Default> = UseDefaultPinned<T>;
    #[inline(always)]
    fn into_hook(self) -> Pinned<T> {
        Default::default()
    }
    #[inline(always)]
    fn update_hook(self, _hook: _) {}
    #[inline(always)]
    fn h(self, hook: Pinned<T>) {
        hooks_core::Hook::use_hook(hook)
    }
];

#[inline(always)]
pub fn use_default_pinned<T: Default>() -> UseDefaultPinned<T> {
    UseDefaultPinned(PhantomData)
}
