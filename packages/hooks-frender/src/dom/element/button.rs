use gloo::events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlButtonElement;

use crate::dom::{Context, RenderState, UpdateRenderState};

pub trait ButtonPropsTypes {}

pin_project_lite::pin_project! {
    pub struct ButtonState<Children, OnClick> {
        node: Option<web_sys::HtmlButtonElement>,
        #[pin]
        children: Children,
        on_click: Option<OnClick>,
    }
}

impl<Children: RenderState, OnClick> RenderState for ButtonState<Children, OnClick> {
    fn new_uninitialized() -> Self {
        Self {
            node: None,
            children: Children::new_uninitialized(),
            on_click: None,
        }
    }

    fn destroy(self: std::pin::Pin<&mut Self>) {
        let current_rendered = self.project();

        Children::destroy(current_rendered.children);

        current_rendered.on_click.take();

        if let Some(node) = current_rendered.node.take() {
            node.remove();
        }
    }

    fn poll_render(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        false.into()
    }
}

pub struct Button<Children, OnClick> {
    children: Children,
    on_click: OnClick,
}

impl<Children, OnClick> Button<Children, OnClick> {
    #[inline]
    pub fn children<C>(self, children: C) -> Button<C, OnClick> {
        Button {
            children,
            on_click: self.on_click,
        }
    }

    #[inline]
    pub fn on_click<V>(self, on_click: V) -> Button<Children, V> {
        Button {
            children: self.children,
            on_click,
        }
    }
}

// this is for <button ... /> macro
pub fn button() -> Button<(), ()> {
    Button {
        children: (),
        on_click: (),
    }
}

impl<OnClick: FnMut(&web_sys::MouseEvent) + 'static, Children: UpdateRenderState> UpdateRenderState
    for Button<Children, OnClick>
where
    Children::State: Default,
{
    type State = ButtonState<Children::State, EventListener>;

    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        let current_rendered = ctx.current_rendered.project();

        let node = current_rendered.node.get_or_insert_with(|| {
            let node = ctx.document.create_element("button").unwrap_throw();
            ctx.current_parent.append_child(&node).unwrap_throw();
            node.dyn_into::<HtmlButtonElement>().unwrap_throw()
        });

        let mut on_click = self.on_click;

        *current_rendered.on_click = Some(EventListener::new(node, "click", move |event| {
            on_click(event.dyn_ref().unwrap_throw())
        }));

        self.children.update_render_state(Context {
            document: ctx.document,
            current_parent: node,
            current_rendered: current_rendered.children,
        });
    }
}
