use std::pin::Pin;

use hooks_core::{Hook, HookPollNextUpdate};

pin_project_lite::pin_project! {
    #[derive(Debug, Default)]
    pub struct DefaultPinnedHook<H: HookPollNextUpdate> {
        #[pin]
        pub hook: H
    }
}

impl<H: HookPollNextUpdate> DefaultPinnedHook<H> {
    #[inline]
    pub fn pin_project_hook(self: Pin<&mut Self>) -> Pin<&mut H> {
        self.project().hook
    }
}

crate::utils::impl_hook! {
    impl [H: HookPollNextUpdate] for DefaultPinnedHook<H> {
        #[inline]
        poll_next_update(self, cx) {
            <H as HookPollNextUpdate>::poll_next_update(self.pin_project_hook(), cx)
        }
        #[inline]
        use_hook(self) -> Pin<&'hook mut H> {
            self.pin_project_hook()
        }
    }
}

impl<H: HookPollNextUpdate + Default, F: FnOnce() -> H> Hook<(F,)> for DefaultPinnedHook<H> {
    type Value<'hook> = Pin<&'hook mut H>
    where
        Self: 'hook;

    #[inline]
    fn use_hook<'hook>(self: Pin<&'hook mut Self>, _: (F,)) -> Self::Value<'hook>
    where
        Self: 'hook,
    {
        self.use_hook(())
    }
}

/// Use another hook by just [`poll_next_update`] without calling [`use_hook`].
/// This allows conditionally using hooks.
/// See also [`use_lazy_pinned_hook`](crate::use_lazy_pinned_hook).
///
/// [`poll_next_update`]: HookPollNextUpdate::poll_next_update
/// [`use_hook`]: crate::Hook::use_hook
///
/// ```
/// # use hooks::{hook, use_default_pinned_hook, use_state, Hook};
/// #[hook]
/// fn use_demo() -> i32 {
///     let (state, updater) = use_state(0);
///     let hook_effect = use_default_pinned_hook::<hooks::Effect<i32, _>>();
///     if *state < 2 {
///         let updater = updater.clone();
///         hook_effect.use_hook((move |v: &i32| updater.set(*v + 1), *state));
///     }
///     *state
/// }
/// ```
///
/// You can also pass a fn instead of specifying the hook type.
/// The fn is just used to infer the type, and it will not be called.
///
/// ```
/// # use hooks::{hook, use_default_pinned_hook, use_effect, use_state, Hook};
/// #[hook]
/// fn use_demo() -> i32 {
///     let (state, updater) = use_state(0);
///     let hook_effect = use_default_pinned_hook(use_effect);
///     if *state < 2 {
///         let updater = updater.clone();
///         hook_effect.use_hook((move |v: &i32| updater.set(*v + 1), *state));
///     }
///     *state
/// }
/// ```
#[inline]
pub fn use_default_pinned_hook<H: HookPollNextUpdate + Default>() -> DefaultPinnedHook<H> {
    Default::default()
}
