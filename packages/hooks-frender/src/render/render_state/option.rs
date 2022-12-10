use std::{pin::Pin, task::Poll};

use super::{RenderState, UpdateRenderState};

pin_project_lite::pin_project! {
    pub struct OptionState<S> {
        pub mounted: bool,
        #[pin]
        pub state: S,
    }
}

impl<S: RenderState> RenderState for OptionState<S> {
    fn new_uninitialized() -> Self {
        Self {
            mounted: false,
            state: S::new_uninitialized(),
        }
    }

    fn unmount(self: Pin<&mut Self>) {
        let this = self.project();
        if *this.mounted {
            S::unmount(this.state);
            *this.mounted = false;
        }
    }

    fn poll_reactive(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        S::poll_reactive(self.project().state, cx)
    }
}

impl<Ctx, R: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for Option<R> {
    type State = R::State;

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        if let Some(element) = self {
            element.update_render_state(ctx, state);
        } else {
            R::State::unmount(state);
        }
    }
}
