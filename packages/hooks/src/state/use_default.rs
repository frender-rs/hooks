use std::marker::PhantomData;

use hooks_core::Hook;

use super::{
    PollNextUpdateFromStateUpdater, State, StateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT,
};

pub struct UseStateDefault<
    'a,
    T: PollNextUpdateFromStateUpdater<EQ> + Default,
    const N: usize,
    const EQ: bool,
>(PhantomData<(&'a (), T)>);

hooks_core::impl_hook![
    type For<'a, T: PollNextUpdateFromStateUpdater<EQ> + Default, const N: usize, const EQ: bool> =
        UseStateDefault<'a, T, N, EQ>;

    #[inline]
    fn into_hook(self) -> State<'a, T, N, EQ> {
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
) -> UseStateDefault<'a, T, STAGING_STATES_DEFAULT_STACK_COUNT, false> {
    UseStateDefault(PhantomData)
}

#[inline(always)]
pub fn use_state_default_n<'a, T: Default, const N: usize>() -> UseStateDefault<'a, T, N, false> {
    UseStateDefault(PhantomData)
}

#[inline(always)]
pub fn use_state_eq_default<'a, T: Default + PartialEq>(
) -> UseStateDefault<'a, T, STAGING_STATES_DEFAULT_STACK_COUNT, true> {
    UseStateDefault(PhantomData)
}

#[inline(always)]
pub fn use_state_eq_default_n<'a, T: Default + PartialEq, const N: usize>(
) -> UseStateDefault<'a, T, N, true> {
    UseStateDefault(PhantomData)
}
