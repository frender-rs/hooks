#[cfg(feature = "use_shared_set")]
pub use shared::*;

use super::UpdateState;

#[derive(Debug)]
pub struct Set<T> {
    pub new_value: Option<T>,
}

impl<S> Set<S> {
    pub fn set(&mut self, new_value: S) {
        self.new_value = Some(new_value)
    }
}

impl<T> Default for Set<T> {
    fn default() -> Self {
        Self { new_value: None }
    }
}

impl<T> UpdateState<T> for Set<T> {
    fn update_state(&mut self, state: &mut T) -> bool {
        if let Some(this) = self.new_value.take() {
            *state = this;
            true
        } else {
            false
        }
    }
}

#[cfg(feature = "use_shared_set")]
mod shared {
    use super::{
        super::{
            use_shared_update_state, use_shared_update_state_with, SharedUpdateState,
            UseSharedUpdateState, UseSharedUpdateStateWith,
        },
        Set,
    };

    pub type SharedSet<S> = SharedUpdateState<Set<S>>;

    impl<S> SharedSet<S> {
        pub fn set(&self, new_value: S) {
            self.map_mut_update_state(|v| v.set(new_value))
        }
    }

    pub type UseSharedSet<S> = UseSharedUpdateState<S, Set<S>>;
    pub fn use_shared_set<S>(initial_state: S) -> UseSharedSet<S> {
        use_shared_update_state(initial_state, Set::default())
    }

    pub type UseSharedSetWith<S, F> = UseSharedUpdateStateWith<S, Set<S>, F>;
    pub fn use_shared_set_with<S>(
        get_initial_state: impl FnOnce() -> S,
    ) -> UseSharedSetWith<S, impl FnOnce() -> (S, SharedSet<S>)> {
        use_shared_update_state_with(move || (get_initial_state(), Set::default()))
    }
}
