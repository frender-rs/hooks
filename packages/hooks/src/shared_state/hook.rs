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
}
