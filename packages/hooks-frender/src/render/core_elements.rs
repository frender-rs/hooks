use super::{RenderState, UpdateRenderState};

pub struct CachedState<S> {
    pub state: S,
    pub cached_state: Option<S>,
}

impl<S: RenderState + Unpin> RenderState for CachedState<S> {
    #[inline]
    fn new_uninitialized() -> Self {
        CachedState {
            state: S::new_uninitialized(),
            cached_state: None,
        }
    }

    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>) {
        let this = self.get_mut();
        let old_state = std::mem::replace(&mut this.state, S::new_uninitialized());
        this.cached_state = Some(old_state);
    }
}

pub struct Cached<E>(pub E);

impl<E, Ctx> UpdateRenderState<Ctx> for Cached<E>
where
    E: UpdateRenderState<Ctx>,
    E::State: Unpin,
{
    type State = CachedState<E::State>;

    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        let state = state.get_mut();
    }
}
