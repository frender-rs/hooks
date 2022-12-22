use crate::{RenderState, UpdateRenderState};

pub struct Scope<T>(pub T);

pin_project_lite::pin_project! {
    #[repr(transparent)]
    pub struct ScopedState<S> {
        #[pin]
        pub state: S,
    }
}

impl<S: RenderState> RenderState for ScopedState<S> {
    #[inline]
    fn new_uninitialized() -> Self {
        Self {
            state: S::new_uninitialized(),
        }
    }

    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>) {
        S::unmount(self.project().state)
    }

    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        S::poll_reactive(self.project().state, cx)
    }
}

impl<E: UpdateRenderState<Ctx>, Ctx> UpdateRenderState<Ctx> for Scope<E> {
    type State = ScopedState<E::State>;

    #[inline]
    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        E::update_render_state(self.0, ctx, state.project().state)
    }
}
