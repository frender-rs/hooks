pub use state::StateWithUpdater;
pub use {
    UseStateWithUpdater as use_state_with_updater,
    UseStateWithUpdaterWith as use_state_with_updater_with,
};

#[cfg(feature = "UpdateState")]
pub use update_state::*;

use std::task::Poll;

pub trait StateUpdater<S: ?Sized> {
    /// - Returning `Poll::Ready(true)` means `state` is newly updated.
    /// - Returning `Poll::Pending` means `state` is not updated but there might be updates.
    /// - Returning `Poll::Ready(false)` means `state` is not updated and there will never be updates.
    fn poll_update_state(&mut self, state: &mut S, cx: &mut std::task::Context<'_>) -> Poll<bool>;
}

mod state {
    use super::StateUpdater;

    #[derive(Debug)]
    pub struct StateWithUpdater<T, U: StateUpdater<T>> {
        pub state: T,
        pub updater: U,
    }

    impl<T, U: StateUpdater<T>> Unpin for StateWithUpdater<T, U> {}

    impl<T, U: StateUpdater<T>> StateWithUpdater<T, U> {
        pub fn new(state: T, updater: U) -> Self {
            Self { state, updater }
        }
    }

    hooks_core::impl_hook![
        impl<T, U: StateUpdater<T>> StateWithUpdater<T, U> {
            fn unmount() {}

            #[inline]
            fn poll_next_update(mut self, cx: _) {
                let this = self.get_mut();
                this.updater.poll_update_state(&mut this.state, cx)
            }

            #[inline]
            fn use_hook(self) -> (&'hook mut T, &'hook mut U) {
                let this = self.get_mut();
                (&mut this.state, &mut this.updater)
            }
        }
    ];
}

pub struct UseStateWithUpdater<S, U: StateUpdater<S>>(
    /// initial state
    pub S,
    /// initial updater
    pub U,
);

pub struct UseStateWithUpdaterWith<S, U: StateUpdater<S>, F: FnOnce() -> (S, U)>(pub F);

hooks_core::impl_hook![
    impl<T, U: StateUpdater<T>> UseStateWithUpdater<T, U> {
        fn into_hook(self) -> StateWithUpdater<T, U> {
            StateWithUpdater::new(self.0, self.1)
        }

        fn update_hook(self, _hook: _) {}

        fn h(self, hook: crate::utils::UninitializedHook<StateWithUpdater<T, U>>) {
            hook.get_mut().use_into_or_update_hook(self)
        }
    }
];

hooks_core::impl_hook![
    impl<T, U: StateUpdater<T>, F: FnOnce() -> (T, U)> UseStateWithUpdaterWith<T, U, F> {
        fn into_hook(self) -> StateWithUpdater<T, U> {
            let (initial_state, updater) = self.0();
            StateWithUpdater::new(initial_state, updater)
        }

        fn update_hook(self, _hook: _) {}

        fn h(self, hook: crate::utils::UninitializedHook<StateWithUpdater<T, U>>) {
            hook.get_mut().use_into_or_update_hook(self)
        }
    }
];

#[cfg(feature = "UpdateState")]
mod update_state {
    #[cfg(feature = "use_shared_update_state")]
    pub use shared::*;

    #[cfg(feature = "use_gen_update_state")]
    pub use gen::*;

    #[cfg(any(feature = "use_shared_call", feature = "use_gen_call"))]
    pub use call::*;
    #[cfg(any(feature = "use_shared_reducer", feature = "use_gen_reducer"))]
    pub use reducer::*;
    #[cfg(any(feature = "use_shared_set", feature = "use_gen_set"))]
    pub use set::*;
    #[cfg(any(feature = "use_shared_toggle", feature = "use_gen_toggle"))]
    pub use toggle::*;

    pub trait UpdateState<S: ?Sized> {
        /// Returns `true` if `state` is newly updated.
        fn update_state(&mut self, state: &mut S) -> bool;
    }

    #[derive(Debug)]
    struct WakerAndUpdater<U> {
        waker: Option<std::task::Waker>,
        updater: U,
    }

    impl<U> WakerAndUpdater<U> {
        fn wake(&mut self) {
            if let Some(waker) = self.waker.take() {
                waker.wake()
            }
        }
    }

    pub trait IntoUpdateStateResult {
        fn into_update_state_result(self) -> bool;
    }

    impl IntoUpdateStateResult for bool {
        fn into_update_state_result(self) -> bool {
            self
        }
    }

    impl IntoUpdateStateResult for () {
        fn into_update_state_result(self) -> bool {
            true
        }
    }

    #[cfg(feature = "use_shared_update_state")]
    mod shared;

    #[cfg(feature = "use_gen_update_state")]
    mod gen;

    #[cfg(any(feature = "use_shared_call", feature = "use_gen_call"))]
    mod call;
    #[cfg(any(feature = "use_shared_reducer", feature = "use_gen_reducer"))]
    mod reducer;
    #[cfg(any(feature = "use_shared_set", feature = "use_gen_set"))]
    mod set;
    #[cfg(any(feature = "use_shared_toggle", feature = "use_gen_toggle"))]
    mod toggle;
}
