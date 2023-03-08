use std::marker::PhantomData;

use hooks_core::Hook;

use super::{State, StateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT};

pub struct UseStateDefault<'a, T: Default, const N: usize>(PhantomData<(&'a (), T)>);

hooks_core::impl_hook![
    type For<'a, T: Default, const N: usize> = UseStateDefault<'a, T, N>;
    #[inline]
    fn into_hook(self) -> State<'a, T, N, false> {
        State::default()
    }
    #[inline(always)]
    fn update_hook(self, _hook: _) {}

    #[inline]
    fn h(self, hook: Self::Hook) -> (&mut T, &StateUpdater<'a, T, N>) {
        hook.use_hook()
    }
];

#[inline(always)]
pub fn use_state_default<'a, T: Default>(
) -> UseStateDefault<'a, T, STAGING_STATES_DEFAULT_STACK_COUNT> {
    UseStateDefault(PhantomData)
}

#[inline(always)]
pub fn use_state_default_n<'a, T: Default, const N: usize>() -> UseStateDefault<'a, T, N> {
    UseStateDefault(PhantomData)
}

#[inline(always)]
pub fn use_state_eq_default<'a, T: Default>(
) -> UseStateEqDefault<'a, T, STAGING_STATES_DEFAULT_STACK_COUNT> {
    UseStateEqDefault(PhantomData)
}

#[inline(always)]
pub fn use_state_eq_default_n<'a, T: Default, const N: usize>() -> UseStateEqDefault<'a, T, N> {
    UseStateEqDefault(PhantomData)
}
