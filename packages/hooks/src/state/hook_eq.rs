use hooks_core::HookExt;

use super::{State, StateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT};

#[derive(Debug)]
pub struct StateEq<'a, T: 'a + PartialEq, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT>(
    State<'a, T, N>,
);

impl<'a, T: 'a + PartialEq, const N: usize> Default for StateEq<'a, T, N> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'a, T: 'a + PartialEq, const N: usize> Unpin for StateEq<'a, T, N> {}

pub struct StateEqWith<'a, T: 'a + PartialEq, const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT>(
    State<'a, T, N>,
);

impl<'a, T: 'a + PartialEq, const N: usize> Unpin for StateEqWith<'a, T, N> {}

crate::utils::impl_hook! {
    impl ['a, T: 'a + PartialEq, const N: usize] for StateEq<'a, T, N> {
        #[inline]
        poll_next_update(mut self, cx) {
            self.0.poll_next_update_if_not_equal(cx, PartialEq::eq)
        }

        #[inline]
        use_hook(self, initial_state: T) -> (&'hook mut T, &'hook StateUpdater<'a, T, N>) {
            self.get_mut().0.use_hook((initial_state,))
        }
    }
}

crate::utils::impl_hook! {
    impl ['a, T: 'a + PartialEq, const N: usize] for StateEqWith<'a, T, N> {
        #[inline]
        poll_next_update(mut self, cx) {
            self.0.poll_next_update_if_not_equal(cx, PartialEq::eq)
        }

        #[inline]
        use_hook[F: FnOnce() -> T](self, get_initial_state: F) -> (&'hook mut T, &'hook StateUpdater<'a, T, N>) {
            ::core::pin::Pin::new(&mut self.get_mut().0).use_hook_with(get_initial_state)
        }
    }
}

#[inline]
pub fn use_state_eq<'a, T: 'a + PartialEq>() -> StateEq<'a, T> {
    Default::default()
}

#[inline]
pub fn use_state_eq_n<'a, T: 'a + PartialEq, const N: usize>() -> StateEq<'a, T, N> {
    Default::default()
}
