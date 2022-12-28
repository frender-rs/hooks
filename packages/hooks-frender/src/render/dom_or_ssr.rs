use std::pin::Pin;

use futures_io::AsyncWrite;
use hooks::LazyPinned;

use crate::{Dom, RenderState, SsrContext, UpdateRenderState};

mod imp {
    use std::pin::Pin;

    use futures_io::AsyncWrite;

    use crate::{Dom, RenderState, SsrContext};

    pub enum DomOrSsr {
        Dom(Dom),
        Ssr(SsrContext<Pin<Box<dyn AsyncWrite>>>),
    }

    pin_project_lite::pin_project! {
        #[project = DomOrSsrStateProj]
        pub enum DomOrSsrState<S1, S2> {
            Dom {
                #[pin]
                state: S1,
            },
            Ssr {
                #[pin]
                state: S2,
            },
        }
    }

    impl<S1, S2> DomOrSsrState<S1, S2> {
        pub(super) fn project_state(
            self: std::pin::Pin<&mut Self>,
        ) -> Result<std::pin::Pin<&mut S1>, std::pin::Pin<&mut S2>> {
            match self.project() {
                DomOrSsrStateProj::Dom { state } => Ok(state),
                DomOrSsrStateProj::Ssr { state } => Err(state),
            }
        }
    }

    impl<S1: RenderState, S2: RenderState> DomOrSsrState<S1, S2> {
        pub(super) fn unmount(self: std::pin::Pin<&mut Self>) {
            match self.project() {
                DomOrSsrStateProj::Dom { state } => S1::unmount(state),
                DomOrSsrStateProj::Ssr { state } => S2::unmount(state),
            }
        }

        pub(super) fn poll_reactive(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<bool> {
            match self.project() {
                DomOrSsrStateProj::Dom { state } => S1::poll_reactive(state, cx),
                DomOrSsrStateProj::Ssr { state } => S2::poll_reactive(state, cx),
            }
        }
    }
}

pub struct DomOrSsr {
    inner: imp::DomOrSsr,
}

pub type DomOrSsrStateFor<E> = DomOrSsrState<
    <E as UpdateRenderState<Dom>>::State,
    <E as UpdateRenderState<SsrContext<Pin<Box<dyn AsyncWrite>>>>>::State,
>;

impl crate::HookContext for DomOrSsr {
    #[inline]
    fn take_context(this: &mut Self) -> Self {
        let inner = match &mut this.inner {
            imp::DomOrSsr::Dom(ctx) => imp::DomOrSsr::Dom(crate::HookContext::take_context(ctx)),
            imp::DomOrSsr::Ssr(ctx) => imp::DomOrSsr::Ssr(crate::HookContext::take_context(ctx)),
        };

        Self { inner }
    }

    #[inline]
    fn with_context(this: &mut Self, f: impl FnOnce(&mut Self)) {
        todo!();
        // match &mut this.inner {
        //     imp::DomOrSsr::Dom(ctx) => crate::HookContext::with_context(ctx, |context| {}),
        //     imp::DomOrSsr::Ssr(ctx) => imp::DomOrSsr::Ssr(crate::HookContext::take_context(ctx)),
        // };
    }
}

impl DomOrSsr {
    pub fn update_render_state<
        E: UpdateRenderState<Dom> + UpdateRenderState<SsrContext<Pin<Box<dyn AsyncWrite>>>>,
    >(
        &mut self,
        element: E,
        state: Pin<&mut DomOrSsrStateFor<E>>,
    ) {
        let state = state
            .project()
            .inner
            .get_pin_or_insert_with(|| match &self.inner {
                imp::DomOrSsr::Dom(_) => imp::DomOrSsrState::Dom {
                    state: RenderState::new_uninitialized(),
                },
                imp::DomOrSsr::Ssr(_) => imp::DomOrSsrState::Ssr {
                    state: RenderState::new_uninitialized(),
                },
            })
            .project_state();

        match &mut self.inner {
            imp::DomOrSsr::Dom(ctx) => {
                let state = state
                    .ok()
                    .expect("DomOrSsrState should be initialized as DomState");

                element.update_render_state(ctx, state)
            }
            imp::DomOrSsr::Ssr(ctx) => {
                let state = state
                    .err()
                    .expect("DomOrSsrState should be initialized as SsrState");

                element.update_render_state(ctx, state)
            }
        }
    }
}

pin_project_lite::pin_project! {
    pub struct DomOrSsrState<S1, S2> {
        #[pin]
        inner: LazyPinned<imp::DomOrSsrState<S1, S2>>,
    }
}

impl<S1: RenderState, S2: RenderState> RenderState for DomOrSsrState<S1, S2> {
    #[inline]
    fn new_uninitialized() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>) {
        if let Some(state) = self.project().inner.as_pin_mut() {
            state.unmount()
        }
    }

    #[inline]
    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        if let Some(state) = self.project().inner.as_pin_mut() {
            state.poll_reactive(cx)
        } else {
            std::task::Poll::Ready(true)
        }
    }
}

#[macro_export]
macro_rules! impl_render_dom_or_ssr {
    ($([$($generics:tt)*])? for $for_ty:ty $(where $($where_clause:tt)* )?) => {
        impl $(<$($generics)*>)? $crate::UpdateRenderState<$crate::DomOrSsr> for $for_ty
        $(where $($where_clause)* )?
        {
            type State = $crate::DomOrSsrStateFor<Self>;

            #[inline]
            fn update_render_state(
                self,
                ctx: &mut $crate::DomOrSsr,
                state: ::core::pin::Pin<&mut Self::State>,
            ) {
                ctx.update_render_state(self, state)
            }
        }
    };
}
