use std::pin::Pin;

use hooks_core::{HookPollNextUpdate, HookUnmount, IntoHook, UpdateHook};

pin_project_lite::pin_project![
    #[derive(Debug)]
    pub struct LazyPinnedHook<H>
    where
        H: HookPollNextUpdate,
        H: HookUnmount,
    {
        #[pin]
        hook: lazy_pinned::LazyPinned<H>,
    }
];

impl<H: HookPollNextUpdate + HookUnmount> Default for LazyPinnedHook<H> {
    fn default() -> Self {
        Self {
            hook: Default::default(),
        }
    }
}

impl<H: HookPollNextUpdate + HookUnmount> LazyPinnedHook<H> {
    #[inline]
    pub fn pin_project(self: Pin<&mut Self>) -> Pin<&mut lazy_pinned::LazyPinned<H>> {
        self.project().hook
    }

    #[inline]
    pub fn pin_project_hook(self: Pin<&mut Self>) -> Option<Pin<&mut H>> {
        self.pin_project().as_pin_mut()
    }
}

hooks_core::impl_hook![
    type For<H> = LazyPinnedHook<H>
        where __![H: HookPollNextUpdate + HookUnmount]: __;

    fn unmount(self) {
        if let Some(hook) = self.pin_project_hook() {
            H::unmount(hook)
        }
    }

    fn poll_next_update(self, cx: _) {
        let hook = self.pin_project_hook();
        if let Some(hook) = hook {
            <H as HookPollNextUpdate>::poll_next_update(hook, cx)
        } else {
            std::task::Poll::Ready(true)
        }
    }
];

/// Use another hook lazily so you can use hooks conditionally.
///
/// [`#[hook]`](crate::hook) macro works by interpreting
/// top level `use_*` method calls as hook calls.
/// Thus, using hook in a `if` branch will not be interpreted by this macro.
///
/// [`use_lazy_pinned_hook`] and [`use_default_pinned_hook`](crate::use_default_pinned_hook)
/// can solve the above problem.
///
/// The following code fails to compile because `#[hook]` doesn't know
/// `use_effect` in `if` branch is a hook call.
///
/// ```compile_fail
/// # use hooks::{hook, use_effect, use_state};
/// #[hook]
/// fn use_demo() -> i32 {
///     let (state, updater) = use_state(0);
///     if *state < 2 {
///         use_effect(|v: &i32| updater.set(*v + 1), *state);
///     }
///     *state
/// }
/// ```
///
/// With [`use_lazy_pinned_hook`], we can call `use_effect` conditionally:
///
/// ```
/// # use hooks::{hook, use_lazy_pinned_hook, use_effect, use_state, Hook};
/// #[hook]
/// fn use_demo() -> i32 {
///     let (state, updater) = use_state(0);
///     let hook_effect = use_lazy_pinned_hook(use_effect);
///     if *state < 2 {
///         let updater = updater.clone();
///         hook_effect.use_hook((move |v: &i32| updater.set(*v + 1), *state));
///     }
///     *state
/// }
/// ```
pub struct UseLazyPinnedHook<I: IntoHook>(pub I);
pub use UseLazyPinnedHook as use_lazy_pinned_hook;

hooks_core::impl_hook![
    type For<I: IntoHook> = UseLazyPinnedHook<I>;
    #[inline]
    fn into_hook(self) -> I::Hook {
        I::into_hook(self.0)
    }
];

hooks_core::impl_hook![
    type For<I: UpdateHook> = UseLazyPinnedHook<I>;
    #[inline]
    fn update_hook(self, hook: _) {
        I::update_hook(self.0, hook)
    }

    fn h(self, hook: LazyPinnedHook<I::Hook>) {
        let hook = hook
            .pin_project()
            .pin_project_or_insert_with(move || self.0.into_hook());
        <I::Hook as hooks_core::Hook>::use_hook(hook)
    }
];
