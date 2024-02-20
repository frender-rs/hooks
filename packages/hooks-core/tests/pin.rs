extern crate hooks_dev as hooks;
use std::{marker::PhantomPinned, pin::Pin};

use hooks_core::{hook_fn, Hook, IntoHook, UpdateHookUninitialized};

hook_fn!(
    fn use_pinned() -> Pin<&'hook mut PhantomPinned> {
        let v = h!(hooks::use_default_pinned::<PhantomPinned>());
        v
    }
);

hook_fn!(
    fn use_my_hook() {
        h!(use_pinned());
    }
);

#[test]
fn uninitialized() {
    let my_hook = use_my_hook();

    let uninitialized = Default::default();
    let uninitialized = std::pin::pin!(uninitialized);

    my_hook.h(uninitialized)
}

#[test]
fn uninitialized_is_not_unpin() {
    trait AssertNotUnpin<T> {
        fn assert_not_unpin(&self) {}
    }

    impl<T> AssertNotUnpin<()> for T {}
    impl<T: UpdateHookUninitialized> AssertNotUnpin<u8> for T where T::Uninitialized: Unpin {}

    use_my_hook().assert_not_unpin()
}

#[test]
fn into_hook() {
    let my_hook = use_my_hook();

    let hook = my_hook.into_hook();

    let hook = std::pin::pin!(hook);
    hook.use_hook()
}

#[test]
fn into_hook_is_not_unpin() {
    trait AssertNotUnpin<T> {
        fn assert_not_unpin(&self) {}
    }

    impl<T> AssertNotUnpin<()> for T {}
    impl<T: IntoHook> AssertNotUnpin<u8> for T where T::Hook: Unpin {}

    use_my_hook().assert_not_unpin()
}
