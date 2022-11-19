use std::pin::Pin;

use hooks_core::HookPollNextUpdateExt;

use super::SharedStateEqData;

#[derive(Debug)]
pub struct SharedStateEq<T: PartialEq>(super::inner::SharedState<SharedStateEqData<T>>);

impl<T: PartialEq> Default for SharedStateEq<T> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: PartialEq> SharedStateEq<T> {
    #[inline]
    pub fn use_hook_with(
        self: Pin<&mut Self>,
        get_initial_state: impl FnOnce() -> T,
    ) -> &SharedStateEqData<T> {
        self.get_mut().0.get_or_init_with(move |waker| {
            SharedStateEqData::new_with_waker(get_initial_state(), waker)
        })
    }
}

impl<T: PartialEq> Unpin for SharedStateEq<T> {}

#[derive(Debug)]
pub struct SharedStateEqWith<T: PartialEq>(SharedStateEq<T>);

impl<T: PartialEq> Default for SharedStateEqWith<T> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: PartialEq> Unpin for SharedStateEqWith<T> {}

crate::utils::impl_hook! {
    impl [T: PartialEq] for SharedStateEq<T> {
        #[inline]
        poll_next_update(self, cx) {
            self.get_mut().0.impl_poll_next_update(cx, SharedStateEqData::impl_poll_next_update)
        }

        #[inline]
        use_hook(self, initial_state: T) -> &'hook SharedStateEqData<T> {
            self.use_hook_with(move || initial_state)
        }
    }
}

crate::utils::impl_hook! {
    impl [T: PartialEq] for SharedStateEqWith<T> {
        #[inline]
        poll_next_update(self, cx) {
            self.get_mut().0.poll_next_update(cx)
        }

        #[inline]
        use_hook[F: FnOnce() -> T](self, get_initial_state: F) -> &'hook SharedStateEqData<T> {
            Pin::new(&mut self.get_mut().0).use_hook_with(get_initial_state)
        }
    }
}

#[inline]
pub fn use_shared_state_eq<T: PartialEq>() -> SharedStateEq<T> {
    Default::default()
}

#[inline]
pub fn use_shared_state_eq_with<T: PartialEq>() -> SharedStateEqWith<T> {
    Default::default()
}
