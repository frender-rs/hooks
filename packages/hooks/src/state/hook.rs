use std::{pin::Pin, task::Poll};

use crate::{StateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT};

#[derive(Debug, Default)]
struct StateInner<'a, T, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT> {
    current_state: T,
    state_updater: StateUpdater<'a, T, N>,
}

impl<'a, T, const N: usize> StateInner<'a, T, N> {
    fn new(current_state: T) -> Self {
        Self {
            current_state,
            state_updater: StateUpdater::default(),
        }
    }
}

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
    pub fn poll_next_update_if_not_equal(
        &mut self,
        cx: &mut std::task::Context<'_>,
        compare: impl FnMut(&T, &T) -> bool,
    ) -> Poll<bool> {
        if let Some(data) = &mut self.data {
            data.state_updater
                .poll_next_update_if_not_equal(&mut data.current_state, compare, cx)
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
                .poll_next_update_always_not_equal(&mut data.current_state, cx)
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

pub mod v2 {
    use super::StateInner;

    impl<'a, T, const N: usize> Unpin for StateInner<'a, T, N> {}

    // hooks_core::v2::v2_impl_hook!(
    //     const _: for<'a> StateInner<T>;
    //     // fn this<> ()-> ;
    //     fn aaa(self) {
    //         let a = 1;
    //     }
    // );
    // fn poll_next_update(mut self, cx) {
    //                 // true.into()
    //             }

    //    fn use_value(self) -> (&'hook mut T, &'hook crate::StateUpdater<'a,T,N>) {
    //                 // let this = self.get_mut();
    //                 // (&mut  this.current_state, &this.state_updater)
    //             }
    // for<'a, T, const N: usize>
    hooks_core::v2::v2_impl_hook!(
        const _: StateInner<'a, T, N> = Generics!['a, T, const N: usize];
        fn poll_next_update(self, cx: _) {
            let this = self.get_mut();
            this.state_updater
                .poll_next_update_always_not_equal(&mut this.current_state, cx)
        }
        fn use_value(self) -> (&'hook mut T, &'hook crate::StateUpdater<'a, T, N>) {
            let this = self.get_mut();
            (&mut this.current_state, &this.state_updater)
        }
    );
}

#[cfg(test)]
mod tests {
    use futures_lite::StreamExt;
    use hooks_core::{fn_hook, v2::Hook, AsyncIterableHook, HookPollNextUpdateExt};
    use hooks_derive::hook;

    use crate::{state::hook::StateInner, use_effect, use_state, use_state_with};

    #[test]
    fn v2() {
        futures_lite::future::block_on(async {
            let hook = super::StateInner::<_, 3>::new(1);
            futures_lite::pin!(hook);

            assert!(!std::future::poll_fn(|cx| hook.poll_next_update(cx)).await);

            let (state, updater) = hook.as_mut().use_value();
            assert_eq!(*state, 1);
            updater.set(2);
            assert_eq!(*state, 1);

            assert!(std::future::poll_fn(|cx| hook.poll_next_update(cx)).await);
            let (state, _updater) = hook.as_mut().use_value();
            assert_eq!(*state, 2);

            assert!(!std::future::poll_fn(|cx| hook.poll_next_update(cx)).await);
        });
    }

    #[test]
    fn state_2_v2() {
        use hooks_core::hook;

        fn use_state<'a, T>(initial_value: T) -> StateInner<'a, T> {
            StateInner::new(initial_value)
        }

        fn_hook! {
            fn use_state_2() -> (i32, i32) {
                let (state_1, updater_1) = hook!(use_state(1));
                // let (state_2, updater_2) = hook!(use_state_with(|| *state_1 + 1));

                // let ret = (*state_1, *state_2);

                // let updater_1 = updater_1.clone();
                // let updater_2 = updater_2.clone();
                // use_effect(
                //     move |(v1, v2): &_| {
                //         if *v2 > 10 {
                //             return;
                //         }
                //         updater_1.set(*v2);
                //         updater_2.set(*v1 + *v2);
                //     },
                //     ret,
                // );

                // ret
            }
        }

        futures_lite::future::block_on(async {
            let values = use_state_2().into_iter().collect::<Vec<_>>().await;

            assert_eq!(values, [(1, 2), (2, 3), (3, 5), (5, 8), (8, 13)]);
        });
    }

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
