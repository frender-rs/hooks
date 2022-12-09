use std::{future::Future, pin::Pin};

use hooks::{Hook, HookExt, HookPollNextUpdateExt};

use crate::dom::{AnyContext, Context, UpdateRenderState, RenderState};

pin_project_lite::pin_project! {
    struct Reentrant<F> {
        #[pin]
        fut: F,
        finished: bool,
    }
}

impl<F: Future<Output = ()>> Future for Reentrant<F> {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        if *this.finished {
            std::task::Poll::Ready(())
        } else {
            let res = this.fut.poll(cx);
            if res.is_ready() {
                *this.finished = true;
            }
            res
        }
    }
}

pin_project_lite::pin_project! {
    pub struct HookElement<H> {
        #[pin]
        hook: H,
    }
}

impl<H> HookElement<H> {
    #[inline]
    pub fn new(hook: H) -> Self {
        Self { hook }
    }

    pub fn into_render<R: 'static + RenderState>(
        mut self,
        document: web_sys::Document,
        current_parent: web_sys::Node,
        destroy: impl Future<Output = ()>,
    ) -> impl Future<Output = ()>
    where
        Self: Unpin,
        R: Unpin,
        H: for<'c> Hook<(AnyContext<'c>,), Value = Context<'c, R>>,
    {
        let mut current_rendered = R::new_uninitialized();

        self.use_hook((Context {
            document: &document,
            current_parent: &current_parent,
            current_rendered: Pin::new(&mut current_rendered),
        },));

        async move {
            let destroy = Reentrant {
                fut: destroy,
                finished: false,
            };

            crate::pin!(destroy);
            crate::pin!(current_rendered);

            futures_lite::future::or(
                async {
                    while self
                        .next_value((Context {
                            document: &document,
                            current_parent: &current_parent,
                            current_rendered: current_rendered.as_mut(),
                        },))
                        .await
                        .is_some()
                    {
                        web_sys::console::log_1(&"re-render".into());
                    }
                },
                destroy.as_mut(),
            )
            .await;

            web_sys::console::log_1(&"non-dynamic or should destroy".into());
            destroy.await;
            R::destroy(current_rendered);
            web_sys::console::log_1(&"destroyed".into());
        }
    }
}

hooks::impl_hook! {
    impl [H, R: 'static] for HookElement<H>
        where [ H: for<'c> Hook<(AnyContext<'c>,), Value = Context<'c, R>> ]
    {
        #[inline]
        poll_next_update(self, cx) {
            self.project().hook.poll_next_update(cx)
        }
        #[inline]
        use_hook{'ctx}(self, ctx: Context<'ctx, R>) -> () {
            self.project().hook.use_hook((ctx.into_any(),));
        }
    }
}

pin_project_lite::pin_project! {
    pub struct HookRendered<H: Unpin, R> {
        hook: Option<HookElement<H>>,
        #[pin]
        rendered: R,
    }
}

impl<H, R> RenderState for HookRendered<H, R>
where
    H: Unpin + for<'ctx> Hook<(AnyContext<'ctx>,), Value = Context<'ctx, R>>,
    R: RenderState + 'static,
{
    fn new_uninitialized() -> Self {
        Self {
            hook: None,
            rendered: R::new_uninitialized(),
        }
    }

    fn destroy(self: Pin<&mut Self>) {
        let this = self.project();
        R::destroy(this.rendered);
        this.hook.take();
    }

    fn poll_render(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let this = self.project();
        let p1 = {
            if let Some(hook) = this.hook {
                hook.poll_next_update(cx)
            } else {
                true.into()
            }
        };
        let p2 = R::poll_render(this.rendered, cx);

        match (p1, p2) {
            (std::task::Poll::Ready(false), std::task::Poll::Ready(false)) => false.into(),
            (std::task::Poll::Pending, std::task::Poll::Pending) => std::task::Poll::Pending,
            _ => true.into(),
        }
    }
}

pub struct HookFn<F>(pub F);

impl<H, F, R> UpdateRenderState for HookFn<F>
where
    F: FnOnce() -> HookElement<H>,
    R: RenderState + 'static,
    H: Unpin + for<'ctx> Hook<(AnyContext<'ctx>,), Value = Context<'ctx, R>>,
{
    type State = HookRendered<H, R>;

    fn update_render_state(self, ctx: Context<Self::State>) {
        let this = ctx.current_rendered.project();
        let hook = this.hook.get_or_insert_with(self.0);

        Pin::new(hook).use_hook((Context {
            document: ctx.document,
            current_parent: ctx.current_parent,
            current_rendered: this.rendered,
        },));
    }
}
