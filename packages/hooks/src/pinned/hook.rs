use std::pin::Pin;

pin_project_lite::pin_project! {
    #[derive(Debug, Default)]
    pub struct Pinned<T> {
        #[pin]
        pub inner: T,
    }
}

impl<T> Pinned<T> {
    #[inline]
    pub fn pin_project_inner(self: Pin<&mut Self>) -> Pin<&mut T> {
        self.project().inner
    }
}

crate::utils::impl_hook! {
    impl [T] for Pinned<T> {
        #[inline]
        poll_next_update(self) { false .into() }
        #[inline]
        use_hook(self) -> Pin<&'hook mut T> {
            self.pin_project_inner()
        }
    }
}

#[inline]
pub fn use_default_pinned<T: Default>() -> Pinned<T> {
    Default::default()
}
