#[cfg(feature = "use_shared_reducer")]
pub use shared::*;

use super::{IntoUpdateStateResult, UpdateState};

pub trait Reduce<S, A> {
    fn reduce(&mut self, state: &mut S, action: A) -> bool;
}

impl<S, A, F: FnMut(&mut S, A) -> R, R: IntoUpdateStateResult> Reduce<S, A> for F {
    fn reduce(&mut self, state: &mut S, action: A) -> bool {
        self(state, action).into_update_state_result()
    }
}

/// A [`Reducer`] is a [`Reduce`] with a collection of actions.
pub struct Reducer<A, R, AS: Default + Extend<A> + IntoIterator<Item = A> = Vec<A>> {
    pub reduce: R,
    /// Staged actions
    pub actions: AS,
}

impl<S, A, R: Reduce<S, A>, AS> UpdateState<S> for Reducer<A, R, AS>
where
    AS: Default + Extend<A> + IntoIterator<Item = A>,
{
    fn update_state(&mut self, state: &mut S) -> bool {
        std::mem::take(&mut self.actions)
            .into_iter()
            .fold(false, |changed, action| {
                self.reduce.reduce(state, action) || changed
            })
    }
}

#[cfg(feature = "use_shared_reducer")]
mod shared {
    use super::{
        super::{
            use_shared_update_state, use_shared_update_state_with, SharedUpdateState,
            UseSharedUpdateState, UseSharedUpdateStateWith,
        },
        Reduce, Reducer,
    };

    pub type SharedReducer<A, R, AS> = SharedUpdateState<Reducer<A, R, AS>>;
    pub type SharedReduce<A, R> = SharedReducer<A, R, Vec<A>>;

    impl<A, R, AS: Default + Extend<A> + IntoIterator<Item = A>> SharedReducer<A, R, AS> {
        pub fn dispatch(&self, action: A) {
            self.map_mut_update_state(|reducer| reducer.actions.extend(Some(action)))
        }

        pub fn dispatch_actions<AA: IntoIterator<Item = A>>(&self, actions: AA) {
            self.map_mut_update_state(|reducer| reducer.actions.extend(actions))
        }
    }

    pub type UseSharedReducer<S, A, R, AS> = UseSharedUpdateState<S, Reducer<A, R, AS>>;
    pub fn use_shared_reducer<
        S,
        A,
        R: Reduce<S, A>,
        AS: Default + Extend<A> + IntoIterator<Item = A>,
    >(
        initial_state: S,
        reduce: R,
    ) -> UseSharedReducer<S, A, R, AS> {
        use_shared_update_state(
            initial_state,
            Reducer {
                reduce,
                actions: AS::default(),
            },
        )
    }

    pub type UseSharedReducerWith<S, A, R, AS, F> =
        UseSharedUpdateStateWith<S, Reducer<A, R, AS>, F>;
    pub fn use_shared_reducer_with<
        S,
        A,
        R: Reduce<S, A>,
        AS: Default + Extend<A> + IntoIterator<Item = A>,
    >(
        f: impl FnOnce() -> (S, R),
    ) -> UseSharedReducerWith<S, A, R, AS, impl FnOnce() -> (S, SharedReducer<A, R, AS>)> {
        use_shared_update_state_with(move || {
            let (initial_state, reduce) = f();
            (
                initial_state,
                Reducer {
                    reduce,
                    actions: AS::default(),
                },
            )
        })
    }

    pub type UseSharedReduce<S, A, R> = UseSharedReducer<S, A, R, Vec<A>>;
    pub fn use_shared_reduce<S, A, R: Reduce<S, A>>(
        initial_state: S,
        reduce: R,
    ) -> UseSharedReduce<S, A, R> {
        use_shared_reducer(initial_state, reduce)
    }

    pub type UseSharedReduceWith<S, A, R, F> = UseSharedReducerWith<S, A, R, Vec<A>, F>;
    pub fn use_shared_reduce_with<S, A, R: Reduce<S, A>>(
        f: impl FnOnce() -> (S, R),
    ) -> UseSharedReduceWith<S, A, R, impl FnOnce() -> (S, SharedReduce<A, R>)> {
        use_shared_reducer_with(f)
    }

    #[cfg(test)]
    mod tests {
        use futures_lite::future::block_on;
        use hooks_core::{HookExt, IntoHook};

        use crate::{hook_fn, Reduce, SharedReduce};

        type SharedMyReduce = SharedReduce<MyAction, MyReduce>;

        struct MyReduce {
            step: i32,
        }

        enum MyAction {
            Increment,
            Decrement,
        }

        impl Reduce<i32, MyAction> for MyReduce {
            fn reduce(&mut self, state: &mut i32, action: MyAction) -> bool {
                if self.step == 0 {
                    return false;
                }

                match action {
                    MyAction::Increment => *state += self.step,
                    MyAction::Decrement => *state -= self.step,
                }

                true
            }
        }

        hook_fn!(
            fn use_my_reduce(step: i32) -> (i32, &'hook SharedMyReduce) {
                let (value, reduce) = h![crate::use_shared_reduce_with(|| (0, MyReduce { step }))];
                (*value, reduce)
            }
        );

        #[test]
        fn reduce() {
            block_on(async {
                let mut hook = use_my_reduce(2).into_hook();
                let (value, _) = hook.next_value().await.unwrap();
                assert_eq!(value, 0);
                assert!(hook.next_value().await.is_none());

                {
                    let (value, reduce) = hook.use_hook();
                    assert_eq!(value, 0);

                    reduce.dispatch(MyAction::Increment);
                    assert_eq!(hook.next_value().await.unwrap().0, 2);
                }

                {
                    let (value, reduce) = hook.use_hook();
                    assert_eq!(value, 2);

                    reduce.dispatch(MyAction::Increment);
                    reduce.dispatch(MyAction::Decrement);
                    assert_eq!(hook.next_value().await.unwrap().0, 2);
                }

                {
                    let (value, reduce) = hook.use_hook();
                    assert_eq!(value, 2);

                    reduce.map_mut_update_state(|reducer| reducer.reduce.step = 1);

                    reduce.dispatch(MyAction::Increment);
                    reduce.dispatch(MyAction::Increment);
                    reduce.dispatch(MyAction::Decrement);
                    reduce.dispatch(MyAction::Increment);
                    reduce.dispatch(MyAction::Increment);
                    assert_eq!(hook.next_value().await.unwrap().0, 5);
                }

                {
                    let (value, reduce) = hook.use_hook();
                    assert_eq!(value, 5);

                    reduce.map_mut_update_state(|reducer| reducer.reduce.step = 0);

                    reduce.dispatch(MyAction::Increment);
                    assert!(hook.next_value().await.is_none());
                }

                assert!(hook.next_value().await.is_none());
            })
        }
    }
}
