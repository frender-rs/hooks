use std::pin::Pin;

use hooks_core::HookPollNextUpdateExt;

use super::SharedStateEqData;

pub struct SharedStateEq<T: PartialEq>(Option<SharedStateEqData<T>>);

impl<T: PartialEq> SharedStateEq<T> {
    pub fn use_hook_with(
        self: Pin<&mut Self>,
        get_initial_state: impl FnOnce() -> T,
    ) -> &SharedStateEqData<T> {
        self.get_mut()
            .0
            .get_or_insert_with(|| SharedStateEqData::new(get_initial_state()))
    }
}

impl<T: PartialEq> Unpin for SharedStateEq<T> {}

pub struct SharedStateEqWith<T: PartialEq>(SharedStateEq<T>);

impl<T: PartialEq> Unpin for SharedStateEqWith<T> {}

crate::utils::impl_hook! {
    impl [T: PartialEq] for SharedStateEq<T> {
        #[inline]
        poll_next_update(self, cx) {
            self.get_mut().0.as_mut().map_or(
                ::core::task::Poll::Ready(true),
                |this| this.impl_poll_next_update(cx),
            )
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
    SharedStateEq(None)
}

#[inline]
pub fn use_shared_state_eq_with<T: PartialEq>() -> SharedStateEqWith<T> {
    SharedStateEqWith(use_shared_state_eq())
}
