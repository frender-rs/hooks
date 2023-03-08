use std::marker::PhantomData;

use super::{State, StateUninitialized, StateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT};

pub struct UseStateEq<'a, T, const N: usize>(T, PhantomData<&'a ()>);

hooks_core::impl_hook![
    type For<'a, T: PartialEq, const N: usize> = UseStateEq<'a, T, N>;

    #[inline]
    fn into_hook(self) -> State<'a, T, N, true> {
        State::new(self.0)
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}

    fn h(self, hook: StateUninitialized<'a, T, N, true>) -> (&mut T, &StateUpdater<'a, T, N>) {
        let hook = hook.get_mut();
        (
            hook.current_state.get_or_insert(self.0),
            &hook.state_updater,
        )
    }
];

#[inline(always)]
pub fn use_state_eq<'a, T>(
    initial_value: T,
) -> UseStateEq<'a, T, STAGING_STATES_DEFAULT_STACK_COUNT> {
    UseStateEq(initial_value, PhantomData)
}

#[inline(always)]
pub fn use_state_eq_n<'a, T, const N: usize>(initial_value: T) -> UseStateEq<'a, T, N> {
    UseStateEq(initial_value, PhantomData)
}

pub struct UseStateEqWith<'a, F, const N: usize>(F, PhantomData<&'a ()>);

hooks_core::impl_hook![
    type For<'a, T: PartialEq, F, const N: usize> = UseStateEqWith<'a, F, N>
        where __![F: FnOnce() -> T]: __;

    #[inline]
    fn into_hook(self) -> State<'a, T, N, true> {
        State::new(self.0())
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}

    fn h(self, hook: StateUninitialized<'a, T, N, true>) -> (&mut T, &StateUpdater<'a, T, N>) {
        let hook = hook.get_mut();
        (
            hook.current_state.get_or_insert_with(self.0),
            &hook.state_updater,
        )
    }
];

#[inline(always)]
pub fn use_state_eq_with<'a, T>(
    get_initial_value: impl FnOnce() -> T,
) -> UseStateEqWith<'a, impl FnOnce() -> T, STAGING_STATES_DEFAULT_STACK_COUNT> {
    UseStateEqWith(get_initial_value, PhantomData)
}

#[inline(always)]
pub fn use_state_eq_n_with<'a, T, const N: usize>(
    get_initial_value: impl FnOnce() -> T,
) -> UseStateEqWith<'a, impl FnOnce() -> T, N> {
    UseStateEqWith(get_initial_value, PhantomData)
}
