use std::marker::PhantomData;

use super::{State, StateUninitialized, StateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT};

pub struct UseState<'a, T, const N: usize>(T, PhantomData<&'a ()>);

hooks_core::impl_hook![
    type For<'a, T, const N: usize> = UseState<'a, T, N>;

    #[inline]
    fn into_hook(self) -> State<'a, T, N, false> {
        State::new(self.0)
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}

    fn h(self, hook: StateUninitialized<'a, T, N, false>) -> (&mut T, &StateUpdater<'a, T, N>) {
        let hook = hook.get_mut();
        (
            hook.current_state.get_or_insert(self.0),
            &hook.state_updater,
        )
    }
];

#[inline(always)]
pub fn use_state<'a, T>(initial_value: T) -> UseState<'a, T, STAGING_STATES_DEFAULT_STACK_COUNT> {
    UseState(initial_value, PhantomData)
}

#[inline(always)]
pub fn use_state_n<'a, T, const N: usize>(initial_value: T) -> UseState<'a, T, N> {
    UseState(initial_value, PhantomData)
}

pub struct UseStateWith<'a, F, const N: usize>(F, PhantomData<&'a ()>);

hooks_core::impl_hook![
    type For<'a, T, F, const N: usize> = UseStateWith<'a, F, N>
        where __![F: FnOnce() -> T]: __;

    #[inline]
    fn into_hook(self) -> State<'a, T, N, false> {
        State::new(self.0())
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}

    fn h(self, hook: StateUninitialized<'a, T, N, false>) -> (&mut T, &StateUpdater<'a, T, N>) {
        let hook = hook.get_mut();
        (
            hook.current_state.get_or_insert_with(self.0),
            &hook.state_updater,
        )
    }
];

/// [`use_state`] with a lazy initializer.
///
/// ```
/// # use hooks::{hook, HookExt, use_state_with};
/// #[hook]
/// fn use_demo_state_with() -> &'hook i32 {
///     let (state, updater) = use_state_with(|| 1);
///     if *state < 2 {
///         updater.replace_with_fn_pointer(|v| *v + 1);
///     }
///     state
/// }
///
/// let mut running_hook = use_demo_state_with();
///
/// # futures_lite::future::block_on(async {
/// assert_eq!(running_hook.next_value(()).await, Some(&1));
/// assert_eq!(running_hook.next_value(()).await, Some(&2));
/// assert_eq!(running_hook.next_value(()).await, None);
/// # })
/// ```
#[inline(always)]
pub fn use_state_with<'a, T>(
    get_initial_value: impl FnOnce() -> T,
) -> UseStateWith<'a, impl FnOnce() -> T, STAGING_STATES_DEFAULT_STACK_COUNT> {
    UseStateWith(get_initial_value, PhantomData)
}

/// [`use_state_n`] with a lazy initializer.
#[inline(always)]
pub fn use_state_n_with<'a, T, const N: usize>(
    get_initial_value: impl FnOnce() -> T,
) -> UseStateWith<'a, impl FnOnce() -> T, N> {
    UseStateWith(get_initial_value, PhantomData)
}
