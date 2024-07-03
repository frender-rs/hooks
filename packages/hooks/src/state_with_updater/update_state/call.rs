#[cfg(feature = "use_shared_call")]
pub use shared::*;

#[cfg(feature = "use_gen_call")]
pub use gen::*;

use super::{IntoUpdateStateResult, UpdateState};

pub struct Call<F> {
    pub f: F,
    pub times: usize,
}

impl<F> Call<F> {
    pub const fn new(f: F) -> Self {
        Self { f, times: 0 }
    }

    pub fn call(&mut self) {
        self.times += 1
    }
}

impl<F: FnMut(&mut S) -> R, S: ?Sized, R: IntoUpdateStateResult> UpdateState<S> for Call<F> {
    fn update_state(&mut self, state: &mut S) -> bool {
        let mut changed = false;

        while self.times > 0 {
            let res = (self.f)(state);
            if !changed {
                changed = res.into_update_state_result();
            }
            self.times -= 1;
        }

        changed
    }
}

#[cfg(feature = "use_shared_call")]
mod shared {
    use super::{
        super::{
            use_shared_update_state, use_shared_update_state_with, IntoUpdateStateResult,
            SharedUpdateState, UseSharedUpdateState, UseSharedUpdateStateWith,
        },
        Call,
    };

    pub type SharedCall<F> = SharedUpdateState<Call<F>>;

    impl<F> SharedCall<F> {
        pub fn call(&self) {
            self.map_mut_update_state(Call::call)
        }
    }

    pub type UseSharedCall<S, F> = UseSharedUpdateState<S, Call<F>>;
    /// Note that only the initial `f` will be called.
    pub fn use_shared_call<S, F: FnMut(&mut S) -> R, R: IntoUpdateStateResult>(
        initial_state: S,
        f: F,
    ) -> UseSharedCall<S, F> {
        use_shared_update_state(initial_state, Call::new(f))
    }

    pub type UseSharedCallWith<S, F, F2> = UseSharedUpdateStateWith<S, Call<F>, F2>;
    pub fn use_shared_call_with<S, F: FnMut(&mut S) -> R, R: IntoUpdateStateResult>(
        f: impl FnOnce() -> (S, F),
    ) -> UseSharedCallWith<S, F, impl FnOnce() -> (S, SharedCall<F>)> {
        use_shared_update_state_with(move || {
            let (initial_state, f) = f();
            (initial_state, Call::new(f))
        })
    }

    #[cfg(test)]
    mod tests {
        use futures_lite::future::block_on;
        use hooks_core::{HookExt, IntoHook};

        use crate::{hook_fn, use_shared_call, SharedCall};

        type SharedMultiply2 = SharedCall<fn(&mut u32)>;
        hook_fn!(
            fn use_value() -> (u32, &'hook SharedMultiply2) {
                let (value, call) = h![use_shared_call(1, (|v| *v *= 2) as fn(&mut u32))];
                (*value, call)
            }
        );

        #[test]
        fn call() {
            block_on(async {
                let mut hook = use_value().into_hook();
                let (value, _) = hook.next_value().await.unwrap();
                assert_eq!(value, 1);
                assert!(hook.next_value().await.is_none());

                {
                    let (value, updater) = hook.use_hook();
                    assert_eq!(value, 1);
                    updater.call();
                    assert_eq!(hook.next_value().await.unwrap().0, 2);
                }

                {
                    let (value, updater) = hook.use_hook();
                    assert_eq!(value, 2);
                    updater.call();
                    updater.call();
                    updater.call();
                    assert_eq!(hook.next_value().await.unwrap().0, 16);
                }

                assert!(hook.next_value().await.is_none());
            })
        }
    }
}

#[cfg(feature = "use_gen_call")]
mod gen {
    use super::{
        super::{
            use_gen_update_state, use_gen_update_state_with, GenUpdateStateKey,
            GenUpdateStateOwner, IntoUpdateStateResult, UseGenUpdateState, UseGenUpdateStateWith,
        },
        Call,
    };

    pub type GenCallOwner<F> = GenUpdateStateOwner<Call<F>>;
    pub type GenCallKey<F> = GenUpdateStateKey<Call<F>>;

    impl<F> GenCallKey<F> {
        pub fn call(&self) {
            self.map_mut_update_state(Call::call)
        }
    }

    pub type UseGenCall<S, F> = UseGenUpdateState<S, Call<F>>;
    /// Note that only the initial `f` will be called.
    pub fn use_gen_call<S, F: FnMut(&mut S) -> R, R: IntoUpdateStateResult>(
        initial_state: S,
        f: F,
    ) -> UseGenCall<S, F> {
        use_gen_update_state(initial_state, Call::new(f))
    }

    pub type UseGenCallWith<F> = UseGenUpdateStateWith<F>;
    pub fn use_gen_call_with<S, F: FnMut(&mut S) -> R, R: IntoUpdateStateResult>(
        f: impl FnOnce() -> (S, F),
    ) -> UseGenCallWith<impl FnOnce() -> (S, Call<F>)> {
        use_gen_update_state_with(move || {
            let (initial_state, f) = f();
            (initial_state, Call::new(f))
        })
    }

    #[cfg(test)]
    mod tests {
        use futures_lite::future::block_on;
        use hooks_core::{HookExt, IntoHook};

        use crate::{hook_fn, utils::testing::assert_always_pending};

        use super::{use_gen_call, GenCallKey};

        type GenMultiply2Key = GenCallKey<fn(&mut u32)>;
        hook_fn!(
            fn use_value() -> (u32, GenMultiply2Key) {
                let (value, call) = h![use_gen_call(1, (|v| *v *= 2) as fn(&mut u32))];
                (*value, call)
            }
        );

        #[test]
        fn call() {
            block_on(async {
                let mut hook = use_value().into_hook();
                let (value, _) = hook.next_value().await.unwrap();
                assert_eq!(value, 1);

                assert_always_pending(|| hook.next_value());

                {
                    let (value, updater) = hook.use_hook();
                    assert_eq!(value, 1);
                    updater.call();
                    assert_eq!(hook.next_value().await.unwrap().0, 2);
                }

                {
                    let (value, updater) = hook.use_hook();
                    assert_eq!(value, 2);
                    updater.call();
                    updater.call();
                    updater.call();
                    assert_eq!(hook.next_value().await.unwrap().0, 16);
                }

                assert_always_pending(|| hook.next_value());
            })
        }
    }
}
