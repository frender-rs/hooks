use std::{pin::Pin, task::Poll};

pub trait RenderState {
    fn new_uninitialized() -> Self;
    fn destroy(self: Pin<&mut Self>);

    #[inline]
    fn poll_reactive(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        let _ = cx;
        Poll::Ready(false)
    }
}

pub trait UpdateRenderState<Ctx> {
    type State: RenderState;

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>);
}

impl<Ctx, R: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for Box<R> {
    type State = R::State;

    #[inline]
    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        R::update_render_state(*self, ctx, state)
    }
}

impl<Ctx, R: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for Option<R> {
    type State = R::State;

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        if let Some(element) = self {
            element.update_render_state(ctx, state)
        } else {
            state.destroy()
        }
    }
}
