use std::pin::Pin;

use hooks_core::HookPollNextUpdateExt;

use super::SharedStateData;

#[derive(Debug)]
pub struct SharedState<T>(super::inner::SharedState<SharedStateData<T>>);

impl<T> Default for SharedState<T> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> SharedState<T> {
    pub fn use_hook_with(
        self: Pin<&mut Self>,
        get_initial_state: impl FnOnce() -> T,
    ) -> &SharedStateData<T> {
        self.get_mut().0.get_or_init_with(move |waker| {
            SharedStateData::new_with_waker(get_initial_state(), waker)
        })
    }
}

impl<T> Unpin for SharedState<T> {}

#[derive(Debug)]
pub struct SharedStateWith<T>(SharedState<T>);

impl<T> Default for SharedStateWith<T> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Unpin for SharedStateWith<T> {}

crate::utils::impl_hook! {
    impl [T] for SharedState<T> {
        #[inline]
        poll_next_update(self, cx) {
            self.get_mut().0.impl_poll_next_update(cx, SharedStateData::impl_poll_next_update)
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
    Default::default()
}

#[inline]
pub fn use_shared_state_with<T>() -> SharedStateWith<T> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use futures_lite::StreamExt;
    use hooks_core::AsyncIterableHook;

    use crate::{hook, use_effect, use_shared_state, ShareValue};

    #[test]
    fn shared_state() {
        #[hook(hooks_core_path = "hooks_core")]
        fn use_test() -> i32 {
            let state = use_shared_state(0);

            let value = state.get();
            let s = state.clone();

            use_effect(
                move |v: &_| {
                    if *v < 2 {
                        s.set(*v + 1);
                    }
                },
                value,
            );

            value
        }

        futures_lite::future::block_on(async {
            let values = use_test().into_iter().collect::<Vec<_>>().await;
            assert_eq!(values, [0, 1, 2]);
        });
    }
}
