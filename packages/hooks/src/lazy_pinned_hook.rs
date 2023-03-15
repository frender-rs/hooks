use std::{marker::PhantomData, pin::Pin};

use hooks_core::{Hook, HookPollNextUpdate, HookUnmount, IntoHook, UpdateHook};

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
impl<H: Hook> LazyPinnedHook<H> {
    pub fn h(self: Pin<&mut Self>, into_hook: impl UpdateHook<Hook = H>) -> hooks_core::Value![H] {
        self.pin_project()
            .use_pin_or_insert_with_data(
                into_hook,
                |into_hook, hook| into_hook.update_hook(hook),
                IntoHook::into_hook,
            )
            .use_hook()
    }
}

hooks_core::impl_hook![
    type For<H: HookPollNextUpdate + HookUnmount> = LazyPinnedHook<H>;

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
            std::task::Poll::Ready(false)
        }
    }

    #[inline(always)]
    fn use_hook(self) -> Pin<&'hook mut Self> {
        self
    }
];

pub struct UseLazyPinnedHook<H: Hook>(PhantomData<H>);

/// Use another hook lazily so you can use hooks conditionally.
///
/// See also [`use_uninitialized_hook`](crate::use_uninitialized_hook).
///
/// The following code compiles but `use_effect()` actually does nothing
/// because `#[hook]` doesn't know the `use_effect` in `if` branch is a hook call.
///
#[cfg_attr(
    all(
        feature = "futures-core",
        feature = "proc-macro",
        feature = "use_state",
        feature = "use_effect",
    ),
    doc = r###"
```
# use hooks::prelude::*;
#[hook]
fn use_demo() -> i32 {
    let (state, updater) = use_state(0);
    if *state < 2 {
        use_effect(|v: &i32| updater.set(*v + 1), *state);
    }
    *state
}

# use futures_lite::StreamExt;
# futures_lite::future::block_on(async {
let values: Vec<_> = use_demo().into_hook_values().collect().await;
assert_eq!(values, [0])
# });
```
"###
)]
///
/// With [`use_lazy_pinned_hook`], we can call `use_effect` conditionally:
///
#[cfg_attr(
    all(
        feature = "futures-core",
        feature = "proc-macro",
        feature = "use_state",
        feature = "use_effect",
    ),
    doc = r###"
```
# use hooks::prelude::*;
#[hook]
fn use_demo() -> i32 {
    let (state, updater) = use_state(0);
    let hook_effect = use_lazy_pinned_hook();
    if *state < 2 {
        let updater = updater.clone();
        hook_effect.h(use_effect(move |v: &i32| updater.set(*v + 1), *state));
    }
    *state
}

# use futures_lite::StreamExt;
# futures_lite::future::block_on(async {
let values: Vec<_> = use_demo().into_hook_values().collect().await;
assert_eq!(values, [0, 1, 2])
# });
```
"###
)]
#[inline(always)]
pub fn use_lazy_pinned_hook<H: Hook>() -> UseLazyPinnedHook<H> {
    UseLazyPinnedHook(PhantomData)
}

hooks_core::impl_hook![
    type For<H: Hook> = UseLazyPinnedHook<H>;
    #[inline]
    fn into_hook(self) -> LazyPinnedHook<H> {
        LazyPinnedHook::default()
    }

    #[inline]
    fn update_hook(self, _hook: _) {}

    #[inline]
    fn h(self, hook: LazyPinnedHook<H>) {
        hook
    }
];
