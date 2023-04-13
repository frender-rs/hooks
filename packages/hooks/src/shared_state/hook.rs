use super::SharedState;

pub struct UseSharedState<T>(pub T);
pub use UseSharedState as use_shared_state;

hooks_core::impl_hook![
    type For<T> = UseSharedState<T>;
    #[inline]
    fn into_hook(self) -> SharedState<T> {
        SharedState::new(self.0)
    }
    #[inline(always)]
    fn update_hook(self, _hook: _) {}
    fn h(self, hook: crate::utils::UninitializedHook<SharedState<T>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
];

pub struct UseSharedStateWith<T, F: FnOnce() -> T>(pub F);
pub use UseSharedStateWith as use_shared_state_with;

hooks_core::impl_hook![
    type For<T, F: FnOnce() -> T> = UseSharedStateWith<T, F>;

    #[inline]
    fn into_hook(self) -> SharedState<T> {
        SharedState::new(self.0())
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}
    fn h(self, hook: crate::utils::UninitializedHook<SharedState<T>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
];

#[cfg(feature = "futures-core")]
#[cfg(test)]
mod tests {
    use futures_lite::StreamExt;
    use hooks_core::hook_fn;

    use crate::{use_shared_state, ShareValue};

    #[test]
    #[cfg(feature = "use_effect")]
    fn shared_state() {
        use hooks_core::IntoHook;

        use crate::use_effect;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h![use_shared_state(0)];

                let value = state.get();
                let s = state.clone();

                h![use_effect(
                    move |v: &_| {
                        if *v < 2 {
                            s.set(*v + 1);
                        }
                    },
                    value,
                )];

                value
            }
        );

        futures_lite::future::block_on(async {
            let values = use_test().into_hook_values();

            let values = values.collect::<Vec<_>>().await;
            assert_eq!(values, [0, 1, 2]);
        });
    }

    #[test]
    fn drop_in_map() {
        use hooks_core::IntoHook;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h!(use_shared_state(0));

                let value = state.get();
                let s = state.clone();

                let _: () = state.map(|_| drop(s));

                value
            }
        );

        assert_eq!(
            futures_lite::future::block_on(use_test().into_hook_values().collect::<Vec<_>>(),),
            [0]
        )
    }

    #[test]
    fn drop_in_conditional_map_mut() {
        use hooks_core::IntoHook;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h!(use_shared_state(0));

                let value = state.get();
                let s = state.clone();

                if value == 0 {
                    let _: () = state.map_mut(|v| {
                        drop(s);
                        *v = 1;
                    });
                }

                value
            }
        );

        assert_eq!(
            futures_lite::future::block_on(use_test().into_hook_values().collect::<Vec<_>>(),),
            [0, 1]
        )
    }

    fn assert_timeout<Fut: std::future::Future>(
        get_fut: impl 'static + Send + FnOnce() -> Fut,
        timeout: u64,
    ) {
        use std::sync::mpsc::RecvTimeoutError;

        let (tx, rx) = std::sync::mpsc::sync_channel(0);

        let hook_thread = std::thread::spawn(move || {
            let _ = futures_lite::future::block_on(get_fut());
            tx.send(()).unwrap();
        });

        assert!(matches!(
            rx.recv_timeout(std::time::Duration::from_millis(timeout)),
            Err(RecvTimeoutError::Timeout)
        ));

        assert!(!hook_thread.is_finished());
    }

    fn assert_always_pending<Fut: std::future::Future>(
        get_fut: impl 'static + Send + FnOnce() -> Fut,
    ) {
        const TIMEOUT: u64 = 100;
        assert_timeout(get_fut, TIMEOUT)
    }

    #[test]
    fn reference_cycle_should_always_pending() {
        use hooks_core::IntoHook;

        struct Data(Option<super::SharedState<Self>>);

        hook_fn!(
            fn use_test() {
                let state = h!(use_shared_state(Data(None))).clone();
                state.set(Data(Some(state.clone())));
            }
        );

        assert_always_pending(|| use_test().into_hook_values().collect::<Vec<_>>());
    }

    #[test]
    fn unconditional_map_mut_should_always_pending() {
        use hooks_core::IntoHook;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h!(use_shared_state(0));

                let value = state.get();

                let _: () = state.map_mut(|_| {});

                value
            }
        );

        assert_always_pending(|| use_test().into_hook_values().collect::<Vec<_>>());
    }
}
