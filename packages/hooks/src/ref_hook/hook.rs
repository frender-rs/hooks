use super::SharedRef;

#[derive(Debug)]
pub struct Ref<T>(Option<SharedRef<T>>);

impl<T> Default for Ref<T> {
    #[inline]
    fn default() -> Self {
        Self(None)
    }
}

impl<T> Unpin for Ref<T> {}

#[derive(Debug)]
pub struct RefWith<T>(Ref<T>);

impl<T> Default for RefWith<T> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Unpin for RefWith<T> {}

impl<T> Ref<T> {
    pub fn use_hook_with(
        self: std::pin::Pin<&mut Self>,
        get_initial_value: impl FnOnce() -> T,
    ) -> &SharedRef<T> {
        self.get_mut()
            .0
            .get_or_insert_with(move || SharedRef::new(get_initial_value()))
    }
}

crate::utils::impl_hook! {
    impl [T] for Ref<T> {
        #[inline]
        poll_next_update(self) {
            ::core::task::Poll::Ready(false)
        }
        #[inline]
        use_hook(self, initial_value: T) -> &'hook SharedRef<T> {
            self.use_hook_with(move || initial_value)
        }
    }
}

crate::utils::impl_hook! {
    impl [T] for RefWith<T> {
        #[inline]
        poll_next_update(self) {
            ::core::task::Poll::Ready(false)
        }
        #[inline]
        use_hook[F: FnOnce() -> T](self, get_initial_value: F) -> &'hook SharedRef<T> {
            std::pin::Pin::new(&mut self.get_mut().0).use_hook_with(get_initial_value)
        }
    }
}

#[inline]
pub fn use_ref<T>() -> Ref<T> {
    Default::default()
}

#[inline]
pub fn use_ref_with<T>() -> RefWith<T> {
    Default::default()
}
