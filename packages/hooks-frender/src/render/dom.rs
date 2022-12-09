use std::future::IntoFuture;

use super::{RenderState, UpdateRenderState};

#[derive(Clone)]
#[non_exhaustive]
pub struct Dom {
    pub document: web_sys::Document,
    pub current_parent: web_sys::Node,
}

impl Dom {
    pub async fn render_element<E: UpdateRenderState<Dom>>(
        &mut self,
        mut get_element: impl FnMut() -> E,
        stop: impl IntoFuture<Output = ()>,
    ) {
        let state = E::State::new_uninitialized();
        futures_lite::pin!(state);

        get_element().update_render_state(self, state.as_mut());

        let stop = crate::utils::reentrant(stop.into_future());

        futures_lite::pin!(stop);

        futures_lite::future::or(stop.as_mut(), async {
            while futures_lite::future::zip(
                std::future::poll_fn(|cx| state.as_mut().poll_reactive(cx)),
                gloo::timers::future::TimeoutFuture::new(0),
            )
            .await
            .0
            {
                web_sys::console::log_1(&"update_render_state".into());
                get_element().update_render_state(self, state.as_mut());
            }
        })
        .await;

        web_sys::console::log_1(&"stopped or non-dynamic".into());

        stop.await;

        web_sys::console::log_1(&"stopped".into());

        state.destroy();
    }
}
