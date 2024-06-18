#[cfg(feature = "use_shared_toggle")]
pub use shared::*;

#[cfg(feature = "use_gen_toggle")]
pub use gen::*;

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

#[cfg(feature = "use_gen_toggle")]
mod gen {
    use super::{
        super::{
            use_gen_update_state, use_gen_update_state_with, GenUpdateState, GenUpdateStateKey,
            UseGenUpdateState, UseGenUpdateStateWith,
        },
        Toggle,
    };

    pub type GenToggle = GenUpdateState<Toggle>;
    pub type GenToggleKey = GenUpdateStateKey<Toggle>;

    impl GenToggleKey {
        pub fn toggle(&self) {
            self.map_mut_update_state(|v| v.toggle())
        }
    }

    pub type UseGenToggle = UseGenUpdateState<bool, Toggle>;
    pub fn use_gen_toggle(initial_state: bool) -> UseGenToggle {
        use_gen_update_state(initial_state, Toggle::new())
    }

    pub type UseGenToggleWith<F> = UseGenUpdateStateWith<F>;
    pub fn use_gen_toggle_with(
        f: impl FnOnce() -> bool,
    ) -> UseGenToggleWith<impl FnOnce() -> (bool, Toggle)> {
        use_gen_update_state_with(move || (f(), Toggle::new()))
    }
}
