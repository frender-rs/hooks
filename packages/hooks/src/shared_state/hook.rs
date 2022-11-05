use std::pin::Pin;

use hooks_core::HookPollNextUpdateExt;

use super::SharedStateData;

pub struct SharedState<T>(Option<SharedStateData<T>>);

impl<T> SharedState<T> {
    pub fn use_hook_with(
        self: Pin<&mut Self>,
        get_initial_state: impl FnOnce() -> T,
    ) -> &SharedStateData<T> {
        self.get_mut()
            .0
            .get_or_insert_with(|| SharedStateData::new(get_initial_state()))
    }
}

impl<T> Unpin for SharedState<T> {}

pub struct SharedStateWith<T>(SharedState<T>);

impl<T> Unpin for SharedStateWith<T> {}

crate::utils::impl_hook! {
    impl [T] for SharedState<T> {
        #[inline]
        poll_next_update(self, cx) {
            self.get_mut().0.as_mut().map_or(
                ::core::task::Poll::Ready(true),
                |this| this.impl_poll_next_update(cx),
            )
        }

        #[inline]
        use_hook(self, initial_state: T) -> &'hook SharedStateData<T> {
            self.use_hook_with(move || initial_state)
        }
    }
}

crate::utils::impl_hook! {
    impl [T] for SharedStateWith<T> {
        #[inline]
        poll_next_update(self, cx) {
            self.get_mut().0.poll_next_update(cx)
        }

        #[inline]
        use_hook[F: FnOnce() -> T](self, get_initial_state: F) -> &'hook SharedStateData<T> {
            Pin::new(&mut self.get_mut().0).use_hook_with(get_initial_state)
        }
    }
}

#[inline]
pub fn use_shared_state<T>() -> SharedState<T> {
    SharedState(None)
}

#[inline]
pub fn use_shared_state_with<T>() -> SharedStateWith<T> {
    SharedStateWith(use_shared_state())
}
