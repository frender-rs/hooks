use std::{pin::Pin, task::Poll};

use crate::{StateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT};

#[derive(Debug)]
struct StateInner<'a, T, const N: usize> {
    current_state: T,
    state_updater: StateUpdater<'a, T, N>,
}

impl<'a, T, const N: usize> Unpin for State<'a, T, N> {}

#[derive(Debug)]
pub struct State<'a, T, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT> {
    data: Option<StateInner<'a, T, N>>,
}

impl<'a, T, const N: usize> Default for State<'a, T, N> {
    #[inline]
    fn default() -> Self {
        Self { data: None }
    }
}

impl<'a, T, const N: usize> State<'a, T, N> {
    /// If `compare` returns true,
    /// which indicates the old and new values are equal,
    /// the polling will keep pending.
    pub fn poll_next_update_if_not_equal(
        &mut self,
        cx: &mut std::task::Context<'_>,
        compare: impl FnMut(&T, &T) -> bool,
    ) -> Poll<bool> {
        if let Some(data) = &mut self.data {
            data.state_updater
                .map_mut(|(waker, staging_states), rc_status| {
                    if staging_states.is_empty() {
                        match rc_status {
                            crate::utils::RcStatus::Shared => {
                                // further updates are possible
                                *waker = Some(cx.waker().clone());
                                Poll::Pending
                            }
                            crate::utils::RcStatus::Owned => {
                                // no further updates
                                Poll::Ready(false)
                            }
                        }
                    } else {
                        let is_equal =
                            staging_states.drain_into_and_compare(&mut data.current_state, compare);

                        if is_equal {
                            Poll::Pending
                        } else {
                            Poll::Ready(true)
                        }
                    }
                })
        } else {
            Poll::Ready(true)
        }
    }

    pub fn poll_next_update_always_not_equal(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<bool> {
        if let Some(data) = &mut self.data {
            data.state_updater
                .map_mut(|(waker, staging_states), rc_status| {
                    let not_changed = staging_states.drain_into(&mut data.current_state);

                    if not_changed {
                        match rc_status {
                            crate::utils::RcStatus::Shared => {
                                // further updates are possible
                                *waker = Some(cx.waker().clone());
                                Poll::Pending
                            }
                            crate::utils::RcStatus::Owned => {
                                // no further updates
                                Poll::Ready(false)
                            }
                        }
                    } else {
                        Poll::Ready(true)
                    }
                })
        } else {
            Poll::Ready(true)
        }
    }

    pub fn use_hook_with(
        self: Pin<&mut Self>,
        get_initial_state: impl FnOnce() -> T,
    ) -> (&mut T, &StateUpdater<'a, T, N>) {
        let data = &mut self.get_mut().data;

        let data = data.get_or_insert_with(|| StateInner {
            current_state: get_initial_state(),
            state_updater: Default::default(),
        });

        (&mut data.current_state, &data.state_updater)
    }
}

crate::utils::impl_hook! {
    impl ['a, T, const N: usize] for State<'a, T, N> {
        #[inline]
        poll_next_update(mut self, cx) {
            self.poll_next_update_always_not_equal(cx)
        }

        #[inline]
        use_hook(self, initial_state: T) -> (&'hook mut T, &'hook StateUpdater<'a, T, N>) {
            self.use_hook_with(|| initial_state)
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

#[cfg(test)]
mod tests {
    use futures_lite::StreamExt;
    use hooks_core::AsyncIterableHook;
    use hooks_derive::hook;

    use crate::{use_effect, use_state, use_state_with};

    #[test]
    fn state_2() {
        #[hook(hooks_core_path = "::hooks_core")]
        fn use_state_2() -> (i32, i32) {
            let (state_1, updater_1) = use_state(1);
            let (state_2, updater_2) = use_state_with(|| *state_1 + 1);

            let ret = (*state_1, *state_2);

            let updater_1 = updater_1.clone();
            let updater_2 = updater_2.clone();
            use_effect(
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
            let values = use_state_2().into_iter().collect::<Vec<_>>().await;

            assert_eq!(values, [(1, 2), (2, 3), (3, 5), (5, 8), (8, 13)]);
        });
    }
}
