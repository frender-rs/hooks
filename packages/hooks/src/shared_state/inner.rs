use std::task::Waker;

#[derive(Debug)]
pub enum SharedState<T> {
    Uninitialized(Option<std::task::Waker>),
    Initialized(T),
}

impl<T> SharedState<T> {
    #[inline]
    pub fn get_or_init_with(&mut self, initialize: impl FnOnce(Option<Waker>) -> T) -> &mut T {
        match self {
            SharedState::Initialized(data) => data,
            SharedState::Uninitialized(waker) => {
                *self = Self::Initialized(initialize(waker.take()));
                match self {
                    SharedState::Initialized(v) => v,
                    SharedState::Uninitialized(_) => unreachable!(),
                }
            }
        }
    }

    #[inline]
    pub fn impl_poll_next_update(
        &mut self,
        cx: &mut std::task::Context,
        impl_poll: impl FnOnce(&mut T, &mut std::task::Context) -> std::task::Poll<bool>,
    ) -> std::task::Poll<bool> {
        match self {
            SharedState::Uninitialized(w) => {
                *w = Some(cx.waker().clone());
                ::core::task::Poll::Ready(true)
            }
            SharedState::Initialized(this) => impl_poll(this, cx),
        }
    }
}

impl<T> Default for SharedState<T> {
    #[inline]
    fn default() -> Self {
        Self::Uninitialized(None)
    }
}
