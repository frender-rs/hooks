use std::{marker::PhantomData, pin::Pin};

use hooks::{Hook, HookExt, HookPollNextUpdate};

use crate::{
    dom::{AnyContext, Context, UpdateRenderState, RenderState},
    HookElement,
};

pin_project_lite::pin_project! {
    pub struct HookComponent<H> {
        #[pin]
        hook: HookElement<H>,
    }
}

impl<H: for<'ctx> Hook<(AnyContext<'ctx>,), Value = Context<'ctx, R>>, R> HookComponent<H> {
    #[inline]
    pub fn new(hook: H) -> Self {
        Self {
            hook: HookElement::new(hook),
        }
    }

    #[inline]
    pub async fn into_render(self, document: &web_sys::Document, current_parent: &web_sys::Node)
    where
        R: RenderState + 'static,
    {
        let this = self;
        futures_lite::pin!(this);

        let current_rendered = R::new_uninitialized();

        futures_lite::pin!(current_rendered);

        this.as_mut().use_hook(()).update_render_state(Context {
            document,
            current_parent,
            current_rendered: current_rendered.as_mut(),
        });

        while let Some(el) = this.next_value(()).await {
            el.update_render_state(Context {
                document,
                current_parent,
                current_rendered: current_rendered.as_mut(),
            })
        }
    }
}

hooks::impl_hook! {
    impl [H: for<'ctx> Hook<(AnyContext<'ctx>,), Value = Context<'ctx, R>>, R: 'static] for HookComponent<H> {
        poll_next_update(self, cx) {
            self.project().hook.poll_next_update(cx)
        }
        use_hook(self) -> HookComp<'hook, HookElement<H>, R> {
            let this = self.project();
            HookComp::new(this.hook)
        }
    }
}

pub struct HookComp<'a, H, R> {
    inner: Pin<&'a mut H>,
    _rendered: PhantomData<R>,
}

impl<'a, H, R> HookComp<'a, H, R> {
    pub fn new(hook: Pin<&'a mut H>) -> Self {
        Self {
            inner: hook,
            _rendered: PhantomData,
        }
    }
}

impl<'a, H, R: RenderState> UpdateRenderState for HookComp<'a, H, R>
where
    H: for<'ctx> Hook<(Context<'ctx, R>,)>,
{
    type State = R;

    fn update_render_state(self, ctx: Context<Self::State>) {
        self.inner.use_hook((ctx,));
    }
}

// pub trait RenderHookImpl:
//     for<'ctx> Hook<(AnyContext<'ctx>,), Value = Context<'ctx, Self::Rendered>>
// {
//     type Rendered: Rendered;
// }

// impl<H, R: Rendered> RenderHookImpl for H
// where
//     H: for<'ctx> Hook<(AnyContext<'ctx>,), Value = Context<'ctx, R>>,
// {
//     type Rendered = R;
// }
