use std::pin::Pin;

/// Pinned data which can be lazily initialized.
///
/// ## [`LazyPinned<T>`] vs. [`Option<T>`]
///
/// [`LazyPinned<T>`] act like [`Option<T>`].
/// In fact, `LazyPinned<T>` is implemented by just wrapping `Option<T>`.
/// However, they have different behaviors in pinning.
///
/// `Pin<P<Option<T>>>` guarantees the `Option<T>` is not moved,
/// where `P<_>` is a pointer type which deref to `_`.
/// Thus, when the data is `None`, it cannot be set to `Some(T)` unless
/// `T: Unpin`. This means the whole
///
/// `Pin<P<LazyPinned<T>>>` only guarantees the inner `T` is pinned.
/// Thus, `Pin<&mut LazyPinned<T>>` can `get_pin_or_insert_with`.
#[derive(Debug)]
pub struct LazyPinned<T>(pub Option<T>);

impl<T> Default for LazyPinned<T> {
    #[inline]
    fn default() -> Self {
        Self(None)
    }
}

impl<T> LazyPinned<T> {
    #[inline]
    #[must_use]
    pub fn as_pin_ref(self: Pin<&Self>) -> Option<Pin<&T>> {
        Pin::get_ref(self).0.as_ref().map(|x| {
            // SAFETY: `x` is guaranteed to be pinned because it comes from `self`
            // which is pinned.
            unsafe { Pin::new_unchecked(x) }
        })
    }

    #[inline]
    #[must_use]
    pub fn as_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>> {
        // SAFETY: `get_unchecked_mut` is never used to move the `Option` inside `self`.
        // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
        unsafe {
            Pin::get_unchecked_mut(self)
                .0
                .as_mut()
                .map(|x| Pin::new_unchecked(x))
        }
    }

    pub fn get_pin_or_insert_with(self: Pin<&mut Self>, f: impl FnOnce() -> T) -> Pin<&mut T> {
        // SAFETY: `get_unchecked_mut` is never used to move the `Some(T)` inside `self`.
        let this = unsafe { Pin::get_unchecked_mut(self) };
        let x = this.0.get_or_insert_with(f);
        // SAFETY: `x` is guaranteed to be pinned because it comes from `self` which is pinned.
        unsafe { Pin::new_unchecked(x) }
    }
}

impl<T> LazyPinned<T> {
    #[inline]
    pub fn use_hook_with(
        self: Pin<&mut Self>,
        get_initial_value: impl FnOnce() -> T,
    ) -> Pin<&mut T> {
        self.get_pin_or_insert_with(get_initial_value)
    }
}

crate::utils::impl_hook! {
    impl [T] for LazyPinned<T> {
        #[inline]
        poll_next_update(self) {
            ::core::task::Poll::Ready(false)
        }
        #[inline]
        use_hook(self, initial_value: T) -> Pin<&'hook mut T> {
            self.get_pin_or_insert_with(move || initial_value)
        }
    }
}

pin_project_lite::pin_project! {
    #[derive(Debug)]
    pub struct LazyPinnedWith<T> {
        #[pin]
        inner: LazyPinned<T>,
    }
}

impl<T> Default for LazyPinnedWith<T> {
    #[inline]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

crate::utils::impl_hook! {
    impl [T] for LazyPinnedWith<T> {
        #[inline]
        poll_next_update(self) {
            ::core::task::Poll::Ready(false)
        }
        #[inline]
        use_hook[F: FnOnce() -> T](self, get_initial_value: F) -> Pin<&'hook mut T> {
            self.project().inner.get_pin_or_insert_with(get_initial_value)
        }
    }
}

#[inline]
pub fn use_lazy_pinned<T>() -> LazyPinned<T> {
    LazyPinned(None)
}

#[inline]
pub fn use_lazy_pinned_with<T>() -> LazyPinnedWith<T> {
    LazyPinnedWith {
        inner: use_lazy_pinned(),
    }
}

#[cfg(test)]
mod tests {
    use std::{marker::PhantomPinned, pin::Pin};

    use hooks_core::HookExt;

    #[test]
    fn test_lazy_pinned() {
        let hook = super::use_lazy_pinned::<PhantomPinned>();
        futures_lite::pin!(hook);

        futures_lite::future::block_on(async {
            assert!(hook.next_value_with_default_args().await.is_none());
            let _: Pin<&mut PhantomPinned> = hook.use_hook((PhantomPinned,));
            assert!(hook.next_value_with_default_args().await.is_none());
            let _: Pin<&mut PhantomPinned> = hook.as_mut().use_hook_with(|| unreachable!());
            assert!(hook.next_value_with_default_args().await.is_none());
        })
    }
}
