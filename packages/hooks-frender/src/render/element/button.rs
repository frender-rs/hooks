#![allow(non_camel_case_types)]

use bg::builder;
use futures_io::AsyncWrite;

use crate::{
    props::{events, UpdateDomEventListener},
    render::{Dom, SsrContext, UpdateRenderState},
    utils::insert_element_and_update,
};

builder! {
    pub struct ButtonProps {
        children[impl Sized],
        on_click[impl Sized],
    }
}

pub mod dom {
    use crate::render::RenderState;

    pin_project_lite::pin_project! {
        pub struct State<Children, OnClick> {
            pub mounted: bool,
            pub node: Option<web_sys::HtmlButtonElement>,
            #[pin]
            pub children: Children,
            pub on_click: OnClick,
        }
    }

    impl<Children, OnClick> RenderState for State<Children, OnClick>
    where
        Children: RenderState,
        OnClick: Default,
    {
        fn new_uninitialized() -> Self {
            Self {
                mounted: false,
                node: None,
                children: RenderState::new_uninitialized(),
                on_click: Default::default(),
            }
        }

        fn unmount(self: std::pin::Pin<&mut Self>) {
            if !self.mounted || self.node.is_none() {
                return;
            }

            let this = self.project();
            if let Some(node) = this.node {
                node.remove()
            }
        }
    }
}

pub mod ssr {
    use std::{borrow::Cow, pin::Pin, task::Poll};

    use futures_io::AsyncWrite;

    use crate::render::{RenderState, SsrWriter};

    struct StateInner<'a, W: AsyncWrite + Unpin> {
        writer: SsrWriter<'a, W>,
        owned_buf: Cow<'static, [u8]>,
        written: usize,
    }

    pub struct State<'a, W: AsyncWrite + Unpin>(Option<StateInner<'a, W>>);

    impl<'a, W: AsyncWrite + Unpin> State<'a, W> {
        #[inline]
        pub(super) fn update_render_state_with_str(
            &mut self,
            buf: impl Into<Cow<'static, str>>,
            ctx: &mut crate::render::SsrContext<'a, W>,
        ) {
            self.0 = ctx.writer.take().map(|writer| {
                let buf: Cow<str> = buf.into();

                let owned_buf = match buf {
                    Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
                    Cow::Owned(s) => Cow::Owned(s.into_bytes()),
                };

                StateInner {
                    writer,
                    owned_buf,
                    written: 0,
                }
            });
        }
    }

    fn poll_write_all<W: AsyncWrite>(
        mut writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
        written: &mut usize,
    ) -> Poll<std::io::Result<()>> {
        while *written < buf.len() {
            let buf = &buf[*written..];
            let n = futures_lite::ready!(writer.as_mut().poll_write(cx, buf))?;

            if n == 0 {
                return Poll::Ready(Err(std::io::ErrorKind::WriteZero.into()));
            }

            *written += n;
        }

        Poll::Ready(Ok(()))
    }

    impl<'a, W: AsyncWrite + Unpin> RenderState for State<'a, W> {
        fn new_uninitialized() -> Self {
            Self(None)
        }

        fn unmount(self: std::pin::Pin<&mut Self>) {
            self.get_mut().0 = None;
        }

        /// The implementation is from [`futures_lite::io::WriteAllFuture`].
        fn poll_reactive(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<bool> {
            let this = &mut self.get_mut().0;
            if let Some(StateInner {
                writer,
                owned_buf,
                written,
            }) = this
            {
                if writer.error.is_none() {
                    if let Err(err) = futures_lite::ready!(poll_write_all(
                        Pin::new(writer.writer),
                        cx,
                        owned_buf,
                        written
                    )) {
                        *writer.error = Some(err)
                    }
                }
            }

            Poll::Ready(false)
        }
    }
}

builder! {
    pub struct button(ButtonProps);

    pub use build as build_element;
}

impl<TypeDefs: ?Sized + button::Types> UpdateRenderState<Dom> for button::Data<TypeDefs>
where
    TypeDefs::children: UpdateRenderState<Dom>,
    TypeDefs::on_click: UpdateDomEventListener<events::OnClick>,
{
    type State = dom::State<
        <TypeDefs::children as UpdateRenderState<Dom>>::State,
        <TypeDefs::on_click as UpdateDomEventListener<events::OnClick>>::State,
    >;

    #[inline]
    fn update_render_state(self, ctx: &mut Dom, state: std::pin::Pin<&mut Self::State>) {
        let props = self.0;
        let state = state.project();

        insert_element_and_update(state.node, ctx, "button", |node, children_ctx| {
            props
                .children
                .update_render_state(children_ctx, state.children);

            props
                .on_click
                .update_dom_event_listener(node, state.on_click);
        })
    }
}

impl<'a, W: AsyncWrite + Unpin, TypeDefs: ?Sized + button::Types>
    UpdateRenderState<SsrContext<'a, W>> for button::Data<TypeDefs>
{
    type State = ssr::State<'a, W>;

    #[inline]
    fn update_render_state(
        self,
        ctx: &mut SsrContext<'a, W>,
        state: std::pin::Pin<&mut Self::State>,
    ) {
        // state.get_mut().update_render_state_with_str(self.0, ctx);
    }
}
