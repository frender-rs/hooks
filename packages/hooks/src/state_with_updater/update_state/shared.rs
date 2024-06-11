use std::{cell::RefCell, rc::Rc, task::Poll};

use super::{
    super::{StateUpdater, UseStateWithUpdater, UseStateWithUpdaterWith},
    UpdateState, WakerAndUpdater,
};

#[derive(Debug)]
pub struct SharedUpdateState<U>(Rc<RefCell<WakerAndUpdater<U>>>);

impl<U> Clone for SharedUpdateState<U> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<U> SharedUpdateState<U> {
    pub fn new(updater: U) -> Self {
        Self(Rc::new(RefCell::new(WakerAndUpdater {
            waker: None,
            updater,
        })))
    }

    pub fn map_mut_update_state<R>(&self, f: impl FnOnce(&mut U) -> R) -> R {
        let mut this = self.0.borrow_mut();
        let ret = f(&mut this.updater);
        if let Some(waker) = this.waker.take() {
            waker.wake()
        }

        ret
    }

    pub fn map_mut_update_state_and_notify_if<R>(&self, f: impl FnOnce(&mut U) -> (R, bool)) -> R {
        let mut this = self.0.borrow_mut();
        let (ret, should_notify) = f(&mut this.updater);

        if should_notify {
            if let Some(waker) = this.waker.take() {
                waker.wake()
            }
        }

        ret
    }
}

impl<U: UpdateState<S>, S: ?Sized> StateUpdater<S> for SharedUpdateState<U> {
    fn poll_update_state(&mut self, state: &mut S, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        crate::utils::rc_ref_cell_borrow_mut(
            &mut self.0,
            |WakerAndUpdater { waker, updater }, rc_status| {
                if updater.update_state(state) {
                    Poll::Ready(true)
                } else {
                    match rc_status {
                        crate::utils::RcStatus::Owned => {
                            // no further updates
                            Poll::Ready(false)
                        }
                        crate::utils::RcStatus::Shared => {
                            // further updates are possible
                            *waker = Some(cx.waker().clone());
                            Poll::Pending
                        }
                    }
                }
            },
        )
    }
}

pub type UseSharedUpdateState<S, U> = UseStateWithUpdater<S, SharedUpdateState<U>>;
pub fn use_shared_update_state<S, U: UpdateState<S>>(
    initial_state: S,
    update_state: U,
) -> UseSharedUpdateState<S, U> {
    UseStateWithUpdater(initial_state, SharedUpdateState::new(update_state))
}

pub type UseSharedUpdateStateWith<S, U, F> = UseStateWithUpdaterWith<S, SharedUpdateState<U>, F>;
pub fn use_shared_update_state_with<S, U: UpdateState<S>>(
    f: impl FnOnce() -> (S, U),
) -> UseSharedUpdateStateWith<S, U, impl FnOnce() -> (S, SharedUpdateState<U>)> {
    UseStateWithUpdaterWith(move || {
        let (initial_state, update_state) = f();
        (initial_state, SharedUpdateState::new(update_state))
    })
}
