use super::{StateUpdater, STAGING_STATES_DEFAULT_STACK_COUNT};

pub trait PollNextUpdateFromStateUpdater<const EQ: bool>: Sized {
    fn poll_next_update_from_state_updater<const N: usize>(
        current_state: &mut Self,
        state_updater: &mut StateUpdater<'_, Self, N>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool>;
}

impl<T> PollNextUpdateFromStateUpdater<false> for T {
    #[inline(always)]
    fn poll_next_update_from_state_updater<const N: usize>(
        current_state: &mut Self,
        state_updater: &mut StateUpdater<'_, Self, N>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        state_updater.poll_next_update_always_not_equal(current_state, cx)
    }
}

impl<T: PartialEq> PollNextUpdateFromStateUpdater<true> for T {
    #[inline(always)]
    fn poll_next_update_from_state_updater<const N: usize>(
        current_state: &mut Self,
        state_updater: &mut StateUpdater<'_, Self, N>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        state_updater.poll_next_update_if_not_equal(current_state, PartialEq::eq, cx)
    }
}

#[derive(Debug, Default)]
pub struct State<'a, T: PollNextUpdateFromStateUpdater<EQ>, const N: usize, const EQ: bool> {
    pub current_state: T,
    pub state_updater: StateUpdater<'a, T, N>,
}

impl<'a, T: PollNextUpdateFromStateUpdater<EQ>, const N: usize, const EQ: bool> Unpin
    for State<'a, T, N, EQ>
{
}

impl<'a, T: PollNextUpdateFromStateUpdater<EQ>, const N: usize, const EQ: bool>
    State<'a, T, N, EQ>
{
    pub fn new(current_state: T) -> Self {
        Self {
            current_state,
            state_updater: StateUpdater::default(),
        }
    }
}

#[derive(Debug)]
pub struct StateUninitialized<
    'a,
    T,
    const N: usize = STAGING_STATES_DEFAULT_STACK_COUNT,
    const EQ: bool = false,
> {
    pub current_state: Option<T>,
    pub state_updater: StateUpdater<'a, T, N>,
}

impl<'a, T, const N: usize, const EQ: bool> Unpin for StateUninitialized<'a, T, N, EQ> {}

impl<'a, T, const N: usize, const EQ: bool> Default for StateUninitialized<'a, T, N, EQ> {
    fn default() -> Self {
        Self {
            current_state: None,
            state_updater: Default::default(),
        }
    }
}

hooks_core::impl_hook![
    type For<'a, T: PollNextUpdateFromStateUpdater<EQ>, const N: usize, const EQ: bool> =
        State<'a, T, N, EQ>;

    fn unmount() {}

    #[inline]
    fn poll_next_update(mut self, cx: _) {
        let this = self.get_mut();
        T::poll_next_update_from_state_updater(&mut this.current_state, &mut this.state_updater, cx)
    }

    #[inline]
    fn use_hook(self) -> (&'hook mut T, &'hook StateUpdater<'a, T, N>) {
        let this = self.get_mut();
        (&mut this.current_state, &mut this.state_updater)
    }
];

hooks_core::impl_hook![
    type For<'a, T: PollNextUpdateFromStateUpdater<EQ>, const N: usize, const EQ: bool> =
        StateUninitialized<'a, T, N, EQ>;

    fn unmount() {}

    #[inline]
    fn poll_next_update(mut self, cx: _) {
        let this = self.get_mut();
        if let Some(current_state) = &mut this.current_state {
            this.state_updater
                .poll_next_update_always_not_equal(current_state, cx)
        } else {
            std::task::Poll::Ready(false)
        }
    }
];
