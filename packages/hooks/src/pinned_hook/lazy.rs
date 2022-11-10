use std::pin::Pin;

use hooks_core::HookPollNextUpdate;

use crate::LazyPinned;

pin_project_lite::pin_project! {
    #[derive(Debug)]
    pub struct LazyPinnedHook<H: HookPollNextUpdate> {
        #[pin]
        pub hook: LazyPinned<H>,
    }
}

impl<H: HookPollNextUpdate> Default for LazyPinnedHook<H> {
    #[inline]
    fn default() -> Self {
        Self {
            hook: Default::default(),
        }
    }
}

impl<H: HookPollNextUpdate> LazyPinnedHook<H> {
    #[inline]
    pub fn pin_project_hook(self: Pin<&mut Self>) -> Option<Pin<&mut H>> {
        self.project().hook.as_pin_mut()
    }
}

crate::utils::impl_hook! {
    impl [H: HookPollNextUpdate] for LazyPinnedHook<H> {
        #[inline]
        poll_next_update(self, cx) {
            let hook = self.pin_project_hook();
            if let Some(hook) = hook {
                <H as HookPollNextUpdate>::poll_next_update(hook, cx)
            } else {
                true.into()
            }
        }

        #[inline]
        use_hook[F: FnOnce() -> H](self, get_initial_hook: F) -> Pin<&'hook mut H> {
            self.project().hook.use_hook_with(get_initial_hook)
        }
    }
}

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
#[inline]
pub fn use_lazy_pinned_hook<H: HookPollNextUpdate>() -> LazyPinnedHook<H> {
    Default::default()
}
