use std::{pin::Pin, task::Poll};

use hooks_core::{Hook, HookBounds, HookLifetime};

use crate::{DeferredStateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT};

struct StateInner<'a, T, const N: usize> {
    current_state: T,
    state_updater: DeferredStateUpdater<'a, T, N>,
}

impl<'a, T, const N: usize> Unpin for State<'a, T, N> {}

pub struct State<'a, T, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT> {
    data: Option<StateInner<'a, T, N>>,
}

impl<'a, T: 'a, const N: usize> HookBounds for State<'a, T, N> {
    type Bounds = Self;
}

impl<'hook, 'a, T: 'a, const N: usize> HookLifetime<'hook, &'hook Self> for State<'a, T, N> {
    //                                                     ^^^^^^^^^^^
    //                       Write explicitly `&'hook Self::Bounds`  |
    //                       so that impl body has implicit bounds   |
    //                       where `Self: 'hook`                     |

    type Value = (&'hook mut T, &'hook DeferredStateUpdater<'a, T, N>);
    type Args = (T,);
}

impl<'a, T: 'a, const N: usize> Hook for State<'a, T, N> {
    fn poll_next_update(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        if let Some(data) = &mut self.data {
            data.state_updater
                .map_mut(|(waker, staging_states), shared| {
                    if staging_states.is_empty() {
                        if shared {
                            // further updates are possible
                            *waker = Some(cx.waker().clone());
                            Poll::Pending
                        } else {
                            // no further updates
                            Poll::Ready(false)
                        }
                    } else {
                        Poll::Ready(true)
                    }
                })
        } else {
            Poll::Ready(true)
        }
    }

    fn use_hook<'hook>(
        self: Pin<&'hook mut Self>,
        (initial_state,): <Self as HookLifetime<'hook>>::Args,
    ) -> <Self as HookLifetime<'hook, &'hook Self>>::Value
    where
        Self: 'hook,
    {
        let this = self.get_mut();
        let data = &mut this.data;
        if let Some(data) = data {
            data.state_updater.drain_into(&mut data.current_state)
        } else {
            let state_updater = DeferredStateUpdater::new();
            let data = data.get_or_insert(StateInner {
                current_state: initial_state,
                state_updater,
            });
            (&mut data.current_state, &data.state_updater)
        }
    }
}

#[inline]
pub fn use_state<'a, T>() -> State<'a, T> {
    State { data: None }
}

#[inline]
pub fn use_state_n<'a, T, const N: usize>() -> State<'a, T, N> {
    State { data: None }
}
