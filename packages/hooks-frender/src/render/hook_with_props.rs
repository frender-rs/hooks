use std::{any::Any, pin::Pin};

use hooks::{Hook, HookBounds, HookLifetime, HookPollNextUpdate, LazyPinned, LazyPinnedHook};

use super::{ContextAndState, Dom, RenderState, UpdateRenderState};

#[derive(Clone, Copy, Debug)]
pub struct HookElementWithProps<H, Props>(pub H, pub Props);

pin_project_lite::pin_project! {
    pub struct HookStateWithProps<H: HookPollNextUpdate, S, Props> {
        #[pin]
        hook: LazyPinnedHook<H>,
        #[pin]
        render_state: S,
        dom_and_props: Option<(Dom, Props)>,
    }
}

impl<H, S: RenderState + 'static, Props> RenderState for HookStateWithProps<H, S, Props>
where
    H: for<'a, 'props> Hook<
        (ContextAndState<'a, Dom, dyn Any>, &'props Props),
        Value = ContextAndState<'a, Dom, S>,
    >,
{
    fn new_uninitialized() -> Self {
        Self {
            hook: Default::default(),
            render_state: S::new_uninitialized(),
            dom_and_props: None,
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
                if let (Some(hook), Some((context, props))) =
                    (this.hook.pin_project_hook(), this.dom_and_props.as_mut())
                {
                    context.with_position(|context| {
                        hook.use_hook((ContextAndState::new(context, this.render_state), props));
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

impl<F, H, S: RenderState + 'static, Props> UpdateRenderState<Dom>
    for HookElementWithProps<F, Props>
where
    F: FnOnce() -> H,
    H: for<'a, 'props> Hook<
        (ContextAndState<'a, Dom, dyn Any>, &'props Props),
        Value = ContextAndState<'a, Dom, S>,
    >,
{
    type State = HookStateWithProps<H, S, Props>;

    fn update_render_state(self, ctx: &mut Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        let (dom, props) = state.dom_and_props.insert((ctx.clone(), self.1));
        let hook = state.hook.use_hook((self.0,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), props));
    }
}

pub trait FnOnceOutputElementHookWithProps<Ctx, Props>: FnOnce() -> Self::Hook {
    type RenderState;
    type Hook: for<'a, 'props> Hook<
        (ContextAndState<'a, Ctx, dyn Any>, &'props Props),
        Value = ContextAndState<'a, Ctx, Self::RenderState>,
    >;
}

impl<F, H, Ctx, S, Props> FnOnceOutputElementHookWithProps<Ctx, Props> for F
where
    F: FnOnce() -> H,
    H: for<'a, 'props> Hook<
        (ContextAndState<'a, Ctx, dyn Any>, &'props Props),
        Value = ContextAndState<'a, Ctx, S>,
    >,
{
    type RenderState = S;
    type Hook = H;
}
