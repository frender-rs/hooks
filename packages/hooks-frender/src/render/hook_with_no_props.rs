use std::{any::Any, pin::Pin};

use hooks::{Hook, HookPollNextUpdate, LazyPinnedHook};

use super::{ContextAndState, Dom, RenderState, UpdateRenderState};

#[derive(Clone, Copy, Debug)]
pub struct HookElementWithNoProps<H>(pub H);

pin_project_lite::pin_project! {
    pub struct HookStateWithNoProps<H: HookPollNextUpdate, S> {
        #[pin]
        hook: LazyPinnedHook<H>,
        #[pin]
        render_state: S,
        dom: Option<Dom>,
    }
}

impl<H, S: RenderState + 'static> RenderState for HookStateWithNoProps<H, S>
where
    H: for<'a> Hook<(ContextAndState<'a, Dom, dyn Any>,), Value = ContextAndState<'a, Dom, S>>,
{
    fn new_uninitialized() -> Self {
        Self {
            hook: Default::default(),
            render_state: S::new_uninitialized(),
            dom: None,
        }
    }

    fn unmount(self: Pin<&mut Self>) {
        self.project().render_state.unmount()
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let mut this = self.project();

        let res = this.hook.as_mut().poll_next_update(cx);
        let r = this.render_state.as_mut().poll_reactive(cx);

        match (res, r) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(false)) => {
                std::task::Poll::Ready(false)
            }
            (
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
                std::task::Poll::Ready(false) | std::task::Poll::Pending,
            ) => std::task::Poll::Pending,
            _ => {
                if let (Some(hook), Some(context)) =
                    (this.hook.pin_project_hook(), this.dom.as_mut())
                {
                    context.with_position(|context| {
                        hook.use_hook((ContextAndState::new(context, this.render_state),));
                    });
                    cx.waker().wake_by_ref();
                    std::task::Poll::Pending
                } else {
                    std::task::Poll::Ready(true)
                }
            }
        }
    }
}

impl<F, H, S: RenderState + 'static> UpdateRenderState<Dom> for HookElementWithNoProps<F>
where
    F: FnOnce() -> H,
    H: for<'a> Hook<(ContextAndState<'a, Dom, dyn Any>,), Value = ContextAndState<'a, Dom, S>>,
{
    type State = HookStateWithNoProps<H, S>;

    fn update_render_state(self, ctx: &mut Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        *state.dom = Some(ctx.clone());
        let hook = state.hook.use_hook((self.0,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state),));
    }
}

pub trait FnOnceOutputElementHookWithNoProps<Ctx>: FnOnce() -> Self::Hook {
    type RenderState;
    type Hook: for<'a> Hook<
        (ContextAndState<'a, Dom, dyn Any>,),
        Value = ContextAndState<'a, Dom, Self::RenderState>,
    >;
}

impl<F, H, Ctx, S> FnOnceOutputElementHookWithNoProps<Ctx> for F
where
    F: FnOnce() -> H,
    H: for<'a> Hook<(ContextAndState<'a, Dom, dyn Any>,), Value = ContextAndState<'a, Dom, S>>,
{
    type RenderState = S;
    type Hook = H;
}
