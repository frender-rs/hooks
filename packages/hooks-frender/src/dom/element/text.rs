use std::borrow::Cow;

use wasm_bindgen::{JsValue, UnwrapThrowExt};

use crate::dom::{Dom, RenderState, UpdateRenderState};

use super::super::Context;

// use crate::{
//     dom::{MountToDom, RenderToDom},
//     RenderTo,
// };

#[derive(Debug, Default)]
pub struct TextRendered {
    node: Option<web_sys::Text>,
    cached_data: Option<Cow<'static, str>>,
}

impl RenderState for TextRendered {
    #[inline]
    fn new_uninitialized() -> Self {
        Default::default()
    }

    fn destroy(self: std::pin::Pin<&mut Self>) {
        let current_rendered = self.get_mut();
        web_sys::console::log_2(
            &format!("destroy string").into(),
            &current_rendered
                .node
                .as_ref()
                .map_or(JsValue::default(), JsValue::from),
        );
        if let Some(node) = current_rendered.node.take() {
            node.remove();
        }
        current_rendered.cached_data.take();
    }
}

impl Unpin for TextRendered {}

fn render_static(data: Cow<'static, str>, ctx: Context<TextRendered>) {
    let rendered = ctx.current_rendered.get_mut();

    if let Some(node) = &rendered.node {
        if rendered.cached_data.as_ref() != Some(&data) {
            node.set_data(&data);
            rendered.cached_data = Some(data);
        }
    } else {
        let node = ctx.document.create_text_node(&data);
        ctx.current_parent.append_child(&node).unwrap_throw();
        rendered.node = Some(node);
        rendered.cached_data = Some(data);
    }
}

fn render(data: &str, ctx: Context<TextRendered>) {
    let rendered = ctx.current_rendered.get_mut();

    if let Some(node) = &rendered.node {
        if rendered.cached_data.as_ref() != Some(&Cow::Borrowed(data)) {
            node.set_data(&data);
            rendered.cached_data = None;
        }
    } else {
        let node = ctx.document.create_text_node(&data);
        ctx.current_parent.append_child(&node).unwrap_throw();
        rendered.node = Some(node);
        rendered.cached_data = None;
    }
}

impl UpdateRenderState<Dom<'_>> for &str {
    type State = TextRendered;

    #[inline]
    fn update_render_state(self, ctx: Context<Self::State>) {
        render(self, ctx)
    }
}

impl UpdateRenderState<Dom<'_>> for String {
    type State = TextRendered;

    #[inline]
    fn update_render_state(self, ctx: Context<Self::State>) {
        render_static(Cow::Owned(self), ctx)
    }
}

impl UpdateRenderState<Dom<'_>> for StaticText {
    type State = TextRendered;

    #[inline]
    fn update_render_state(self, ctx: Context<Self::State>) {
        render_static(Cow::Borrowed(self.0), ctx)
    }
}

pub struct StaticText(pub &'static str);
