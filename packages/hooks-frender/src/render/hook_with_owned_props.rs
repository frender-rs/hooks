use std::{any::Any, pin::Pin};

use hooks::Hook;

use super::{ContextAndState, Dom, HookStatePollOnce, RenderState, UpdateRenderState};

#[derive(Clone, Copy, Debug)]
pub struct HookElementWithOwnedProps<H, Props>(pub H, pub Props);

impl<F, H, S: RenderState + 'static, Props> UpdateRenderState<Dom>
    for HookElementWithOwnedProps<F, Props>
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
        let hook = state.hook.use_hook((self.0,));
        hook.use_hook((ContextAndState::new(ctx, state.render_state), self.1));
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
