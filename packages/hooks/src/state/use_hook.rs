use std::marker::PhantomData;

use super::{
    PollNextUpdateFromStateUpdater, State, StateUninitialized, StateUpdater,
    STAGING_STATES_DEFAULT_STACK_COUNT,
};

pub struct UseState<'a, T: PollNextUpdateFromStateUpdater<EQ>, const N: usize, const EQ: bool>(
    T,
    PhantomData<&'a ()>,
);

hooks_core::impl_hook![
    type For<'a, T: PollNextUpdateFromStateUpdater<EQ>, const N: usize, const EQ: bool> =
        UseState<'a, T, N, EQ>;

    #[inline]
    fn into_hook(self) -> State<'a, T, N, EQ> {
        State::new(self.0)
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}

    fn h(self, hook: StateUninitialized<'a, T, N, EQ>) -> (&mut T, &StateUpdater<'a, T, N>) {
        let hook = hook.get_mut();
        (
            hook.current_state.get_or_insert(self.0),
            &hook.state_updater,
        )
    }
];

pub struct UseStateWith<'a, F, const N: usize, const EQ: bool>(F, PhantomData<&'a ()>);

hooks_core::impl_hook![
    type For<
        'a,
        T: PollNextUpdateFromStateUpdater<EQ>,
        F: FnOnce() -> T,
        const N: usize,
        const EQ: bool,
    > = UseStateWith<'a, F, N, EQ>;

    #[inline]
    fn into_hook(self) -> State<'a, T, N, EQ> {
        State::new(self.0())
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}

    fn h(self, hook: StateUninitialized<'a, T, N, EQ>) -> (&mut T, &StateUpdater<'a, T, N>) {
        let hook = hook.get_mut();
        (
            hook.current_state.get_or_insert_with(self.0),
            &hook.state_updater,
        )
    }
];

#[inline(always)]
pub fn use_state<'a, T>(
    initial_value: T,
) -> UseState<'a, T, STAGING_STATES_DEFAULT_STACK_COUNT, false> {
    UseState(initial_value, PhantomData)
}

#[inline(always)]
pub fn use_state_n<'a, T, const N: usize>(initial_value: T) -> UseState<'a, T, N, false> {
    UseState(initial_value, PhantomData)
}

/// [`use_state`] with a lazy initializer.
///
/// ```
/// # use hooks::prelude::*;
/// hook_fn!(
///     fn use_demo_state_with() -> &'hook i32 {
///         let (state, updater) = h![use_state_with(|| 1)];
///         if *state < 2 {
///             updater.replace_with_fn_pointer(|v| *v + 1);
///         }
///         state
///     }
/// );
///
/// let mut running_hook = use_demo_state_with().into_hook();
///
/// # futures_lite::future::block_on(async {
/// assert_eq!(running_hook.next_value().await, Some(&1));
/// assert_eq!(running_hook.next_value().await, Some(&2));
/// assert_eq!(running_hook.next_value().await, None);
/// # })
/// ```
#[inline(always)]
pub fn use_state_with<'a, T>(
    get_initial_value: impl FnOnce() -> T,
) -> UseStateWith<'a, impl FnOnce() -> T, STAGING_STATES_DEFAULT_STACK_COUNT, false> {
    UseStateWith(get_initial_value, PhantomData)
}

/// [`use_state_n`] with a lazy initializer.
#[inline(always)]
pub fn use_state_n_with<'a, T, const N: usize>(
    get_initial_value: impl FnOnce() -> T,
) -> UseStateWith<'a, impl FnOnce() -> T, N, false> {
    UseStateWith(get_initial_value, PhantomData)
}

#[inline(always)]
pub fn use_state_eq<'a, T: PartialEq>(
    initial_value: T,
) -> UseState<'a, T, STAGING_STATES_DEFAULT_STACK_COUNT, true> {
    UseState(initial_value, PhantomData)
}

#[inline(always)]
pub fn use_state_eq_n<'a, T: PartialEq, const N: usize>(
    initial_value: T,
) -> UseState<'a, T, N, true> {
    UseState(initial_value, PhantomData)
}

#[inline(always)]
pub fn use_state_eq_with<'a, T: PartialEq>(
    get_initial_value: impl FnOnce() -> T,
) -> UseStateWith<'a, impl FnOnce() -> T, STAGING_STATES_DEFAULT_STACK_COUNT, true> {
    UseStateWith(get_initial_value, PhantomData)
}

#[inline(always)]
pub fn use_state_eq_n_with<'a, T: PartialEq, const N: usize>(
    get_initial_value: impl FnOnce() -> T,
) -> UseStateWith<'a, impl FnOnce() -> T, N, true> {
    UseStateWith(get_initial_value, PhantomData)
}
