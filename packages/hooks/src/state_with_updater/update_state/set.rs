#[cfg(feature = "use_shared_set")]
pub use shared::*;

#[cfg(feature = "use_gen_set")]
pub use gen::*;

use super::UpdateState;

#[derive(Debug)]
pub struct Set<T> {
    pub new_value: Option<T>,
}

impl<S> Set<S> {
    pub fn set(&mut self, new_value: S) {
        self.new_value = Some(new_value)
    }
}

impl<T> Default for Set<T> {
    fn default() -> Self {
        Self { new_value: None }
    }
}

impl<T> UpdateState<T> for Set<T> {
    fn update_state(&mut self, state: &mut T) -> bool {
        if let Some(this) = self.new_value.take() {
            *state = this;
            true
        } else {
            false
        }
    }
}

#[cfg(feature = "use_shared_set")]
mod shared {
    use super::{
        super::{
            use_shared_update_state, use_shared_update_state_with, SharedUpdateState,
            UseSharedUpdateState, UseSharedUpdateStateWith,
        },
        Set,
    };

    pub type SharedSet<S> = SharedUpdateState<Set<S>>;

    impl<S> SharedSet<S> {
        pub fn set(&self, new_value: S) {
            self.map_mut_update_state(|v| v.set(new_value))
        }
    }

    pub type UseSharedSet<S> = UseSharedUpdateState<S, Set<S>>;
    pub fn use_shared_set<S>(initial_state: S) -> UseSharedSet<S> {
        use_shared_update_state(initial_state, Set::default())
    }

    pub type UseSharedSetWith<S, F> = UseSharedUpdateStateWith<S, Set<S>, F>;
    pub fn use_shared_set_with<S>(
        get_initial_state: impl FnOnce() -> S,
    ) -> UseSharedSetWith<S, impl FnOnce() -> (S, SharedSet<S>)> {
        use_shared_update_state_with(move || (get_initial_state(), Set::default()))
    }

    #[cfg(test)]
    mod tests {
        use crate::{Hook, HookPollNextUpdateExt, IntoHook};

        #[test]
        fn set() {
            futures_lite::future::block_on(async {
                let hook = super::use_shared_set(1).into_hook();
                futures_lite::pin!(hook);

                assert!(!std::future::poll_fn(|cx| hook.poll_next_update(cx)).await);

                let (state, updater) = hook.as_mut().use_hook();
                assert_eq!(*state, 1);
                updater.set(2);
                assert_eq!(*state, 1);

                assert!(std::future::poll_fn(|cx| hook.poll_next_update(cx)).await);
                let (state, _updater) = hook.as_mut().use_hook();
                assert_eq!(*state, 2);

                assert!(!std::future::poll_fn(|cx| hook.poll_next_update(cx)).await);
            });
        }

        #[cfg(feature = "futures-core")]
        #[cfg(feature = "use_effect")]
        #[test]
        fn hook_fn_state_2() {
            use futures_lite::StreamExt;
            use hooks_core::{hook_fn, IntoHook};

            use crate::{use_shared_set, use_shared_set_with};

            hook_fn!(
                fn use_state_2() -> (i32, i32) {
                    let (state_1, updater_1) = h!(use_shared_set(1));
                    let (state_2, updater_2) = h!(use_shared_set_with(|| *state_1 + 1));

                    let ret = (*state_1, *state_2);

                    let updater_1 = updater_1.clone();
                    let updater_2 = updater_2.clone();
                    h![crate::use_effect(
                        move |(v1, v2): &_| {
                            if *v2 > 10 {
                                return;
                            }
                            updater_1.set(*v2);
                            updater_2.set(*v1 + *v2);
                        },
                        ret,
                    )];

                    ret
                }
            );

            futures_lite::future::block_on(async {
                let values = use_state_2().into_hook_values().collect::<Vec<_>>().await;

                assert_eq!(values, [(1, 2), (2, 3), (3, 5), (5, 8), (8, 13)]);
            });
        }

        #[cfg(feature = "futures-core")]
        #[cfg(feature = "use_effect")]
        #[cfg(feature = "proc-macro")]
        #[test]
        fn state_2() {
            use futures_lite::StreamExt;
            use hooks_core::IntoHook;
            use hooks_macro::hook;

            use super::{use_shared_set, use_shared_set_with};

            #[hook(hooks_core_path = "::hooks_core")]
            fn use_state_2() -> (i32, i32) {
                let (state_1, updater_1) = use_shared_set(1);
                let (state_2, updater_2) = use_shared_set_with(|| *state_1 + 1);

                let ret = (*state_1, *state_2);

                let updater_1 = updater_1.clone();
                let updater_2 = updater_2.clone();
                crate::use_effect(
                    move |(v1, v2): &_| {
                        if *v2 > 10 {
                            return;
                        }
                        updater_1.set(*v2);
                        updater_2.set(*v1 + *v2);
                    },
                    ret,
                );

                ret
            }

            futures_lite::future::block_on(async {
                let values = use_state_2().into_hook_values().collect::<Vec<_>>().await;

                assert_eq!(values, [(1, 2), (2, 3), (3, 5), (5, 8), (8, 13)]);
            });
        }
    }
}

#[cfg(feature = "use_gen_set")]
mod gen {
    use super::{
        super::{
            use_gen_update_state, use_gen_update_state_with, GenUpdateState, GenUpdateStateKey,
            UseGenUpdateState, UseGenUpdateStateWith,
        },
        Set,
    };

    pub type GenSet<S> = GenUpdateState<Set<S>>;
    pub type GenSetKey<S> = GenUpdateStateKey<Set<S>>;

    impl<S> GenSetKey<S> {
        pub fn set(&self, new_value: S) {
            self.map_mut_update_state(|v| v.set(new_value))
        }
    }

    pub type UseGenSet<S> = UseGenUpdateState<S, Set<S>>;
    pub fn use_gen_set<S>(initial_state: S) -> UseGenSet<S> {
        use_gen_update_state(initial_state, Set::default())
    }

    pub type UseGenSetWith<F> = UseGenUpdateStateWith<F>;
    pub fn use_gen_set_with<S>(
        get_initial_state: impl FnOnce() -> S,
    ) -> UseGenSetWith<impl FnOnce() -> (S, Set<S>)> {
        use_gen_update_state_with(move || (get_initial_state(), Set::default()))
    }

    #[cfg(test)]
    mod tests {
        use crate::{utils::testing::assert_always_pending, Hook, HookPollNextUpdateExt, IntoHook};

        #[test]
        fn set() {
            futures_lite::future::block_on(async {
                let hook = super::use_gen_set(1).into_hook();
                futures_lite::pin!(hook);

                assert_always_pending(|| std::future::poll_fn(|cx| hook.poll_next_update(cx)));

                let (state, updater) = hook.as_mut().use_hook();
                assert_eq!(*state, 1);
                updater.set(2);
                assert_eq!(*state, 1);

                assert!(std::future::poll_fn(|cx| hook.poll_next_update(cx)).await);
                let (state, _updater) = hook.as_mut().use_hook();
                assert_eq!(*state, 2);

                assert_always_pending(|| std::future::poll_fn(|cx| hook.poll_next_update(cx)));
            });
        }

        #[cfg(feature = "futures-core")]
        #[cfg(feature = "use_effect")]
        #[test]
        fn hook_fn_state_2() {
            use futures_lite::StreamExt;
            use hooks_core::{hook_fn, IntoHook};

            use super::{use_gen_set, use_gen_set_with};

            hook_fn!(
                fn use_state_2() -> (i32, i32) {
                    let (state_1, updater_1) = h!(use_gen_set(1));
                    let (state_2, updater_2) = h!(use_gen_set_with(|| *state_1 + 1));

                    let ret = (*state_1, *state_2);

                    h![crate::use_effect(
                        move |(v1, v2): &_| {
                            if *v2 > 10 {
                                return;
                            }
                            updater_1.set(*v2);
                            updater_2.set(*v1 + *v2);
                        },
                        ret,
                    )];

                    ret
                }
            );

            futures_lite::future::block_on(async {
                let mut hook_values = use_state_2().into_hook_values();
                let values = (&mut hook_values).take(5).collect::<Vec<_>>().await;

                assert_eq!(values, [(1, 2), (2, 3), (3, 5), (5, 8), (8, 13)]);

                assert_always_pending(|| hook_values.next())
            });
        }

        #[cfg(feature = "futures-core")]
        #[cfg(feature = "use_effect")]
        #[cfg(feature = "proc-macro")]
        #[test]
        fn state_2() {
            use futures_lite::StreamExt;
            use hooks_core::IntoHook;
            use hooks_macro::hook;

            use super::{use_gen_set, use_gen_set_with};

            #[hook(hooks_core_path = "::hooks_core")]
            fn use_state_2() -> (i32, i32) {
                let (state_1, updater_1) = use_gen_set(1);
                let (state_2, updater_2) = use_gen_set_with(|| *state_1 + 1);

                let ret = (*state_1, *state_2);

                crate::use_effect(
                    move |(v1, v2): &_| {
                        if *v2 > 10 {
                            return;
                        }
                        updater_1.set(*v2);
                        updater_2.set(*v1 + *v2);
                    },
                    ret,
                );

                ret
            }

            futures_lite::future::block_on(async {
                let mut hook_values = use_state_2().into_hook_values();
                let values = (&mut hook_values).take(5).collect::<Vec<_>>().await;

                assert_eq!(values, [(1, 2), (2, 3), (3, 5), (5, 8), (8, 13)]);

                assert_always_pending(|| hook_values.next())
            });
        }
    }
}
