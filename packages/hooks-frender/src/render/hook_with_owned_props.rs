use std::{any::Any, pin::Pin};

use futures_io::AsyncWrite;
use hooks::Hook;

use crate::SsrContext;

use super::{ContextAndState, Dom, HookStatePollOnce, RenderState, UpdateRenderState};

#[derive(Clone, Copy, Debug)]
pub struct HookElementWithOwnedProps<HDom, HSsr, Props> {
    pub with_dom: HDom,
    pub with_ssr: HSsr,
    pub props: Props,
}

impl<F, F2, H, S: RenderState + 'static, Props> UpdateRenderState<Dom>
    for HookElementWithOwnedProps<F, F2, Props>
where
    F: FnOnce() -> H,
    H: for<'a> Hook<
        (ContextAndState<'a, Dom, dyn Any>, Props),
        Value = ContextAndState<'a, Dom, S>,
    >,
{
    type State = HookStatePollOnce<H, S>;

    fn update_render_state(self, ctx: &mut Dom, state: Pin<&mut Self::State>) {
        let state = state.project();
        let hook = state.hook.use_hook((self.with_dom,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), self.props));
    }
}

impl<F, F1, H, S: RenderState + 'static, Props, W: AsyncWrite + Unpin>
    UpdateRenderState<SsrContext<W>> for HookElementWithOwnedProps<F1, F, Props>
where
    F: FnOnce() -> H,
    H: for<'a> Hook<
        (ContextAndState<'a, SsrContext<W>, dyn Any>, Props),
        Value = ContextAndState<'a, SsrContext<W>, S>,
    >,
{
    type State = HookStatePollOnce<H, S>;

    fn update_render_state(self, ctx: &mut SsrContext<W>, state: Pin<&mut Self::State>) {
        let state = state.project();
        let hook = state.hook.use_hook((self.with_ssr,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), self.props));
    }
}

pub trait FnOnceOutputElementHookWithOwnedProps<Ctx, Props>: FnOnce() -> Self::Hook {
    type RenderState;
    type Hook: for<'a> Hook<
        (ContextAndState<'a, Ctx, dyn Any>, Props),
        Value = ContextAndState<'a, Ctx, Self::RenderState>,
    >;
}

impl<F, H, Ctx, S, Props> FnOnceOutputElementHookWithOwnedProps<Ctx, Props> for F
where
    F: FnOnce() -> H,
    H: for<'a> Hook<
        (ContextAndState<'a, Ctx, dyn Any>, Props),
        Value = ContextAndState<'a, Ctx, S>,
    >,
{
    type RenderState = S;
    type Hook = H;
}
