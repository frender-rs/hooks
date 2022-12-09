use std::{any::Any, borrow::Cow, pin::Pin};

use wasm_bindgen::UnwrapThrowExt;

use super::UpdateRenderState;

#[non_exhaustive]
pub struct Context<'a, Rendered: ?Sized> {
    pub document: &'a web_sys::Document,
    pub current_parent: &'a web_sys::Node,
    pub current_rendered: Pin<&'a mut Rendered>,
}

impl<'a, Rendered: ?Sized> Context<'a, Rendered> {
    #[inline]
    pub fn render<E: UpdateRenderState<State = Rendered>>(&mut self, element: E) {
        element.update_render_state(self.as_mut())
    }

    #[inline]
    pub fn as_mut(&mut self) -> Context<'_, Rendered> {
        Context {
            document: self.document,
            current_parent: self.current_parent,
            current_rendered: self.current_rendered.as_mut(),
        }
    }

    #[inline]
    pub fn into_any(self) -> AnyContext<'a>
    where
        Rendered: Sized + 'static,
    {
        AnyContext {
            document: self.document,
            current_parent: self.current_parent,
            current_rendered: self.current_rendered,
        }
    }
}

pub type AnyContext<'a> = Context<'a, dyn Any>;

impl<'a> AnyContext<'a> {
    pub fn downcast_into<Rendered: Any>(self) -> Option<Context<'a, Rendered>> {
        let current_rendered = self.current_rendered;
        // SAFETY: get_unchecked_mut is never used to mutate the data
        let current_rendered = unsafe { current_rendered.get_unchecked_mut() };

        if let Some(current_rendered) = current_rendered.downcast_mut::<Rendered>() {
            Some(Context {
                document: self.document,
                current_parent: self.current_parent,
                // SAFETY: current_rendered comes from a pinned `&mut Rendered`
                current_rendered: unsafe { Pin::new_unchecked(current_rendered) },
            })
        } else {
            None
        }
    }
}

#[non_exhaustive]
pub struct DomContext<'a> {
    pub document: &'a web_sys::Document,
    pub current_parent: &'a web_sys::Node,
}

pub struct DomRenderedNode<'a> {
    pub node: &'a web_sys::Node,
    pub rendered: &'a mut dyn Any,
}

pub trait MountToDom {
    fn mount_to_dom(&mut self, parent: &web_sys::Node, document: &web_sys::Document);
}

#[cfg(aaa)]
pub trait RenderToDom {
    type DomRendered: 'static + MountToDom;

    fn can_update(&self, rendered: &Self::DomRendered, node: &web_sys::Node) -> bool;

    fn create_to_dom(&self, ctx: DomContext) -> Self::DomRendered;

    #[inline]
    fn create_into_dom(self, ctx: DomContext) -> Self::DomRendered
    where
        Self: Sized,
    {
        self.create_to_dom(ctx)
    }

    fn update_to_rendered(&self, rendered: &mut Self::DomRendered);
    fn update_into_rendered(&self, rendered: &mut Self::DomRendered)
    where
        Self: Sized,
    {
        self.update_to_rendered(rendered)
    }

    fn render_to_dom<'ctx>(
        &self,
        ctx: Context<'ctx, Option<Self::DomRendered>>,
    ) -> &'ctx mut Option<Self::DomRendered> {
        if let Some(current_rendered) = ctx.current_rendered {
            self.update_to_rendered(current_rendered);
        } else {
            *ctx.current_rendered = Some(self.create_to_dom(DomContext {
                document: ctx.document,
                current_parent: ctx.current_parent,
            }));
        }

        ctx.current_rendered
    }
}

fn remove_children_after_including(
    parent: &web_sys::Node,
    mut first_child_to_remove: Cow<web_sys::Node>,
) {
    #[cfg(debug_assertions)]
    assert_eq!(first_child_to_remove.parent_node().as_ref(), Some(parent));

    while let Some(next_node) = first_child_to_remove.next_sibling() {
        parent.remove_child(&first_child_to_remove).unwrap_throw();
        first_child_to_remove = Cow::Owned(next_node);
    }

    parent.remove_child(&first_child_to_remove).unwrap_throw();
}
