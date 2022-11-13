use hooks_core::HookPollNextUpdateExt;

use crate::{use_state, use_state_n, State, StateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT};

#[derive(Debug)]
pub struct StateWith<'a, T, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT> {
    inner: State<'a, T, N>,
}

impl<'a, T, const N: usize> Default for StateWith<'a, T, N> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<'a, T, const N: usize> Unpin for StateWith<'a, T, N> {}

crate::utils::impl_hook! {
    impl ['a, T, const N: usize] for StateWith<'a, T, N> {
        #[inline]
        poll_next_update(self, cx) {
            self.get_mut().inner.poll_next_update(cx)
        }

        #[inline]
        use_hook[F: FnOnce() -> T](self, get_initial_state: F) -> (&'hook mut T, &'hook StateUpdater<'a, T, N>) {
            ::core::pin::Pin::new(&mut self.get_mut().inner).use_hook_with(get_initial_state)
        }
    }
}

/// [`use_state`](crate::use_state) with a lazy initializer.
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
#[inline]
pub fn use_state_with<'a, T>() -> StateWith<'a, T> {
    StateWith { inner: use_state() }
}

/// [`use_state_n`](crate::use_state_n) with a lazy initializer.
#[inline]
pub fn use_state_n_with<'a, T, const N: usize>() -> StateWith<'a, T, N> {
    StateWith {
        inner: use_state_n(),
    }
}
