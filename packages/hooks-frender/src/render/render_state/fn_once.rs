use std::pin::Pin;

use super::UpdateRenderState;

pub struct ElementFnOnce<E>(pub E);

#[cfg(aaaa)]
impl<F: FnOnce() -> E, Ctx, E: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for ElementFnOnce<F> {
    type State = Pin<Box<E::State>>;

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        self.0().update_render_state(
            ctx,
            // SAFETY: see Pin::as_deref_mut
            unsafe { state.get_unchecked_mut() }.as_mut(),
        )
    }
}
