#![feature(prelude_import)]
#![deny(clippy::undocumented_unsafe_blocks)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod hook_component {
    use std::{marker::PhantomData, pin::Pin};
    use hooks::Hook;
    use crate::dom::{Context, RenderToDom, Rendered};
    pub struct HookComponent<F, R> {
        get_hook: F,
        _rendered: PhantomData<R>,
    }
    impl<F, H, R: Rendered> RenderToDom for HookComponent<F, R>
    where
        F: FnOnce() -> H,
        H: for<'ctx> Hook<(Context<'ctx, R>,), Value = Context<'ctx, R>>,
    {
        type Rendered = R;
        fn render_to_dom(self, ctx: crate::dom::Context<Self::Rendered>) {
            ::core::panicking::panic("not yet implemented")
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
    impl<'a, H, R: Rendered> RenderToDom for HookComp<'a, H, R>
    where
        H: for<'ctx> Hook<(Context<'ctx, R>,), Value = Context<'ctx, R>>,
    {
        type Rendered = R;
        fn render_to_dom(self, ctx: Context<Self::Rendered>) {
            self.inner.use_hook((ctx,));
        }
    }
}
mod hook_element {
    use std::{future::Future, pin::Pin};
    use hooks::{Hook, HookExt};
    use crate::dom::{AnyContext, Context, Rendered};
    struct Reentrant<F> {
        fut: F,
        finished: bool,
    }
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::used_underscore_binding)]
    const _: () = {
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::ref_option_ref)]
        #[allow(clippy::type_repetition_in_bounds)]
        struct Projection<'__pin, F>
        where
            Reentrant<F>: '__pin,
        {
            fut: ::pin_project_lite::__private::Pin<&'__pin mut (F)>,
            finished: &'__pin mut (bool),
        }
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::ref_option_ref)]
        #[allow(clippy::type_repetition_in_bounds)]
        struct ProjectionRef<'__pin, F>
        where
            Reentrant<F>: '__pin,
        {
            fut: ::pin_project_lite::__private::Pin<&'__pin (F)>,
            finished: &'__pin (bool),
        }
        impl<F> Reentrant<F> {
            fn project<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
            ) -> Projection<'__pin, F> {
                unsafe {
                    let Self { fut, finished } = self.get_unchecked_mut();
                    Projection {
                        fut: ::pin_project_lite::__private::Pin::new_unchecked(fut),
                        finished: finished,
                    }
                }
            }
            fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, F> {
                unsafe {
                    let Self { fut, finished } = self.get_ref();
                    ProjectionRef {
                        fut: ::pin_project_lite::__private::Pin::new_unchecked(fut),
                        finished: finished,
                    }
                }
            }
        }
        #[allow(non_snake_case)]
        struct __Origin<'__pin, F> {
            __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
            fut: F,
            finished: ::pin_project_lite::__private::AlwaysUnpin<bool>,
        }
        impl<'__pin, F> ::pin_project_lite::__private::Unpin for Reentrant<F>
        where
            __Origin<'__pin, F>: ::pin_project_lite::__private::Unpin,
        {}
        trait MustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
        impl<F> MustNotImplDrop for Reentrant<F> {}
        #[forbid(unaligned_references, safe_packed_borrows)]
        fn __assert_not_repr_packed<F>(this: &Reentrant<F>) {
            let _ = &this.fut;
            let _ = &this.finished;
        }
    };
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
    pub struct HookElement<H> {
        hook: H,
    }
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::used_underscore_binding)]
    const _: () = {
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::ref_option_ref)]
        #[allow(clippy::type_repetition_in_bounds)]
        pub(crate) struct Projection<'__pin, H>
        where
            HookElement<H>: '__pin,
        {
            hook: ::pin_project_lite::__private::Pin<&'__pin mut (H)>,
        }
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::ref_option_ref)]
        #[allow(clippy::type_repetition_in_bounds)]
        pub(crate) struct ProjectionRef<'__pin, H>
        where
            HookElement<H>: '__pin,
        {
            hook: ::pin_project_lite::__private::Pin<&'__pin (H)>,
        }
        impl<H> HookElement<H> {
            pub(crate) fn project<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
            ) -> Projection<'__pin, H> {
                unsafe {
                    let Self { hook } = self.get_unchecked_mut();
                    Projection {
                        hook: ::pin_project_lite::__private::Pin::new_unchecked(hook),
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, H> {
                unsafe {
                    let Self { hook } = self.get_ref();
                    ProjectionRef {
                        hook: ::pin_project_lite::__private::Pin::new_unchecked(hook),
                    }
                }
            }
        }
        #[allow(non_snake_case)]
        pub struct __Origin<'__pin, H> {
            __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
            hook: H,
        }
        impl<'__pin, H> ::pin_project_lite::__private::Unpin for HookElement<H>
        where
            __Origin<'__pin, H>: ::pin_project_lite::__private::Unpin,
        {}
        trait MustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
        impl<H> MustNotImplDrop for HookElement<H> {}
        #[forbid(unaligned_references, safe_packed_borrows)]
        fn __assert_not_repr_packed<H>(this: &HookElement<H>) {
            let _ = &this.hook;
        }
    };
    impl<H> HookElement<H> {
        #[inline]
        pub fn new(hook: H) -> Self {
            Self { hook }
        }
        pub fn into_render<R: 'static + Rendered>(
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
            self.use_hook((
                Context {
                    document: &document,
                    current_parent: &current_parent,
                    current_rendered: Pin::new(&mut current_rendered),
                },
            ));
            async move {
                let destroy = Reentrant {
                    fut: destroy,
                    finished: false,
                };
                let mut destroy = destroy;
                #[allow(unused_mut)]
                let mut destroy = unsafe {
                    ::core::pin::Pin::new_unchecked(&mut destroy)
                };
                let mut current_rendered = current_rendered;
                #[allow(unused_mut)]
                let mut current_rendered = unsafe {
                    ::core::pin::Pin::new_unchecked(&mut current_rendered)
                };
                futures_lite::future::or(
                        async {
                            while self
                                .next_value((
                                    Context {
                                        document: &document,
                                        current_parent: &current_parent,
                                        current_rendered: current_rendered.as_mut(),
                                    },
                                ))
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
    impl<H, R: 'static> ::hooks_core::HookBounds for HookElement<H>
    where
        H: for<'c> Hook<(AnyContext<'c>,), Value = Context<'c, R>>,
    {
        type Bounds = Self;
    }
    impl<H, R: 'static> ::hooks_core::HookPollNextUpdate for HookElement<H>
    where
        H: for<'c> Hook<(AnyContext<'c>,), Value = Context<'c, R>>,
    {
        #[inline]
        fn poll_next_update(
            self: ::core::pin::Pin<&mut Self>,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> {
            self.project().hook.poll_next_update(cx)
        }
    }
    impl<
        'hook,
        'ctx,
        H,
        R: 'static,
    > ::hooks_core::HookLifetime<'hook, (Context<'ctx, R>,), &'hook Self>
    for HookElement<H>
    where
        H: for<'c> Hook<(AnyContext<'c>,), Value = Context<'c, R>>,
    {
        type Value = Context<'ctx, R>;
    }
    impl<'ctx, H, R: 'static> ::hooks_core::Hook<(Context<'ctx, R>,)> for HookElement<H>
    where
        H: for<'c> Hook<(AnyContext<'c>,), Value = Context<'c, R>>,
    {
        #[inline]
        fn use_hook<'hook>(
            self: ::core::pin::Pin<&'hook mut Self>,
            (ctx,): (Context<'ctx, R>,),
        ) -> <Self as ::hooks_core::HookLifetime<'hook, (Context<'ctx, R>,)>>::Value
        where
            Self: 'hook,
        {
            self.project().hook.use_hook((ctx.into_any(),))
        }
    }
}
mod pin_macro {}
pub use hook_component::*;
pub use hook_element::*;
pub use pin_macro::*;
use dom::{RenderToDom, Rendered};
use std::{any::Any, pin::Pin};
use gloo::events::EventListener;
use hooks::{hook, memo_with, DataAndDep, Hook, HookExt};
use wasm_bindgen::UnwrapThrowExt;
use crate::dom::{AnyContext, Context};
pub mod dom {
    mod context {
        use std::{any::Any, borrow::Cow, pin::Pin};
        use wasm_bindgen::UnwrapThrowExt;
        use super::RenderToDom;
        #[non_exhaustive]
        pub struct Context<'a, Rendered: ?Sized> {
            pub document: &'a web_sys::Document,
            pub current_parent: &'a web_sys::Node,
            pub current_rendered: Pin<&'a mut Rendered>,
        }
        impl<'a, Rendered: ?Sized> Context<'a, Rendered> {
            #[inline]
            pub fn render<E: RenderToDom<Rendered = Rendered>>(&mut self, element: E) {
                element.render_to_dom(self.as_mut())
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
                let current_rendered = unsafe { current_rendered.get_unchecked_mut() };
                if let Some(current_rendered)
                    = current_rendered.downcast_mut::<Rendered>()
                {
                    Some(Context {
                        document: self.document,
                        current_parent: self.current_parent,
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
            fn mount_to_dom(
                &mut self,
                parent: &web_sys::Node,
                document: &web_sys::Document,
            );
        }
        fn remove_children_after_including(
            parent: &web_sys::Node,
            mut first_child_to_remove: Cow<web_sys::Node>,
        ) {
            match (&first_child_to_remove.parent_node().as_ref(), &Some(parent)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            while let Some(next_node) = first_child_to_remove.next_sibling() {
                parent.remove_child(&first_child_to_remove).unwrap_throw();
                first_child_to_remove = Cow::Owned(next_node);
            }
            parent.remove_child(&first_child_to_remove).unwrap_throw();
        }
    }
    mod render_to {
        use std::pin::Pin;
        use super::Context;
        pub trait Rendered {
            fn new_uninitialized() -> Self;
            fn destroy(self: Pin<&mut Self>);
        }
        impl Rendered for () {
            #[inline]
            fn new_uninitialized() -> Self {}
            #[inline]
            fn destroy(self: Pin<&mut Self>) {}
        }
        pub trait RenderToDom {
            type Rendered: Rendered;
            fn render_to_dom(self, ctx: Context<Self::Rendered>);
        }
        impl<R: RenderToDom> RenderToDom for Box<R> {
            type Rendered = R::Rendered;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                R::render_to_dom(*self, ctx)
            }
        }
        impl<R: RenderToDom> RenderToDom for Option<R> {
            type Rendered = R::Rendered;
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                if let Some(element) = self {
                    element.render_to_dom(ctx)
                } else {
                    ctx.current_rendered.destroy()
                }
            }
        }
    }
    mod tuple {
        #![allow(non_snake_case)]
        use super::{Context, RenderToDom, Rendered};
        impl RenderToDom for () {
            type Rendered = ();
            #[inline]
            fn render_to_dom(self, _: Context<Self::Rendered>) {}
        }
        impl<R0: RenderToDom> RenderToDom for (R0,) {
            type Rendered = R0::Rendered;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                R0::render_to_dom(self.0, ctx)
            }
        }
        pub struct Tuple2Rendered<R0, R1> {
            R0: R0,
            R1: R1,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
        > ::core::default::Default for Tuple2Rendered<R0, R1> {
            #[inline]
            fn default() -> Tuple2Rendered<R0, R1> {
                Tuple2Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R0, R1>
            where
                Tuple2Rendered<R0, R1>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R0, R1>
            where
                Tuple2Rendered<R0, R1>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
            }
            impl<R0, R1> Tuple2Rendered<R0, R1> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R0, R1> {
                    unsafe {
                        let Self { R0, R1 } = self.get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R0, R1> {
                    unsafe {
                        let Self { R0, R1 } = self.get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R0, R1> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
            }
            impl<'__pin, R0, R1> ::pin_project_lite::__private::Unpin
            for Tuple2Rendered<R0, R1>
            where
                __Origin<'__pin, R0, R1>: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1> MustNotImplDrop for Tuple2Rendered<R0, R1> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R0, R1>(this: &Tuple2Rendered<R0, R1>) {
                let _ = &this.R0;
                let _ = &this.R1;
            }
        };
        impl<R0: Rendered, R1: Rendered> Rendered for Tuple2Rendered<R0, R1> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
            }
        }
        impl<R0: RenderToDom, R1: RenderToDom> RenderToDom for (R0, R1) {
            type Rendered = Tuple2Rendered<R0::Rendered, R1::Rendered>;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
            }
        }
        pub struct Tuple3Rendered<R0, R1, R2> {
            R0: R0,
            R1: R1,
            R2: R2,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
        > ::core::default::Default for Tuple3Rendered<R0, R1, R2> {
            #[inline]
            fn default() -> Tuple3Rendered<R0, R1, R2> {
                Tuple3Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R0, R1, R2>
            where
                Tuple3Rendered<R0, R1, R2>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R0, R1, R2>
            where
                Tuple3Rendered<R0, R1, R2>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
            }
            impl<R0, R1, R2> Tuple3Rendered<R0, R1, R2> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R0, R1, R2> {
                    unsafe {
                        let Self { R0, R1, R2 } = self.get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R0, R1, R2> {
                    unsafe {
                        let Self { R0, R1, R2 } = self.get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R0, R1, R2> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
            }
            impl<'__pin, R0, R1, R2> ::pin_project_lite::__private::Unpin
            for Tuple3Rendered<R0, R1, R2>
            where
                __Origin<'__pin, R0, R1, R2>: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2> MustNotImplDrop for Tuple3Rendered<R0, R1, R2> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R0, R1, R2>(this: &Tuple3Rendered<R0, R1, R2>) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
            }
        };
        impl<R0: Rendered, R1: Rendered, R2: Rendered> Rendered
        for Tuple3Rendered<R0, R1, R2> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
            }
        }
        impl<R0: RenderToDom, R1: RenderToDom, R2: RenderToDom> RenderToDom
        for (R0, R1, R2) {
            type Rendered = Tuple3Rendered<R0::Rendered, R1::Rendered, R2::Rendered>;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
            }
        }
        pub struct Tuple4Rendered<R0, R1, R2, R3> {
            R0: R0,
            R1: R1,
            R2: R2,
            R3: R3,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
            R3: ::core::default::Default,
        > ::core::default::Default for Tuple4Rendered<R0, R1, R2, R3> {
            #[inline]
            fn default() -> Tuple4Rendered<R0, R1, R2, R3> {
                Tuple4Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                    R3: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R0, R1, R2, R3>
            where
                Tuple4Rendered<R0, R1, R2, R3>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin mut (R3)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R0, R1, R2, R3>
            where
                Tuple4Rendered<R0, R1, R2, R3>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin (R3)>,
            }
            impl<R0, R1, R2, R3> Tuple4Rendered<R0, R1, R2, R3> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R0, R1, R2, R3> {
                    unsafe {
                        let Self { R0, R1, R2, R3 } = self.get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R0, R1, R2, R3> {
                    unsafe {
                        let Self { R0, R1, R2, R3 } = self.get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R0, R1, R2, R3> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
                R3: R3,
            }
            impl<'__pin, R0, R1, R2, R3> ::pin_project_lite::__private::Unpin
            for Tuple4Rendered<R0, R1, R2, R3>
            where
                __Origin<'__pin, R0, R1, R2, R3>: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2, R3> MustNotImplDrop for Tuple4Rendered<R0, R1, R2, R3> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R0, R1, R2, R3>(
                this: &Tuple4Rendered<R0, R1, R2, R3>,
            ) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
                let _ = &this.R3;
            }
        };
        impl<R0: Rendered, R1: Rendered, R2: Rendered, R3: Rendered> Rendered
        for Tuple4Rendered<R0, R1, R2, R3> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                    R3: R3::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
                this.R3.destroy();
            }
        }
        impl<
            R0: RenderToDom,
            R1: RenderToDom,
            R2: RenderToDom,
            R3: RenderToDom,
        > RenderToDom for (R0, R1, R2, R3) {
            type Rendered = Tuple4Rendered<
                R0::Rendered,
                R1::Rendered,
                R2::Rendered,
                R3::Rendered,
            >;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2, R3) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
                R3::render_to_dom(
                    R3,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R3,
                    },
                );
            }
        }
        pub struct Tuple5Rendered<R0, R1, R2, R3, R4> {
            R0: R0,
            R1: R1,
            R2: R2,
            R3: R3,
            R4: R4,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
            R3: ::core::default::Default,
            R4: ::core::default::Default,
        > ::core::default::Default for Tuple5Rendered<R0, R1, R2, R3, R4> {
            #[inline]
            fn default() -> Tuple5Rendered<R0, R1, R2, R3, R4> {
                Tuple5Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                    R3: ::core::default::Default::default(),
                    R4: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R0, R1, R2, R3, R4>
            where
                Tuple5Rendered<R0, R1, R2, R3, R4>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin mut (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin mut (R4)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R0, R1, R2, R3, R4>
            where
                Tuple5Rendered<R0, R1, R2, R3, R4>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin (R4)>,
            }
            impl<R0, R1, R2, R3, R4> Tuple5Rendered<R0, R1, R2, R3, R4> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R0, R1, R2, R3, R4> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4 } = self.get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R0, R1, R2, R3, R4> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4 } = self.get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R0, R1, R2, R3, R4> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
                R3: R3,
                R4: R4,
            }
            impl<'__pin, R0, R1, R2, R3, R4> ::pin_project_lite::__private::Unpin
            for Tuple5Rendered<R0, R1, R2, R3, R4>
            where
                __Origin<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                >: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2, R3, R4> MustNotImplDrop
            for Tuple5Rendered<R0, R1, R2, R3, R4> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R0, R1, R2, R3, R4>(
                this: &Tuple5Rendered<R0, R1, R2, R3, R4>,
            ) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
                let _ = &this.R3;
                let _ = &this.R4;
            }
        };
        impl<
            R0: Rendered,
            R1: Rendered,
            R2: Rendered,
            R3: Rendered,
            R4: Rendered,
        > Rendered for Tuple5Rendered<R0, R1, R2, R3, R4> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                    R3: R3::new_uninitialized(),
                    R4: R4::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
                this.R3.destroy();
                this.R4.destroy();
            }
        }
        impl<
            R0: RenderToDom,
            R1: RenderToDom,
            R2: RenderToDom,
            R3: RenderToDom,
            R4: RenderToDom,
        > RenderToDom for (R0, R1, R2, R3, R4) {
            type Rendered = Tuple5Rendered<
                R0::Rendered,
                R1::Rendered,
                R2::Rendered,
                R3::Rendered,
                R4::Rendered,
            >;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2, R3, R4) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
                R3::render_to_dom(
                    R3,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R3,
                    },
                );
                R4::render_to_dom(
                    R4,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R4,
                    },
                );
            }
        }
        pub struct Tuple6Rendered<R0, R1, R2, R3, R4, R5> {
            R0: R0,
            R1: R1,
            R2: R2,
            R3: R3,
            R4: R4,
            R5: R5,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
            R3: ::core::default::Default,
            R4: ::core::default::Default,
            R5: ::core::default::Default,
        > ::core::default::Default for Tuple6Rendered<R0, R1, R2, R3, R4, R5> {
            #[inline]
            fn default() -> Tuple6Rendered<R0, R1, R2, R3, R4, R5> {
                Tuple6Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                    R3: ::core::default::Default::default(),
                    R4: ::core::default::Default::default(),
                    R5: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R0, R1, R2, R3, R4, R5>
            where
                Tuple6Rendered<R0, R1, R2, R3, R4, R5>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin mut (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin mut (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin mut (R5)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R0, R1, R2, R3, R4, R5>
            where
                Tuple6Rendered<R0, R1, R2, R3, R4, R5>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin (R5)>,
            }
            impl<R0, R1, R2, R3, R4, R5> Tuple6Rendered<R0, R1, R2, R3, R4, R5> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R0, R1, R2, R3, R4, R5> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5 } = self.get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R0, R1, R2, R3, R4, R5> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5 } = self.get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R0, R1, R2, R3, R4, R5> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
                R3: R3,
                R4: R4,
                R5: R5,
            }
            impl<'__pin, R0, R1, R2, R3, R4, R5> ::pin_project_lite::__private::Unpin
            for Tuple6Rendered<R0, R1, R2, R3, R4, R5>
            where
                __Origin<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                >: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2, R3, R4, R5> MustNotImplDrop
            for Tuple6Rendered<R0, R1, R2, R3, R4, R5> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R0, R1, R2, R3, R4, R5>(
                this: &Tuple6Rendered<R0, R1, R2, R3, R4, R5>,
            ) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
                let _ = &this.R3;
                let _ = &this.R4;
                let _ = &this.R5;
            }
        };
        impl<
            R0: Rendered,
            R1: Rendered,
            R2: Rendered,
            R3: Rendered,
            R4: Rendered,
            R5: Rendered,
        > Rendered for Tuple6Rendered<R0, R1, R2, R3, R4, R5> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                    R3: R3::new_uninitialized(),
                    R4: R4::new_uninitialized(),
                    R5: R5::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
                this.R3.destroy();
                this.R4.destroy();
                this.R5.destroy();
            }
        }
        impl<
            R0: RenderToDom,
            R1: RenderToDom,
            R2: RenderToDom,
            R3: RenderToDom,
            R4: RenderToDom,
            R5: RenderToDom,
        > RenderToDom for (R0, R1, R2, R3, R4, R5) {
            type Rendered = Tuple6Rendered<
                R0::Rendered,
                R1::Rendered,
                R2::Rendered,
                R3::Rendered,
                R4::Rendered,
                R5::Rendered,
            >;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2, R3, R4, R5) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
                R3::render_to_dom(
                    R3,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R3,
                    },
                );
                R4::render_to_dom(
                    R4,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R4,
                    },
                );
                R5::render_to_dom(
                    R5,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R5,
                    },
                );
            }
        }
        pub struct Tuple7Rendered<R0, R1, R2, R3, R4, R5, R6> {
            R0: R0,
            R1: R1,
            R2: R2,
            R3: R3,
            R4: R4,
            R5: R5,
            R6: R6,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
            R3: ::core::default::Default,
            R4: ::core::default::Default,
            R5: ::core::default::Default,
            R6: ::core::default::Default,
        > ::core::default::Default for Tuple7Rendered<R0, R1, R2, R3, R4, R5, R6> {
            #[inline]
            fn default() -> Tuple7Rendered<R0, R1, R2, R3, R4, R5, R6> {
                Tuple7Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                    R3: ::core::default::Default::default(),
                    R4: ::core::default::Default::default(),
                    R5: ::core::default::Default::default(),
                    R6: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R0, R1, R2, R3, R4, R5, R6>
            where
                Tuple7Rendered<R0, R1, R2, R3, R4, R5, R6>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin mut (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin mut (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin mut (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin mut (R6)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R0, R1, R2, R3, R4, R5, R6>
            where
                Tuple7Rendered<R0, R1, R2, R3, R4, R5, R6>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin (R6)>,
            }
            impl<R0, R1, R2, R3, R4, R5, R6> Tuple7Rendered<R0, R1, R2, R3, R4, R5, R6> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R0, R1, R2, R3, R4, R5, R6> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6 } = self
                            .get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R0, R1, R2, R3, R4, R5, R6> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6 } = self.get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R0, R1, R2, R3, R4, R5, R6> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
                R3: R3,
                R4: R4,
                R5: R5,
                R6: R6,
            }
            impl<'__pin, R0, R1, R2, R3, R4, R5, R6> ::pin_project_lite::__private::Unpin
            for Tuple7Rendered<R0, R1, R2, R3, R4, R5, R6>
            where
                __Origin<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                >: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2, R3, R4, R5, R6> MustNotImplDrop
            for Tuple7Rendered<R0, R1, R2, R3, R4, R5, R6> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R0, R1, R2, R3, R4, R5, R6>(
                this: &Tuple7Rendered<R0, R1, R2, R3, R4, R5, R6>,
            ) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
                let _ = &this.R3;
                let _ = &this.R4;
                let _ = &this.R5;
                let _ = &this.R6;
            }
        };
        impl<
            R0: Rendered,
            R1: Rendered,
            R2: Rendered,
            R3: Rendered,
            R4: Rendered,
            R5: Rendered,
            R6: Rendered,
        > Rendered for Tuple7Rendered<R0, R1, R2, R3, R4, R5, R6> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                    R3: R3::new_uninitialized(),
                    R4: R4::new_uninitialized(),
                    R5: R5::new_uninitialized(),
                    R6: R6::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
                this.R3.destroy();
                this.R4.destroy();
                this.R5.destroy();
                this.R6.destroy();
            }
        }
        impl<
            R0: RenderToDom,
            R1: RenderToDom,
            R2: RenderToDom,
            R3: RenderToDom,
            R4: RenderToDom,
            R5: RenderToDom,
            R6: RenderToDom,
        > RenderToDom for (R0, R1, R2, R3, R4, R5, R6) {
            type Rendered = Tuple7Rendered<
                R0::Rendered,
                R1::Rendered,
                R2::Rendered,
                R3::Rendered,
                R4::Rendered,
                R5::Rendered,
                R6::Rendered,
            >;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2, R3, R4, R5, R6) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
                R3::render_to_dom(
                    R3,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R3,
                    },
                );
                R4::render_to_dom(
                    R4,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R4,
                    },
                );
                R5::render_to_dom(
                    R5,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R5,
                    },
                );
                R6::render_to_dom(
                    R6,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R6,
                    },
                );
            }
        }
        pub struct Tuple8Rendered<R0, R1, R2, R3, R4, R5, R6, R7> {
            R0: R0,
            R1: R1,
            R2: R2,
            R3: R3,
            R4: R4,
            R5: R5,
            R6: R6,
            R7: R7,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
            R3: ::core::default::Default,
            R4: ::core::default::Default,
            R5: ::core::default::Default,
            R6: ::core::default::Default,
            R7: ::core::default::Default,
        > ::core::default::Default for Tuple8Rendered<R0, R1, R2, R3, R4, R5, R6, R7> {
            #[inline]
            fn default() -> Tuple8Rendered<R0, R1, R2, R3, R4, R5, R6, R7> {
                Tuple8Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                    R3: ::core::default::Default::default(),
                    R4: ::core::default::Default::default(),
                    R5: ::core::default::Default::default(),
                    R6: ::core::default::Default::default(),
                    R7: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R0, R1, R2, R3, R4, R5, R6, R7>
            where
                Tuple8Rendered<R0, R1, R2, R3, R4, R5, R6, R7>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin mut (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin mut (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin mut (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin mut (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin mut (R7)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R0, R1, R2, R3, R4, R5, R6, R7>
            where
                Tuple8Rendered<R0, R1, R2, R3, R4, R5, R6, R7>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin (R7)>,
            }
            impl<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
            > Tuple8Rendered<R0, R1, R2, R3, R4, R5, R6, R7> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R0, R1, R2, R3, R4, R5, R6, R7> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6, R7 } = self
                            .get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R0, R1, R2, R3, R4, R5, R6, R7> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6, R7 } = self.get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R0, R1, R2, R3, R4, R5, R6, R7> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
                R3: R3,
                R4: R4,
                R5: R5,
                R6: R6,
                R7: R7,
            }
            impl<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
            > ::pin_project_lite::__private::Unpin
            for Tuple8Rendered<R0, R1, R2, R3, R4, R5, R6, R7>
            where
                __Origin<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                >: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2, R3, R4, R5, R6, R7> MustNotImplDrop
            for Tuple8Rendered<R0, R1, R2, R3, R4, R5, R6, R7> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R0, R1, R2, R3, R4, R5, R6, R7>(
                this: &Tuple8Rendered<R0, R1, R2, R3, R4, R5, R6, R7>,
            ) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
                let _ = &this.R3;
                let _ = &this.R4;
                let _ = &this.R5;
                let _ = &this.R6;
                let _ = &this.R7;
            }
        };
        impl<
            R0: Rendered,
            R1: Rendered,
            R2: Rendered,
            R3: Rendered,
            R4: Rendered,
            R5: Rendered,
            R6: Rendered,
            R7: Rendered,
        > Rendered for Tuple8Rendered<R0, R1, R2, R3, R4, R5, R6, R7> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                    R3: R3::new_uninitialized(),
                    R4: R4::new_uninitialized(),
                    R5: R5::new_uninitialized(),
                    R6: R6::new_uninitialized(),
                    R7: R7::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
                this.R3.destroy();
                this.R4.destroy();
                this.R5.destroy();
                this.R6.destroy();
                this.R7.destroy();
            }
        }
        impl<
            R0: RenderToDom,
            R1: RenderToDom,
            R2: RenderToDom,
            R3: RenderToDom,
            R4: RenderToDom,
            R5: RenderToDom,
            R6: RenderToDom,
            R7: RenderToDom,
        > RenderToDom for (R0, R1, R2, R3, R4, R5, R6, R7) {
            type Rendered = Tuple8Rendered<
                R0::Rendered,
                R1::Rendered,
                R2::Rendered,
                R3::Rendered,
                R4::Rendered,
                R5::Rendered,
                R6::Rendered,
                R7::Rendered,
            >;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2, R3, R4, R5, R6, R7) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
                R3::render_to_dom(
                    R3,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R3,
                    },
                );
                R4::render_to_dom(
                    R4,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R4,
                    },
                );
                R5::render_to_dom(
                    R5,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R5,
                    },
                );
                R6::render_to_dom(
                    R6,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R6,
                    },
                );
                R7::render_to_dom(
                    R7,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R7,
                    },
                );
            }
        }
        pub struct Tuple9Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8> {
            R0: R0,
            R1: R1,
            R2: R2,
            R3: R3,
            R4: R4,
            R5: R5,
            R6: R6,
            R7: R7,
            R8: R8,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
            R3: ::core::default::Default,
            R4: ::core::default::Default,
            R5: ::core::default::Default,
            R6: ::core::default::Default,
            R7: ::core::default::Default,
            R8: ::core::default::Default,
        > ::core::default::Default
        for Tuple9Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8> {
            #[inline]
            fn default() -> Tuple9Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8> {
                Tuple9Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                    R3: ::core::default::Default::default(),
                    R4: ::core::default::Default::default(),
                    R5: ::core::default::Default::default(),
                    R6: ::core::default::Default::default(),
                    R7: ::core::default::Default::default(),
                    R8: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8>
            where
                Tuple9Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin mut (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin mut (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin mut (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin mut (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin mut (R7)>,
                R8: ::pin_project_lite::__private::Pin<&'__pin mut (R8)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8>
            where
                Tuple9Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin (R7)>,
                R8: ::pin_project_lite::__private::Pin<&'__pin (R8)>,
            }
            impl<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
            > Tuple9Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6, R7, R8 } = self
                            .get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                            R8: ::pin_project_lite::__private::Pin::new_unchecked(R8),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6, R7, R8 } = self.get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                            R8: ::pin_project_lite::__private::Pin::new_unchecked(R8),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
                R3: R3,
                R4: R4,
                R5: R5,
                R6: R6,
                R7: R7,
                R8: R8,
            }
            impl<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
            > ::pin_project_lite::__private::Unpin
            for Tuple9Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8>
            where
                __Origin<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                >: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2, R3, R4, R5, R6, R7, R8> MustNotImplDrop
            for Tuple9Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R0, R1, R2, R3, R4, R5, R6, R7, R8>(
                this: &Tuple9Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8>,
            ) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
                let _ = &this.R3;
                let _ = &this.R4;
                let _ = &this.R5;
                let _ = &this.R6;
                let _ = &this.R7;
                let _ = &this.R8;
            }
        };
        impl<
            R0: Rendered,
            R1: Rendered,
            R2: Rendered,
            R3: Rendered,
            R4: Rendered,
            R5: Rendered,
            R6: Rendered,
            R7: Rendered,
            R8: Rendered,
        > Rendered for Tuple9Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                    R3: R3::new_uninitialized(),
                    R4: R4::new_uninitialized(),
                    R5: R5::new_uninitialized(),
                    R6: R6::new_uninitialized(),
                    R7: R7::new_uninitialized(),
                    R8: R8::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
                this.R3.destroy();
                this.R4.destroy();
                this.R5.destroy();
                this.R6.destroy();
                this.R7.destroy();
                this.R8.destroy();
            }
        }
        impl<
            R0: RenderToDom,
            R1: RenderToDom,
            R2: RenderToDom,
            R3: RenderToDom,
            R4: RenderToDom,
            R5: RenderToDom,
            R6: RenderToDom,
            R7: RenderToDom,
            R8: RenderToDom,
        > RenderToDom for (R0, R1, R2, R3, R4, R5, R6, R7, R8) {
            type Rendered = Tuple9Rendered<
                R0::Rendered,
                R1::Rendered,
                R2::Rendered,
                R3::Rendered,
                R4::Rendered,
                R5::Rendered,
                R6::Rendered,
                R7::Rendered,
                R8::Rendered,
            >;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2, R3, R4, R5, R6, R7, R8) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
                R3::render_to_dom(
                    R3,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R3,
                    },
                );
                R4::render_to_dom(
                    R4,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R4,
                    },
                );
                R5::render_to_dom(
                    R5,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R5,
                    },
                );
                R6::render_to_dom(
                    R6,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R6,
                    },
                );
                R7::render_to_dom(
                    R7,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R7,
                    },
                );
                R8::render_to_dom(
                    R8,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R8,
                    },
                );
            }
        }
        pub struct Tuple10Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> {
            R0: R0,
            R1: R1,
            R2: R2,
            R3: R3,
            R4: R4,
            R5: R5,
            R6: R6,
            R7: R7,
            R8: R8,
            R9: R9,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
            R3: ::core::default::Default,
            R4: ::core::default::Default,
            R5: ::core::default::Default,
            R6: ::core::default::Default,
            R7: ::core::default::Default,
            R8: ::core::default::Default,
            R9: ::core::default::Default,
        > ::core::default::Default
        for Tuple10Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> {
            #[inline]
            fn default() -> Tuple10Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> {
                Tuple10Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                    R3: ::core::default::Default::default(),
                    R4: ::core::default::Default::default(),
                    R5: ::core::default::Default::default(),
                    R6: ::core::default::Default::default(),
                    R7: ::core::default::Default::default(),
                    R8: ::core::default::Default::default(),
                    R9: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8, R9>
            where
                Tuple10Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin mut (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin mut (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin mut (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin mut (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin mut (R7)>,
                R8: ::pin_project_lite::__private::Pin<&'__pin mut (R8)>,
                R9: ::pin_project_lite::__private::Pin<&'__pin mut (R9)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
            >
            where
                Tuple10Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin (R7)>,
                R8: ::pin_project_lite::__private::Pin<&'__pin (R8)>,
                R9: ::pin_project_lite::__private::Pin<&'__pin (R9)>,
            }
            impl<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
            > Tuple10Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6, R7, R8, R9 } = self
                            .get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                            R8: ::pin_project_lite::__private::Pin::new_unchecked(R8),
                            R9: ::pin_project_lite::__private::Pin::new_unchecked(R9),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6, R7, R8, R9 } = self
                            .get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                            R8: ::pin_project_lite::__private::Pin::new_unchecked(R8),
                            R9: ::pin_project_lite::__private::Pin::new_unchecked(R9),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
                R3: R3,
                R4: R4,
                R5: R5,
                R6: R6,
                R7: R7,
                R8: R8,
                R9: R9,
            }
            impl<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
            > ::pin_project_lite::__private::Unpin
            for Tuple10Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9>
            where
                __Origin<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                >: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> MustNotImplDrop
            for Tuple10Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9>(
                this: &Tuple10Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9>,
            ) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
                let _ = &this.R3;
                let _ = &this.R4;
                let _ = &this.R5;
                let _ = &this.R6;
                let _ = &this.R7;
                let _ = &this.R8;
                let _ = &this.R9;
            }
        };
        impl<
            R0: Rendered,
            R1: Rendered,
            R2: Rendered,
            R3: Rendered,
            R4: Rendered,
            R5: Rendered,
            R6: Rendered,
            R7: Rendered,
            R8: Rendered,
            R9: Rendered,
        > Rendered for Tuple10Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                    R3: R3::new_uninitialized(),
                    R4: R4::new_uninitialized(),
                    R5: R5::new_uninitialized(),
                    R6: R6::new_uninitialized(),
                    R7: R7::new_uninitialized(),
                    R8: R8::new_uninitialized(),
                    R9: R9::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
                this.R3.destroy();
                this.R4.destroy();
                this.R5.destroy();
                this.R6.destroy();
                this.R7.destroy();
                this.R8.destroy();
                this.R9.destroy();
            }
        }
        impl<
            R0: RenderToDom,
            R1: RenderToDom,
            R2: RenderToDom,
            R3: RenderToDom,
            R4: RenderToDom,
            R5: RenderToDom,
            R6: RenderToDom,
            R7: RenderToDom,
            R8: RenderToDom,
            R9: RenderToDom,
        > RenderToDom for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9) {
            type Rendered = Tuple10Rendered<
                R0::Rendered,
                R1::Rendered,
                R2::Rendered,
                R3::Rendered,
                R4::Rendered,
                R5::Rendered,
                R6::Rendered,
                R7::Rendered,
                R8::Rendered,
                R9::Rendered,
            >;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
                R3::render_to_dom(
                    R3,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R3,
                    },
                );
                R4::render_to_dom(
                    R4,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R4,
                    },
                );
                R5::render_to_dom(
                    R5,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R5,
                    },
                );
                R6::render_to_dom(
                    R6,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R6,
                    },
                );
                R7::render_to_dom(
                    R7,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R7,
                    },
                );
                R8::render_to_dom(
                    R8,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R8,
                    },
                );
                R9::render_to_dom(
                    R9,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R9,
                    },
                );
            }
        }
        pub struct Tuple11Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> {
            R0: R0,
            R1: R1,
            R2: R2,
            R3: R3,
            R4: R4,
            R5: R5,
            R6: R6,
            R7: R7,
            R8: R8,
            R9: R9,
            R10: R10,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
            R3: ::core::default::Default,
            R4: ::core::default::Default,
            R5: ::core::default::Default,
            R6: ::core::default::Default,
            R7: ::core::default::Default,
            R8: ::core::default::Default,
            R9: ::core::default::Default,
            R10: ::core::default::Default,
        > ::core::default::Default
        for Tuple11Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> {
            #[inline]
            fn default() -> Tuple11Rendered<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
            > {
                Tuple11Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                    R3: ::core::default::Default::default(),
                    R4: ::core::default::Default::default(),
                    R5: ::core::default::Default::default(),
                    R6: ::core::default::Default::default(),
                    R7: ::core::default::Default::default(),
                    R8: ::core::default::Default::default(),
                    R9: ::core::default::Default::default(),
                    R10: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
            >
            where
                Tuple11Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin mut (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin mut (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin mut (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin mut (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin mut (R7)>,
                R8: ::pin_project_lite::__private::Pin<&'__pin mut (R8)>,
                R9: ::pin_project_lite::__private::Pin<&'__pin mut (R9)>,
                R10: ::pin_project_lite::__private::Pin<&'__pin mut (R10)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
            >
            where
                Tuple11Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10>: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin (R7)>,
                R8: ::pin_project_lite::__private::Pin<&'__pin (R8)>,
                R9: ::pin_project_lite::__private::Pin<&'__pin (R9)>,
                R10: ::pin_project_lite::__private::Pin<&'__pin (R10)>,
            }
            impl<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
            > Tuple11Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10 } = self
                            .get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                            R8: ::pin_project_lite::__private::Pin::new_unchecked(R8),
                            R9: ::pin_project_lite::__private::Pin::new_unchecked(R9),
                            R10: ::pin_project_lite::__private::Pin::new_unchecked(R10),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10 } = self
                            .get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                            R8: ::pin_project_lite::__private::Pin::new_unchecked(R8),
                            R9: ::pin_project_lite::__private::Pin::new_unchecked(R9),
                            R10: ::pin_project_lite::__private::Pin::new_unchecked(R10),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<'__pin, R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
                R3: R3,
                R4: R4,
                R5: R5,
                R6: R6,
                R7: R7,
                R8: R8,
                R9: R9,
                R10: R10,
            }
            impl<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
            > ::pin_project_lite::__private::Unpin
            for Tuple11Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10>
            where
                __Origin<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                >: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> MustNotImplDrop
            for Tuple11Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10>(
                this: &Tuple11Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10>,
            ) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
                let _ = &this.R3;
                let _ = &this.R4;
                let _ = &this.R5;
                let _ = &this.R6;
                let _ = &this.R7;
                let _ = &this.R8;
                let _ = &this.R9;
                let _ = &this.R10;
            }
        };
        impl<
            R0: Rendered,
            R1: Rendered,
            R2: Rendered,
            R3: Rendered,
            R4: Rendered,
            R5: Rendered,
            R6: Rendered,
            R7: Rendered,
            R8: Rendered,
            R9: Rendered,
            R10: Rendered,
        > Rendered for Tuple11Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                    R3: R3::new_uninitialized(),
                    R4: R4::new_uninitialized(),
                    R5: R5::new_uninitialized(),
                    R6: R6::new_uninitialized(),
                    R7: R7::new_uninitialized(),
                    R8: R8::new_uninitialized(),
                    R9: R9::new_uninitialized(),
                    R10: R10::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
                this.R3.destroy();
                this.R4.destroy();
                this.R5.destroy();
                this.R6.destroy();
                this.R7.destroy();
                this.R8.destroy();
                this.R9.destroy();
                this.R10.destroy();
            }
        }
        impl<
            R0: RenderToDom,
            R1: RenderToDom,
            R2: RenderToDom,
            R3: RenderToDom,
            R4: RenderToDom,
            R5: RenderToDom,
            R6: RenderToDom,
            R7: RenderToDom,
            R8: RenderToDom,
            R9: RenderToDom,
            R10: RenderToDom,
        > RenderToDom for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10) {
            type Rendered = Tuple11Rendered<
                R0::Rendered,
                R1::Rendered,
                R2::Rendered,
                R3::Rendered,
                R4::Rendered,
                R5::Rendered,
                R6::Rendered,
                R7::Rendered,
                R8::Rendered,
                R9::Rendered,
                R10::Rendered,
            >;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
                R3::render_to_dom(
                    R3,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R3,
                    },
                );
                R4::render_to_dom(
                    R4,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R4,
                    },
                );
                R5::render_to_dom(
                    R5,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R5,
                    },
                );
                R6::render_to_dom(
                    R6,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R6,
                    },
                );
                R7::render_to_dom(
                    R7,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R7,
                    },
                );
                R8::render_to_dom(
                    R8,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R8,
                    },
                );
                R9::render_to_dom(
                    R9,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R9,
                    },
                );
                R10::render_to_dom(
                    R10,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R10,
                    },
                );
            }
        }
        pub struct Tuple12Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11> {
            R0: R0,
            R1: R1,
            R2: R2,
            R3: R3,
            R4: R4,
            R5: R5,
            R6: R6,
            R7: R7,
            R8: R8,
            R9: R9,
            R10: R10,
            R11: R11,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
            R3: ::core::default::Default,
            R4: ::core::default::Default,
            R5: ::core::default::Default,
            R6: ::core::default::Default,
            R7: ::core::default::Default,
            R8: ::core::default::Default,
            R9: ::core::default::Default,
            R10: ::core::default::Default,
            R11: ::core::default::Default,
        > ::core::default::Default
        for Tuple12Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11> {
            #[inline]
            fn default() -> Tuple12Rendered<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
            > {
                Tuple12Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                    R3: ::core::default::Default::default(),
                    R4: ::core::default::Default::default(),
                    R5: ::core::default::Default::default(),
                    R6: ::core::default::Default::default(),
                    R7: ::core::default::Default::default(),
                    R8: ::core::default::Default::default(),
                    R9: ::core::default::Default::default(),
                    R10: ::core::default::Default::default(),
                    R11: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
            >
            where
                Tuple12Rendered<
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                >: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin mut (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin mut (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin mut (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin mut (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin mut (R7)>,
                R8: ::pin_project_lite::__private::Pin<&'__pin mut (R8)>,
                R9: ::pin_project_lite::__private::Pin<&'__pin mut (R9)>,
                R10: ::pin_project_lite::__private::Pin<&'__pin mut (R10)>,
                R11: ::pin_project_lite::__private::Pin<&'__pin mut (R11)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
            >
            where
                Tuple12Rendered<
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                >: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin (R7)>,
                R8: ::pin_project_lite::__private::Pin<&'__pin (R8)>,
                R9: ::pin_project_lite::__private::Pin<&'__pin (R9)>,
                R10: ::pin_project_lite::__private::Pin<&'__pin (R10)>,
                R11: ::pin_project_lite::__private::Pin<&'__pin (R11)>,
            }
            impl<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
            > Tuple12Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                > {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11 } = self
                            .get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                            R8: ::pin_project_lite::__private::Pin::new_unchecked(R8),
                            R9: ::pin_project_lite::__private::Pin::new_unchecked(R9),
                            R10: ::pin_project_lite::__private::Pin::new_unchecked(R10),
                            R11: ::pin_project_lite::__private::Pin::new_unchecked(R11),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                > {
                    unsafe {
                        let Self { R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11 } = self
                            .get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                            R8: ::pin_project_lite::__private::Pin::new_unchecked(R8),
                            R9: ::pin_project_lite::__private::Pin::new_unchecked(R9),
                            R10: ::pin_project_lite::__private::Pin::new_unchecked(R10),
                            R11: ::pin_project_lite::__private::Pin::new_unchecked(R11),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
            > {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
                R3: R3,
                R4: R4,
                R5: R5,
                R6: R6,
                R7: R7,
                R8: R8,
                R9: R9,
                R10: R10,
                R11: R11,
            }
            impl<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
            > ::pin_project_lite::__private::Unpin
            for Tuple12Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11>
            where
                __Origin<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                >: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11> MustNotImplDrop
            for Tuple12Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
            >(this: &Tuple12Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11>) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
                let _ = &this.R3;
                let _ = &this.R4;
                let _ = &this.R5;
                let _ = &this.R6;
                let _ = &this.R7;
                let _ = &this.R8;
                let _ = &this.R9;
                let _ = &this.R10;
                let _ = &this.R11;
            }
        };
        impl<
            R0: Rendered,
            R1: Rendered,
            R2: Rendered,
            R3: Rendered,
            R4: Rendered,
            R5: Rendered,
            R6: Rendered,
            R7: Rendered,
            R8: Rendered,
            R9: Rendered,
            R10: Rendered,
            R11: Rendered,
        > Rendered
        for Tuple12Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                    R3: R3::new_uninitialized(),
                    R4: R4::new_uninitialized(),
                    R5: R5::new_uninitialized(),
                    R6: R6::new_uninitialized(),
                    R7: R7::new_uninitialized(),
                    R8: R8::new_uninitialized(),
                    R9: R9::new_uninitialized(),
                    R10: R10::new_uninitialized(),
                    R11: R11::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
                this.R3.destroy();
                this.R4.destroy();
                this.R5.destroy();
                this.R6.destroy();
                this.R7.destroy();
                this.R8.destroy();
                this.R9.destroy();
                this.R10.destroy();
                this.R11.destroy();
            }
        }
        impl<
            R0: RenderToDom,
            R1: RenderToDom,
            R2: RenderToDom,
            R3: RenderToDom,
            R4: RenderToDom,
            R5: RenderToDom,
            R6: RenderToDom,
            R7: RenderToDom,
            R8: RenderToDom,
            R9: RenderToDom,
            R10: RenderToDom,
            R11: RenderToDom,
        > RenderToDom for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11) {
            type Rendered = Tuple12Rendered<
                R0::Rendered,
                R1::Rendered,
                R2::Rendered,
                R3::Rendered,
                R4::Rendered,
                R5::Rendered,
                R6::Rendered,
                R7::Rendered,
                R8::Rendered,
                R9::Rendered,
                R10::Rendered,
                R11::Rendered,
            >;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
                R3::render_to_dom(
                    R3,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R3,
                    },
                );
                R4::render_to_dom(
                    R4,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R4,
                    },
                );
                R5::render_to_dom(
                    R5,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R5,
                    },
                );
                R6::render_to_dom(
                    R6,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R6,
                    },
                );
                R7::render_to_dom(
                    R7,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R7,
                    },
                );
                R8::render_to_dom(
                    R8,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R8,
                    },
                );
                R9::render_to_dom(
                    R9,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R9,
                    },
                );
                R10::render_to_dom(
                    R10,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R10,
                    },
                );
                R11::render_to_dom(
                    R11,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R11,
                    },
                );
            }
        }
        pub struct Tuple13Rendered<
            R0,
            R1,
            R2,
            R3,
            R4,
            R5,
            R6,
            R7,
            R8,
            R9,
            R10,
            R11,
            R12,
        > {
            R0: R0,
            R1: R1,
            R2: R2,
            R3: R3,
            R4: R4,
            R5: R5,
            R6: R6,
            R7: R7,
            R8: R8,
            R9: R9,
            R10: R10,
            R11: R11,
            R12: R12,
        }
        #[automatically_derived]
        impl<
            R0: ::core::default::Default,
            R1: ::core::default::Default,
            R2: ::core::default::Default,
            R3: ::core::default::Default,
            R4: ::core::default::Default,
            R5: ::core::default::Default,
            R6: ::core::default::Default,
            R7: ::core::default::Default,
            R8: ::core::default::Default,
            R9: ::core::default::Default,
            R10: ::core::default::Default,
            R11: ::core::default::Default,
            R12: ::core::default::Default,
        > ::core::default::Default
        for Tuple13Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12> {
            #[inline]
            fn default() -> Tuple13Rendered<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
                R12,
            > {
                Tuple13Rendered {
                    R0: ::core::default::Default::default(),
                    R1: ::core::default::Default::default(),
                    R2: ::core::default::Default::default(),
                    R3: ::core::default::Default::default(),
                    R4: ::core::default::Default::default(),
                    R5: ::core::default::Default::default(),
                    R6: ::core::default::Default::default(),
                    R7: ::core::default::Default::default(),
                    R8: ::core::default::Default::default(),
                    R9: ::core::default::Default::default(),
                    R10: ::core::default::Default::default(),
                    R11: ::core::default::Default::default(),
                    R12: ::core::default::Default::default(),
                }
            }
        }
        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct Projection<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
                R12,
            >
            where
                Tuple13Rendered<
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                    R12,
                >: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin mut (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin mut (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin mut (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin mut (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin mut (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin mut (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin mut (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin mut (R7)>,
                R8: ::pin_project_lite::__private::Pin<&'__pin mut (R8)>,
                R9: ::pin_project_lite::__private::Pin<&'__pin mut (R9)>,
                R10: ::pin_project_lite::__private::Pin<&'__pin mut (R10)>,
                R11: ::pin_project_lite::__private::Pin<&'__pin mut (R11)>,
                R12: ::pin_project_lite::__private::Pin<&'__pin mut (R12)>,
            }
            #[allow(dead_code)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::mut_mut)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::ref_option_ref)]
            #[allow(clippy::type_repetition_in_bounds)]
            pub(crate) struct ProjectionRef<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
                R12,
            >
            where
                Tuple13Rendered<
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                    R12,
                >: '__pin,
            {
                R0: ::pin_project_lite::__private::Pin<&'__pin (R0)>,
                R1: ::pin_project_lite::__private::Pin<&'__pin (R1)>,
                R2: ::pin_project_lite::__private::Pin<&'__pin (R2)>,
                R3: ::pin_project_lite::__private::Pin<&'__pin (R3)>,
                R4: ::pin_project_lite::__private::Pin<&'__pin (R4)>,
                R5: ::pin_project_lite::__private::Pin<&'__pin (R5)>,
                R6: ::pin_project_lite::__private::Pin<&'__pin (R6)>,
                R7: ::pin_project_lite::__private::Pin<&'__pin (R7)>,
                R8: ::pin_project_lite::__private::Pin<&'__pin (R8)>,
                R9: ::pin_project_lite::__private::Pin<&'__pin (R9)>,
                R10: ::pin_project_lite::__private::Pin<&'__pin (R10)>,
                R11: ::pin_project_lite::__private::Pin<&'__pin (R11)>,
                R12: ::pin_project_lite::__private::Pin<&'__pin (R12)>,
            }
            impl<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
                R12,
            > Tuple13Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12> {
                pub(crate) fn project<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                ) -> Projection<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                    R12,
                > {
                    unsafe {
                        let Self {
                            R0,
                            R1,
                            R2,
                            R3,
                            R4,
                            R5,
                            R6,
                            R7,
                            R8,
                            R9,
                            R10,
                            R11,
                            R12,
                        } = self.get_unchecked_mut();
                        Projection {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                            R8: ::pin_project_lite::__private::Pin::new_unchecked(R8),
                            R9: ::pin_project_lite::__private::Pin::new_unchecked(R9),
                            R10: ::pin_project_lite::__private::Pin::new_unchecked(R10),
                            R11: ::pin_project_lite::__private::Pin::new_unchecked(R11),
                            R12: ::pin_project_lite::__private::Pin::new_unchecked(R12),
                        }
                    }
                }
                pub(crate) fn project_ref<'__pin>(
                    self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                ) -> ProjectionRef<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                    R12,
                > {
                    unsafe {
                        let Self {
                            R0,
                            R1,
                            R2,
                            R3,
                            R4,
                            R5,
                            R6,
                            R7,
                            R8,
                            R9,
                            R10,
                            R11,
                            R12,
                        } = self.get_ref();
                        ProjectionRef {
                            R0: ::pin_project_lite::__private::Pin::new_unchecked(R0),
                            R1: ::pin_project_lite::__private::Pin::new_unchecked(R1),
                            R2: ::pin_project_lite::__private::Pin::new_unchecked(R2),
                            R3: ::pin_project_lite::__private::Pin::new_unchecked(R3),
                            R4: ::pin_project_lite::__private::Pin::new_unchecked(R4),
                            R5: ::pin_project_lite::__private::Pin::new_unchecked(R5),
                            R6: ::pin_project_lite::__private::Pin::new_unchecked(R6),
                            R7: ::pin_project_lite::__private::Pin::new_unchecked(R7),
                            R8: ::pin_project_lite::__private::Pin::new_unchecked(R8),
                            R9: ::pin_project_lite::__private::Pin::new_unchecked(R9),
                            R10: ::pin_project_lite::__private::Pin::new_unchecked(R10),
                            R11: ::pin_project_lite::__private::Pin::new_unchecked(R11),
                            R12: ::pin_project_lite::__private::Pin::new_unchecked(R12),
                        }
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct __Origin<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
                R12,
            > {
                __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
                R0: R0,
                R1: R1,
                R2: R2,
                R3: R3,
                R4: R4,
                R5: R5,
                R6: R6,
                R7: R7,
                R8: R8,
                R9: R9,
                R10: R10,
                R11: R11,
                R12: R12,
            }
            impl<
                '__pin,
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
                R12,
            > ::pin_project_lite::__private::Unpin
            for Tuple13Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12>
            where
                __Origin<
                    '__pin,
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                    R12,
                >: ::pin_project_lite::__private::Unpin,
            {}
            trait MustNotImplDrop {}
            #[allow(clippy::drop_bounds, drop_bounds)]
            impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
            impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12> MustNotImplDrop
            for Tuple13Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12> {}
            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed<
                R0,
                R1,
                R2,
                R3,
                R4,
                R5,
                R6,
                R7,
                R8,
                R9,
                R10,
                R11,
                R12,
            >(
                this: &Tuple13Rendered<
                    R0,
                    R1,
                    R2,
                    R3,
                    R4,
                    R5,
                    R6,
                    R7,
                    R8,
                    R9,
                    R10,
                    R11,
                    R12,
                >,
            ) {
                let _ = &this.R0;
                let _ = &this.R1;
                let _ = &this.R2;
                let _ = &this.R3;
                let _ = &this.R4;
                let _ = &this.R5;
                let _ = &this.R6;
                let _ = &this.R7;
                let _ = &this.R8;
                let _ = &this.R9;
                let _ = &this.R10;
                let _ = &this.R11;
                let _ = &this.R12;
            }
        };
        impl<
            R0: Rendered,
            R1: Rendered,
            R2: Rendered,
            R3: Rendered,
            R4: Rendered,
            R5: Rendered,
            R6: Rendered,
            R7: Rendered,
            R8: Rendered,
            R9: Rendered,
            R10: Rendered,
            R11: Rendered,
            R12: Rendered,
        > Rendered
        for Tuple13Rendered<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12> {
            fn new_uninitialized() -> Self {
                Self {
                    R0: R0::new_uninitialized(),
                    R1: R1::new_uninitialized(),
                    R2: R2::new_uninitialized(),
                    R3: R3::new_uninitialized(),
                    R4: R4::new_uninitialized(),
                    R5: R5::new_uninitialized(),
                    R6: R6::new_uninitialized(),
                    R7: R7::new_uninitialized(),
                    R8: R8::new_uninitialized(),
                    R9: R9::new_uninitialized(),
                    R10: R10::new_uninitialized(),
                    R11: R11::new_uninitialized(),
                    R12: R12::new_uninitialized(),
                }
            }
            fn destroy(self: ::core::pin::Pin<&mut Self>) {
                let this = self.project();
                this.R0.destroy();
                this.R1.destroy();
                this.R2.destroy();
                this.R3.destroy();
                this.R4.destroy();
                this.R5.destroy();
                this.R6.destroy();
                this.R7.destroy();
                this.R8.destroy();
                this.R9.destroy();
                this.R10.destroy();
                this.R11.destroy();
                this.R12.destroy();
            }
        }
        impl<
            R0: RenderToDom,
            R1: RenderToDom,
            R2: RenderToDom,
            R3: RenderToDom,
            R4: RenderToDom,
            R5: RenderToDom,
            R6: RenderToDom,
            R7: RenderToDom,
            R8: RenderToDom,
            R9: RenderToDom,
            R10: RenderToDom,
            R11: RenderToDom,
            R12: RenderToDom,
        > RenderToDom for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12) {
            type Rendered = Tuple13Rendered<
                R0::Rendered,
                R1::Rendered,
                R2::Rendered,
                R3::Rendered,
                R4::Rendered,
                R5::Rendered,
                R6::Rendered,
                R7::Rendered,
                R8::Rendered,
                R9::Rendered,
                R10::Rendered,
                R11::Rendered,
                R12::Rendered,
            >;
            #[inline]
            fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                let current_rendered = ctx.current_rendered.project();
                let (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12) = self;
                R0::render_to_dom(
                    R0,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R0,
                    },
                );
                R1::render_to_dom(
                    R1,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R1,
                    },
                );
                R2::render_to_dom(
                    R2,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R2,
                    },
                );
                R3::render_to_dom(
                    R3,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R3,
                    },
                );
                R4::render_to_dom(
                    R4,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R4,
                    },
                );
                R5::render_to_dom(
                    R5,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R5,
                    },
                );
                R6::render_to_dom(
                    R6,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R6,
                    },
                );
                R7::render_to_dom(
                    R7,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R7,
                    },
                );
                R8::render_to_dom(
                    R8,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R8,
                    },
                );
                R9::render_to_dom(
                    R9,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R9,
                    },
                );
                R10::render_to_dom(
                    R10,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R10,
                    },
                );
                R11::render_to_dom(
                    R11,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R11,
                    },
                );
                R12::render_to_dom(
                    R12,
                    Context {
                        document: ctx.document,
                        current_parent: ctx.current_parent,
                        current_rendered: current_rendered.R12,
                    },
                );
            }
        }
    }
    pub use context::*;
    pub use render_to::*;
    pub use tuple::*;
    pub mod element {
        mod button {
            use gloo::events::EventListener;
            use wasm_bindgen::{JsCast, UnwrapThrowExt};
            use web_sys::HtmlButtonElement;
            use crate::dom::{Context, RenderToDom, Rendered};
            pub struct ButtonRendered<Children, OnClick> {
                node: Option<web_sys::HtmlButtonElement>,
                children: Children,
                on_click: Option<OnClick>,
            }
            #[allow(explicit_outlives_requirements)]
            #[allow(single_use_lifetimes)]
            #[allow(clippy::unknown_clippy_lints)]
            #[allow(clippy::redundant_pub_crate)]
            #[allow(clippy::used_underscore_binding)]
            const _: () = {
                #[allow(dead_code)]
                #[allow(single_use_lifetimes)]
                #[allow(clippy::unknown_clippy_lints)]
                #[allow(clippy::mut_mut)]
                #[allow(clippy::redundant_pub_crate)]
                #[allow(clippy::ref_option_ref)]
                #[allow(clippy::type_repetition_in_bounds)]
                pub(crate) struct Projection<'__pin, Children, OnClick>
                where
                    ButtonRendered<Children, OnClick>: '__pin,
                {
                    node: &'__pin mut (Option<web_sys::HtmlButtonElement>),
                    children: ::pin_project_lite::__private::Pin<&'__pin mut (Children)>,
                    on_click: &'__pin mut (Option<OnClick>),
                }
                #[allow(dead_code)]
                #[allow(single_use_lifetimes)]
                #[allow(clippy::unknown_clippy_lints)]
                #[allow(clippy::mut_mut)]
                #[allow(clippy::redundant_pub_crate)]
                #[allow(clippy::ref_option_ref)]
                #[allow(clippy::type_repetition_in_bounds)]
                pub(crate) struct ProjectionRef<'__pin, Children, OnClick>
                where
                    ButtonRendered<Children, OnClick>: '__pin,
                {
                    node: &'__pin (Option<web_sys::HtmlButtonElement>),
                    children: ::pin_project_lite::__private::Pin<&'__pin (Children)>,
                    on_click: &'__pin (Option<OnClick>),
                }
                impl<Children, OnClick> ButtonRendered<Children, OnClick> {
                    pub(crate) fn project<'__pin>(
                        self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
                    ) -> Projection<'__pin, Children, OnClick> {
                        unsafe {
                            let Self { node, children, on_click } = self
                                .get_unchecked_mut();
                            Projection {
                                node: node,
                                children: ::pin_project_lite::__private::Pin::new_unchecked(
                                    children,
                                ),
                                on_click: on_click,
                            }
                        }
                    }
                    pub(crate) fn project_ref<'__pin>(
                        self: ::pin_project_lite::__private::Pin<&'__pin Self>,
                    ) -> ProjectionRef<'__pin, Children, OnClick> {
                        unsafe {
                            let Self { node, children, on_click } = self.get_ref();
                            ProjectionRef {
                                node: node,
                                children: ::pin_project_lite::__private::Pin::new_unchecked(
                                    children,
                                ),
                                on_click: on_click,
                            }
                        }
                    }
                }
                #[allow(non_snake_case)]
                pub struct __Origin<'__pin, Children, OnClick> {
                    __dummy_lifetime: ::pin_project_lite::__private::PhantomData<
                        &'__pin (),
                    >,
                    node: ::pin_project_lite::__private::AlwaysUnpin<
                        Option<web_sys::HtmlButtonElement>,
                    >,
                    children: Children,
                    on_click: ::pin_project_lite::__private::AlwaysUnpin<
                        Option<OnClick>,
                    >,
                }
                impl<'__pin, Children, OnClick> ::pin_project_lite::__private::Unpin
                for ButtonRendered<Children, OnClick>
                where
                    __Origin<
                        '__pin,
                        Children,
                        OnClick,
                    >: ::pin_project_lite::__private::Unpin,
                {}
                trait MustNotImplDrop {}
                #[allow(clippy::drop_bounds, drop_bounds)]
                impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
                impl<Children, OnClick> MustNotImplDrop
                for ButtonRendered<Children, OnClick> {}
                #[forbid(unaligned_references, safe_packed_borrows)]
                fn __assert_not_repr_packed<Children, OnClick>(
                    this: &ButtonRendered<Children, OnClick>,
                ) {
                    let _ = &this.node;
                    let _ = &this.children;
                    let _ = &this.on_click;
                }
            };
            impl<Children: Rendered, OnClick> Rendered
            for ButtonRendered<Children, OnClick> {
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
            pub fn button() -> Button<(), ()> {
                Button {
                    children: (),
                    on_click: (),
                }
            }
            impl<
                OnClick: FnMut(&web_sys::MouseEvent) + 'static,
                Children: RenderToDom,
            > RenderToDom for Button<Children, OnClick>
            where
                Children::Rendered: Default,
            {
                type Rendered = ButtonRendered<Children::Rendered, EventListener>;
                fn render_to_dom(self, ctx: crate::dom::Context<Self::Rendered>) {
                    let current_rendered = ctx.current_rendered.project();
                    let node = current_rendered
                        .node
                        .get_or_insert_with(|| {
                            let node = ctx
                                .document
                                .create_element("button")
                                .unwrap_throw();
                            ctx.current_parent.append_child(&node).unwrap_throw();
                            node.dyn_into::<HtmlButtonElement>().unwrap_throw()
                        });
                    let mut on_click = self.on_click;
                    *current_rendered
                        .on_click = Some(
                        EventListener::new(
                            node,
                            "click",
                            move |event| { on_click(event.dyn_ref().unwrap_throw()) },
                        ),
                    );
                    self.children
                        .render_to_dom(Context {
                            document: ctx.document,
                            current_parent: node,
                            current_rendered: current_rendered.children,
                        });
                }
            }
        }
        mod text {
            use std::borrow::Cow;
            use wasm_bindgen::{JsValue, UnwrapThrowExt};
            use crate::dom::{RenderToDom, Rendered};
            use super::super::Context;
            pub struct TextRendered {
                node: Option<web_sys::Text>,
                cached_data: Option<Cow<'static, str>>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for TextRendered {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "TextRendered",
                        "node",
                        &&self.node,
                        "cached_data",
                        &&self.cached_data,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for TextRendered {
                #[inline]
                fn default() -> TextRendered {
                    TextRendered {
                        node: ::core::default::Default::default(),
                        cached_data: ::core::default::Default::default(),
                    }
                }
            }
            impl Rendered for TextRendered {
                #[inline]
                fn new_uninitialized() -> Self {
                    Default::default()
                }
                fn destroy(self: std::pin::Pin<&mut Self>) {
                    let current_rendered = self.get_mut();
                    web_sys::console::log_2(
                        &{
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(&["destroy string"], &[]),
                            );
                            res
                        }
                            .into(),
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
            impl RenderToDom for &str {
                type Rendered = TextRendered;
                #[inline]
                fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                    render(self, ctx)
                }
            }
            impl RenderToDom for String {
                type Rendered = TextRendered;
                #[inline]
                fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                    render_static(Cow::Owned(self), ctx)
                }
            }
            impl RenderToDom for StaticText {
                type Rendered = TextRendered;
                #[inline]
                fn render_to_dom(self, ctx: Context<Self::Rendered>) {
                    render_static(Cow::Borrowed(self.0), ctx)
                }
            }
            pub struct StaticText(pub &'static str);
        }
        pub use button::*;
        pub use text::*;
    }
}
struct CounterInner {}
impl RenderToDom for CounterInner {
    type Rendered = ();
    fn render_to_dom(self, ctx: Context<Self::Rendered>) {
        let document = ctx.document.clone();
        let current_parent = ctx.current_parent.clone();
        let fut = Self::use_element()
            .into_render(document, current_parent, std::future::pending());
        wasm_bindgen_futures::spawn_local(fut);
    }
}
impl CounterInner {
    fn use_impl_render() -> impl for<'ctx> ::hooks::core::Hook<
        (AnyContext<'ctx>,),
    > + for<'hook, 'ctx> ::hooks::core::HookLifetime<
        'hook,
        (AnyContext<'ctx>,),
        &'hook (),
        Value = Context<'ctx, impl Rendered + 'static>,
    > + ::hooks::core::HookBounds<Bounds = ()> {
        struct __HookTypes<HooksImplTrait0: Rendered + 'static> {
            __: ::core::marker::PhantomData<
                (::core::marker::PhantomData<HooksImplTrait0>,),
            >,
        }
        impl<HooksImplTrait0: Rendered + 'static> ::hooks::core::HookBounds
        for __HookTypes<HooksImplTrait0> {
            type Bounds = ();
        }
        impl<
            'hook,
            'ctx,
            HooksImplTrait0: Rendered + 'static,
        > ::hooks::core::HookLifetime<'hook, (AnyContext<'ctx>,), &'hook ()>
        for __HookTypes<HooksImplTrait0> {
            type Value = Context<'ctx, HooksImplTrait0>;
        }
        {
            #[inline]
            fn _hooks_def_fn_hook<
                HooksImplTrait0: Rendered + 'static,
                __HooksData,
                __HooksPoll: ::core::ops::Fn(
                        ::core::pin::Pin<&mut __HooksData>,
                        &mut ::core::task::Context,
                    ) -> ::core::task::Poll<::core::primitive::bool>,
                __HooksUseHook: for<'hook, 'ctx> ::core::ops::Fn(
                        ::core::pin::Pin<&'hook mut __HooksData>,
                        (AnyContext<'ctx>,),
                    ) -> Context<'ctx, HooksImplTrait0>,
            >(
                hooks_data: __HooksData,
                hooks_poll: __HooksPoll,
                hooks_use_hook: __HooksUseHook,
            ) -> ::hooks::core::fn_hook::FnHook<
                __HooksData,
                __HooksPoll,
                __HooksUseHook,
                __HookTypes<HooksImplTrait0>,
            > {
                ::hooks::core::fn_hook::FnHook::<
                    __HooksData,
                    __HooksPoll,
                    __HooksUseHook,
                    __HookTypes<HooksImplTrait0>,
                >::new(hooks_data, hooks_poll, hooks_use_hook)
            }
            _hooks_def_fn_hook::<
                _,
                _,
                _,
                _,
            >(
                hooks::use_state(),
                ::hooks::core::HookPollNextUpdate::poll_next_update,
                |__hooks_hook_0, (ctx,)| {
                    let mut ctx = ctx.downcast_into().unwrap();
                    let (state, updater) = ::hooks::core::Hook::<
                        _,
                    >::use_hook(__hooks_hook_0, (0,));
                    let updater = updater.clone();
                    ctx.render(
                        dom::element::button()
                            .children({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["state = "],
                                        &[::core::fmt::ArgumentV1::new_display(&state)],
                                    ),
                                );
                                res
                            })
                            .on_click(move |_: &_| {
                                updater.replace_with_fn_box(|v| *v + 1)
                            }),
                    );
                    ctx
                },
            )
        }
    }
    fn use_render() -> impl ::hooks::core::Hook<
        (),
    > + for<'hook> ::hooks::core::HookLifetime<
        'hook,
        (),
        &'hook (),
        Value = impl RenderToDom,
    > + ::hooks::core::HookBounds<Bounds = ()> {
        struct __HookTypes<HooksImplTrait0: RenderToDom> {
            __: ::core::marker::PhantomData<
                (::core::marker::PhantomData<HooksImplTrait0>,),
            >,
        }
        impl<HooksImplTrait0: RenderToDom> ::hooks::core::HookBounds
        for __HookTypes<HooksImplTrait0> {
            type Bounds = ();
        }
        impl<
            'hook,
            HooksImplTrait0: RenderToDom,
        > ::hooks::core::HookLifetime<'hook, (), &'hook ()>
        for __HookTypes<HooksImplTrait0> {
            type Value = HooksImplTrait0;
        }
        ::hooks::core::fn_hook::new_fn_hook::<
            (),
            _,
            __HookTypes<_>,
        >(
            hooks::use_lazy_pinned_hook(),
            ::hooks::core::HookPollNextUpdate::poll_next_update,
            |__hooks_hook_0, (): ()| {
                let a = ::hooks::core::Hook::<
                    _,
                >::use_hook(__hooks_hook_0, (Self::use_element,));
                HookComp::new(a)
            },
        )
    }
    #[inline]
    fn use_element() -> HookElement<
        impl for<'c> Hook<
            (AnyContext<'c>,),
            Value = Context<'c, impl Rendered + 'static>,
        >,
    > {
        HookElement::new(Self::use_impl_render())
    }
}
struct Counter {}
impl Counter {
    fn use_impl_render() -> impl for<'ctx> ::hooks::core::Hook<
        (AnyContext<'ctx>,),
    > + for<'hook, 'ctx> ::hooks::core::HookLifetime<
        'hook,
        (AnyContext<'ctx>,),
        &'hook (),
        Value = Context<'ctx, impl Rendered + 'static>,
    > + ::hooks::core::HookBounds<Bounds = ()> {
        struct __HookTypes<HooksImplTrait0: Rendered + 'static> {
            __: ::core::marker::PhantomData<
                (::core::marker::PhantomData<HooksImplTrait0>,),
            >,
        }
        impl<HooksImplTrait0: Rendered + 'static> ::hooks::core::HookBounds
        for __HookTypes<HooksImplTrait0> {
            type Bounds = ();
        }
        impl<
            'hook,
            'ctx,
            HooksImplTrait0: Rendered + 'static,
        > ::hooks::core::HookLifetime<'hook, (AnyContext<'ctx>,), &'hook ()>
        for __HookTypes<HooksImplTrait0> {
            type Value = Context<'ctx, HooksImplTrait0>;
        }
        {
            #[inline]
            fn _hooks_def_fn_hook<
                HooksImplTrait0: Rendered + 'static,
                __HooksData,
                __HooksPoll: ::core::ops::Fn(
                        ::core::pin::Pin<&mut __HooksData>,
                        &mut ::core::task::Context,
                    ) -> ::core::task::Poll<::core::primitive::bool>,
                __HooksUseHook: for<'hook, 'ctx> ::core::ops::Fn(
                        ::core::pin::Pin<&'hook mut __HooksData>,
                        (AnyContext<'ctx>,),
                    ) -> Context<'ctx, HooksImplTrait0>,
            >(
                hooks_data: __HooksData,
                hooks_poll: __HooksPoll,
                hooks_use_hook: __HooksUseHook,
            ) -> ::hooks::core::fn_hook::FnHook<
                __HooksData,
                __HooksPoll,
                __HooksUseHook,
                __HookTypes<HooksImplTrait0>,
            > {
                ::hooks::core::fn_hook::FnHook::<
                    __HooksData,
                    __HooksPoll,
                    __HooksUseHook,
                    __HookTypes<HooksImplTrait0>,
                >::new(hooks_data, hooks_poll, hooks_use_hook)
            }
            _hooks_def_fn_hook::<
                _,
                _,
                _,
                _,
            >(
                hooks::use_mut_default::<i32>(),
                ::hooks::core::HookPollNextUpdate::poll_next_update,
                |__hooks_hook_0, (ctx,)| {
                    let mut ctx = ctx.downcast_into().unwrap();
                    let render_id = ::hooks::core::Hook::<
                        _,
                    >::use_hook(__hooks_hook_0, ());
                    web_sys::console::log_1(
                        &{
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["render_id = "],
                                    &[::core::fmt::ArgumentV1::new_display(&render_id)],
                                ),
                            );
                            res
                        }
                            .into(),
                    );
                    ctx.render((
                        CounterInner {},
                        CounterInner {},
                        if *render_id % 2 == 0 {
                            web_sys::console::log_1(&"render some".into());
                            Some({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["render_id = "],
                                        &[::core::fmt::ArgumentV1::new_display(&render_id)],
                                    ),
                                );
                                res
                            })
                        } else {
                            None
                        },
                    ));
                    *render_id += 1;
                    ctx
                },
            )
        }
    }
    #[inline]
    fn use_render() -> HookElement<
        impl for<'c> Hook<
            (AnyContext<'c>,),
            Value = Context<'c, impl Rendered + 'static>,
        >,
    > {
        HookElement::new(Self::use_impl_render())
    }
}
fn main() {
    wasm_bindgen_futures::spawn_local(async {
        let window = web_sys::window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let current_parent = document.get_element_by_id("main").unwrap_throw();
        Counter::use_render()
            .into_render(document, current_parent.into(), std::future::pending())
            .await;
    })
}
