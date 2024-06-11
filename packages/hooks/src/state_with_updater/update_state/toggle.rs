#[cfg(feature = "use_shared_toggle")]
pub use shared::*;

use super::UpdateState;

#[derive(Debug, Default)]
pub struct Toggle {
    pub toggled: bool,
}

impl Toggle {
    pub const fn new() -> Self {
        Self { toggled: false }
    }

    pub fn toggle(&mut self) {
        self.toggled = !self.toggled;
    }
}

impl UpdateState<bool> for Toggle {
    fn update_state(&mut self, state: &mut bool) -> bool {
        if self.toggled {
            self.toggled = false;
            *state = !*state;
            true
        } else {
            false
        }
    }
}

#[cfg(feature = "use_shared_toggle")]
mod shared {
    use super::{
        super::{
            use_shared_update_state, use_shared_update_state_with, SharedUpdateState,
            UseSharedUpdateState, UseSharedUpdateStateWith,
        },
        Toggle,
    };

    pub type SharedToggle = SharedUpdateState<Toggle>;

    impl SharedToggle {
        pub fn toggle(&self) {
            self.map_mut_update_state(|v| v.toggle())
        }
    }

    pub type UseSharedToggle = UseSharedUpdateState<bool, Toggle>;
    pub fn use_shared_toggle(initial_state: bool) -> UseSharedToggle {
        use_shared_update_state(initial_state, Toggle::new())
    }

    pub type UseSharedToggleWith<F> = UseSharedUpdateStateWith<bool, Toggle, F>;
    pub fn use_shared_toggle_with(
        f: impl FnOnce() -> bool,
    ) -> UseSharedToggleWith<impl FnOnce() -> (bool, SharedToggle)> {
        use_shared_update_state_with(move || (f(), Toggle::new()))
    }
}
