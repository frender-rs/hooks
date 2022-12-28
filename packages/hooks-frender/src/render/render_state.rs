mod either;
mod option;
mod wrappers;

use std::{ops::DerefMut, pin::Pin, task::Poll};

use crate::utils::pin_as_deref_mut;

pub trait RenderState {
    /// We are not using [`Default`] trait because
    /// [`Pin<Box<_>>`] does not impl [`Default`].
    fn new_uninitialized() -> Self;

    fn unmount(self: Pin<&mut Self>);

    #[inline]
    fn poll_reactive(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        let _ = cx;
        Poll::Ready(false)
    }
}

impl<S: RenderState> RenderState for Pin<Box<S>> {
    #[inline]
    fn new_uninitialized() -> Self {
        Box::pin(S::new_uninitialized())
    }

    #[inline]
    fn unmount(self: Pin<&mut Self>) {
        S::unmount(pin_as_deref_mut(self))
    }

    #[inline]
    fn poll_reactive(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        S::poll_reactive(pin_as_deref_mut(self), cx)
    }
}

pub trait UpdateRenderState<Ctx> {
    type State: RenderState;

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>);
}
