use std::{marker::PhantomData, pin::Pin};

use hooks_core::{HookPollNextUpdate, HookUnmount, UpdateHookUninitialized};

pin_project_lite::pin_project![
    #[derive(Debug, Default)]
    pub struct UninitializedHook<U>
    where
        U: HookPollNextUpdate,
        U: HookUnmount,
        U: Default,
    {
        #[pin]
        uninitialized_hook: U,
    }
];

impl<U: HookPollNextUpdate + HookUnmount + Default> UninitializedHook<U> {
    #[inline]
    pub fn pin_project(self: Pin<&mut Self>) -> Pin<&mut U> {
        self.project().uninitialized_hook
    }
}

impl<U: HookPollNextUpdate + HookUnmount + Default> UninitializedHook<U> {
    #[inline]
    pub fn h<H: UpdateHookUninitialized<Uninitialized = U>>(
        self: Pin<&mut Self>,
        into_hook: H,
    ) -> hooks_core::Value![H::Hook] {
        into_hook.h(self.pin_project())
    }
}

hooks_core::impl_hook![
    type For<H: HookPollNextUpdate + HookUnmount + Default> = UninitializedHook<H>;

    fn unmount(self) {
        H::unmount(self.pin_project())
    }

    fn poll_next_update(self, cx: _) {
        <H as HookPollNextUpdate>::poll_next_update(self.pin_project(), cx)
    }

    #[inline(always)]
    fn use_hook(self) -> Pin<&'hook mut Self> {
        self
    }
];

pub struct UseUninitializedHook<U: HookPollNextUpdate + HookUnmount + Default>(PhantomData<U>);

hooks_core::impl_hook![
    type For<U: HookPollNextUpdate + HookUnmount + Default> = UseUninitializedHook<U>;
    #[inline]
    fn into_hook(self) -> UninitializedHook<U> {
        Default::default()
    }
    #[inline(always)]
    fn update_hook(self, _hook: _) {}
    #[inline(always)]
    fn h(self, hook: UninitializedHook<U>) {
        hook
    }
];

/// Use another hook lazily so you can use hooks conditionally.
///
/// See also [`use_lazy_pinned_hook`](crate::use_lazy_pinned_hook).
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
    let hook_effect = use_uninitialized_hook();
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
pub fn use_uninitialized_hook<U: HookPollNextUpdate + HookUnmount + Default>(
) -> UseUninitializedHook<U> {
    UseUninitializedHook(PhantomData)
}
