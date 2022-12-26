use std::{any::Any, pin::Pin};

use hooks::{Hook, HookPollNextUpdate, LazyPinnedHook};

use super::{ContextAndState, Dom, RenderState, UpdateRenderState};

#[derive(Clone, Copy, Debug)]
pub struct HookElementWithRefProps<H, Props>(pub H, pub Props);

#[derive(Clone, Copy, Debug)]
pub struct HookElementPollTillEnd<E>(E);

pin_project_lite::pin_project! {
    pub struct HookStateWithRefProps<H: HookPollNextUpdate, S, Props> {
        #[pin]
        hook: LazyPinnedHook<H>,
        #[pin]
        render_state: S,
        dom_and_props: Option<(Dom, Props)>,
    }
}

pin_project_lite::pin_project! {
    pub struct HookStatePollOnce<H: HookPollNextUpdate, S> {
        #[pin]
        hook: LazyPinnedHook<H>,
        #[pin]
        render_state: S,
    }
}

impl<H, S: RenderState + 'static, Props> RenderState for HookStateWithRefProps<H, S, Props>
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

impl<H, S: RenderState + 'static> RenderState for HookStatePollOnce<H, S>
where
    H: HookPollNextUpdate,
{
    fn new_uninitialized() -> Self {
        Self {
            hook: Default::default(),
            render_state: S::new_uninitialized(),
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
            _ => std::task::Poll::Ready(true),
        }
    }
}

impl<F, H, S: RenderState + 'static, Props> UpdateRenderState<Dom>
    for HookElementPollTillEnd<HookElementWithRefProps<F, Props>>
where
    F: FnOnce() -> H,
    H: for<'a, 'props> Hook<
        (ContextAndState<'a, Dom, dyn Any>, &'props Props),
        Value = ContextAndState<'a, Dom, S>,
    >,
{
    type State = HookStateWithRefProps<H, S, Props>;

    fn update_render_state(self, ctx: &mut Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        let (_, props) = state.dom_and_props.insert((ctx.clone(), self.0 .1));
        let hook = state.hook.use_hook((self.0 .0,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), props));
    }
}

impl<F, H, S: RenderState + 'static, Props> UpdateRenderState<Dom>
    for HookElementWithRefProps<F, Props>
where
    F: FnOnce() -> H,
    H: for<'a, 'props> Hook<
        (ContextAndState<'a, Dom, dyn Any>, &'props Props),
        Value = ContextAndState<'a, Dom, S>,
    >,
{
    type State = HookStatePollOnce<H, S>;

    fn update_render_state(self, ctx: &mut Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        let hook = state.hook.use_hook((self.0,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), &self.1));
    }
}

pub trait FnOnceOutputElementHookWithRefProps<Ctx, Props>: FnOnce() -> Self::Hook {
    type RenderState;
    type Hook: for<'a, 'props> Hook<
        (ContextAndState<'a, Ctx, dyn Any>, &'props Props),
        Value = ContextAndState<'a, Ctx, Self::RenderState>,
    >;
}

impl<F, H, Ctx, S, Props> FnOnceOutputElementHookWithRefProps<Ctx, Props> for F
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
