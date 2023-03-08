use std::{marker::PhantomData, pin::Pin, task::Poll};

use super::{StateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT};

#[derive(Debug, Default)]
pub struct State<'a, T, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT, const EQ: bool = false>
{
    pub current_state: T,
    pub state_updater: StateUpdater<'a, T, N>,
}

impl<'a, T, const N: usize, const EQ: bool> Unpin for State<'a, T, N, EQ> {}

impl<'a, T, const N: usize, const EQ: bool> State<'a, T, N, EQ> {
    pub fn new(current_state: T) -> Self {
        Self {
            current_state,
            state_updater: StateUpdater::default(),
        }
    }
}

#[derive(Debug)]
pub struct StateUninitialized<
    'a,
    T,
    const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT,
    const EQ: bool = false,
> {
    pub current_state: Option<T>,
    pub state_updater: StateUpdater<'a, T, N>,
}

impl<'a, T, const N: usize, const EQ: bool> Unpin for StateUninitialized<'a, T, N, EQ> {}

impl<'a, T, const N: usize, const EQ: bool> Default for StateUninitialized<'a, T, N, EQ> {
    fn default() -> Self {
        Self {
            current_state: None,
            state_updater: Default::default(),
        }
    }
}

hooks_core::impl_hook![
    type For<'a, T, const N: usize, const EQ: bool> = State<'a, T, N, EQ>;

    fn unmount() {}
];

hooks_core::impl_hook![
    type For<'a, T, const N: usize> = State<'a, T, N, false>;

    #[inline]
    fn poll_next_update(mut self, cx: _) {
        let this = self.get_mut();
        this.state_updater
            .poll_next_update_always_not_equal(&mut this.current_state, cx)
    }

    #[inline]
    fn use_hook(self) -> (&'hook mut T, &'hook StateUpdater<'a, T, N>) {
        let this = self.get_mut();
        (&mut this.current_state, &mut this.state_updater)
    }
];

hooks_core::impl_hook![
    type For<'a, T: PartialEq, const N: usize> = State<'a, T, N, true>;

    #[inline]
    fn poll_next_update(mut self, cx: _) {
        let this = self.get_mut();
        this.state_updater
            .poll_next_update_if_not_equal(&mut this.current_state, PartialEq::eq, cx)
    }

    #[inline]
    fn use_hook(self) -> (&'hook mut T, &'hook StateUpdater<'a, T, N>) {
        let this = self.get_mut();
        (&mut this.current_state, &mut this.state_updater)
    }
];

hooks_core::impl_hook![
    type For<'a, T, const N: usize, const EQ: bool> = StateUninitialized<'a, T, N, EQ>;

    fn unmount() {}
];

hooks_core::impl_hook![
    type For<'a, T, const N: usize> = StateUninitialized<'a, T, N, false>;

    #[inline]
    fn poll_next_update(mut self, cx: _) {
        let this = self.get_mut();
        if let Some(current_state) = &mut this.current_state {
            this.state_updater
                .poll_next_update_always_not_equal(current_state, cx)
        } else {
            std::task::Poll::Ready(false)
        }
    }
];

hooks_core::impl_hook![
    type For<'a, T: PartialEq, const N: usize> = StateUninitialized<'a, T, N, true>;

    #[inline]
    fn poll_next_update(mut self, cx: _) {
        let this = self.get_mut();
        if let Some(current_state) = &mut this.current_state {
            this.state_updater
                .poll_next_update_if_not_equal(current_state, PartialEq::eq, cx)
        } else {
            std::task::Poll::Ready(false)
        }
    }
];
