use std::{any::Any, pin::Pin};

use super::UpdateRenderState;

pub struct ContextAndState<'a, Ctx, State: ?Sized> {
    context: &'a mut Ctx,
    state: Pin<&'a mut State>,
}

impl<'a, Ctx, State: ?Sized> ContextAndState<'a, Ctx, State> {
    pub fn new(context: &'a mut Ctx, state: Pin<&'a mut State>) -> Self {
        Self { context, state }
    }

    #[inline]
    pub fn render<E: UpdateRenderState<Ctx, State = State>>(mut self, element: E) -> Self {
        element.update_render_state(self.context, self.state.as_mut());

        self
    }
}

impl<'a, Ctx> ContextAndState<'a, Ctx, dyn Any> {
    pub fn downcast_state<S: Any>(self) -> Option<ContextAndState<'a, Ctx, S>> {
        let Self { context, state } = self;

        // SAFETY: get_unchecked_mut is never used to mutate state
        let state = unsafe { state.get_unchecked_mut() };
        let state = state.downcast_mut::<S>()?;
        // SAFETY: state comes from a Pin<&mut dyn Any>
        let state = unsafe { Pin::new_unchecked(state) };

        Some(ContextAndState { context, state })
    }
}
