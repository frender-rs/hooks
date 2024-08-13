pub use {
    UseGenUpdateState as use_gen_update_state, UseGenUpdateStateWith as use_gen_update_state_with,
};

use std::{cell::RefCell, pin::Pin, task::Poll};

use hooks_gen::{local, Key, Owner, Store};

use super::{
    super::{StateUpdater, StateWithUpdater, UseStateWithUpdater},
    UpdateState, WakerAndUpdater,
};

#[derive(Debug)]
pub struct StateWithGenUpdateState<S, U: 'static + UpdateState<S>>(
    StateWithUpdater<S, GenUpdateStateOwner<U>>,
);

hooks_core::impl_hook!(
    impl<S, U: 'static + UpdateState<S>> StateWithGenUpdateState<S, U> {
        fn unmount(self) {
            Pin::new(&mut self.get_mut().0).unmount()
        }

        fn poll_next_update(self, cx: _) {
            Pin::new(&mut self.get_mut().0).poll_next_update(cx)
        }

        #[inline]
        fn use_hook(self) -> (&'hook mut S, GenUpdateStateKey<U>) {
            let (state, updater) = Pin::new(&mut self.get_mut().0).use_hook();
            (state, updater.key())
        }
    }
);

#[derive(Debug)]
pub struct GenUpdateStateOwner<U: 'static>(local::Owner<RefCell<WakerAndUpdater<U>>>);

// Is this needed?
impl<U> Drop for GenUpdateStateOwner<U> {
    fn drop(&mut self) {
        self.0.key().map(|this| this.borrow_mut().wake())
    }
}

impl<U> GenUpdateStateOwner<U> {
    pub fn new(updater: U) -> Self {
        Self(local::Store.insert(RefCell::new(WakerAndUpdater {
            waker: None,
            updater,
        })))
    }

    /// The opposite of [`GenUpdateStateKey::upgrade`].
    pub fn key(&self) -> GenUpdateStateKey<U> {
        GenUpdateStateKey(self.0.key())
    }
}

#[derive(Debug)]
pub struct GenUpdateStateKey<U: 'static>(local::Key<RefCell<WakerAndUpdater<U>>>);

impl<U> Copy for GenUpdateStateKey<U> {}
impl<U> Clone for GenUpdateStateKey<U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<U> GenUpdateStateKey<U> {
    /// The opposite of [`GenUpdateState::key`].
    pub fn upgrade(self) -> GenUpdateStateOwner<U> {
        GenUpdateStateOwner(self.0.owner())
    }

    pub fn map_mut_update_state<R>(&self, f: impl FnOnce(&mut U) -> R) -> R {
        self.0.map(|this| {
            let mut this = this.borrow_mut();
            let ret = f(&mut this.updater);

            this.wake();

            ret
        })
    }

    pub fn map_mut_update_state_and_notify_if<R>(&self, f: impl FnOnce(&mut U) -> (R, bool)) -> R {
        self.0.map(|this| {
            let mut this = this.borrow_mut();
            let (ret, should_notify) = f(&mut this.updater);

            if should_notify {
                this.wake();
            }

            ret
        })
    }
}

impl<U: UpdateState<S>, S: ?Sized> StateUpdater<S> for GenUpdateStateOwner<U> {
    fn poll_update_state(&mut self, state: &mut S, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        self.0.key().map(|this| {
            let WakerAndUpdater { waker, updater } = &mut *this.borrow_mut();
            if updater.update_state(state) {
                Poll::Ready(true)
            } else {
                // further updates are possible
                *waker = Some(cx.waker().clone());
                Poll::Pending
            }
        })
    }
}

pub struct UseGenUpdateState<S, U: 'static + UpdateState<S>>(pub S, pub U);

hooks_core::impl_hook!(
    impl<S, U: UpdateState<S>> UseGenUpdateState<S, U> {
        fn into_hook(self) -> StateWithGenUpdateState<S, U> {
            StateWithGenUpdateState(
                UseStateWithUpdater(self.0, GenUpdateStateOwner::new(self.1)).into_hook(),
            )
        }

        fn update_hook(self, _hook: _) {}

        fn h(self, hook: crate::utils::UninitializedHook<StateWithGenUpdateState<S, U>>) {
            hook.get_mut().use_into_or_update_hook(self)
        }
    }
);

pub struct UseGenUpdateStateWith<F>(pub F);

hooks_core::impl_hook!(
    impl<S, U: 'static + UpdateState<S>, F: FnOnce() -> (S, U)> UseGenUpdateStateWith<F> {
        fn into_hook(self) -> StateWithGenUpdateState<S, U> {
            let (initial_value, update_state) = self.0();
            StateWithGenUpdateState(
                UseStateWithUpdater(initial_value, GenUpdateStateOwner::new(update_state))
                    .into_hook(),
            )
        }

        fn update_hook(self, _hook: _) {}

        fn h(self, hook: crate::utils::UninitializedHook<StateWithGenUpdateState<S, U>>) {
            hook.get_mut().use_into_or_update_hook(self)
        }
    }
);
