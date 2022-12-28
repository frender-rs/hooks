use std::{any::Any, pin::Pin};

use futures_io::AsyncWrite;
use hooks::{Hook, HookPollNextUpdate, LazyPinnedHook};

use crate::{Dom, SsrContext};

use super::{ContextAndState, RenderState, UpdateRenderState};

#[derive(Clone, Copy, Debug)]
pub struct HookElementWithNoProps<HDom, HSsr> {
    pub with_dom: HDom,
    pub with_ssr: HSsr,
}

pin_project_lite::pin_project! {
    pub struct HookStateWithNoProps<H: HookPollNextUpdate, Ctx, S> {
        #[pin]
        hook: LazyPinnedHook<H>,
        #[pin]
        render_state: S,
        ctx: Option<Ctx>,
    }
}

impl<H, Ctx: crate::HookContext, S: RenderState + 'static> RenderState
    for HookStateWithNoProps<H, Ctx, S>
where
    H: for<'a> Hook<(ContextAndState<'a, Ctx, dyn Any>,), Value = ContextAndState<'a, Ctx, S>>,
{
    fn new_uninitialized() -> Self {
        Self {
            hook: Default::default(),
            render_state: S::new_uninitialized(),
            ctx: None,
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
                    (this.hook.pin_project_hook(), this.ctx.as_mut())
                {
                    Ctx::with_context(context, |context| {
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

impl<F2, F, H, S: RenderState + 'static> UpdateRenderState<Dom> for HookElementWithNoProps<F, F2>
where
    F: FnOnce() -> H,
    H: for<'a> Hook<(ContextAndState<'a, Dom, dyn Any>,), Value = ContextAndState<'a, Dom, S>>,
{
    type State = HookStateWithNoProps<H, Dom, S>;

    fn update_render_state(self, ctx: &mut Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        *state.ctx = Some(ctx.clone());
        let hook = state.hook.use_hook((self.with_dom,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state),));
    }
}

impl<F2, F, H, S: RenderState + 'static, W: AsyncWrite + Unpin> UpdateRenderState<SsrContext<W>>
    for HookElementWithNoProps<F, F2>
where
    F: FnOnce() -> H,
    H: for<'a> Hook<
        (ContextAndState<'a, SsrContext<W>, dyn Any>,),
        Value = ContextAndState<'a, SsrContext<W>, S>,
    >,
{
    type State = HookStateWithNoProps<H, SsrContext<W>, S>;

    fn update_render_state(self, ctx: &mut SsrContext<W>, state: Pin<&mut Self::State>) {
        let state = state.project();
        *state.ctx = Some(SsrContext {
            writer: ctx.writer.take(),
        });
        let hook = state.hook.use_hook((self.with_dom,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state),));
    }
}

pub trait FnOnceOutputElementHookWithNoProps<Ctx>: FnOnce() -> Self::Hook {
    type RenderState;
    type Hook: for<'a> Hook<
        (ContextAndState<'a, Ctx, dyn Any>,),
        Value = ContextAndState<'a, Ctx, Self::RenderState>,
    >;
}

impl<F, H, Ctx, S> FnOnceOutputElementHookWithNoProps<Ctx> for F
where
    F: FnOnce() -> H,
    H: for<'a> Hook<(ContextAndState<'a, Ctx, dyn Any>,), Value = ContextAndState<'a, Ctx, S>>,
{
    type RenderState = S;
    type Hook = H;
}
