use hooks_core::{hook_fn, HookExt, IntoHook, UpdateHookUninitialized};

hook_fn!(
    fn use_my_hook() {}
);

fn assert_uninitialized_is_unpin<H>(_: &H)
where
    H: UpdateHookUninitialized,
    H::Uninitialized: Unpin,
{
}

fn assert_into_hook_is_unpin<H>(_: &H)
where
    H: IntoHook,
    H::Hook: Unpin,
{
}

fn assert_is_unpin<T: ?Sized + Unpin>(_: &T) {}

#[test]
fn uninitialized_is_unpin() {
    let my_hook = use_my_hook();
    assert_uninitialized_is_unpin(&my_hook);

    let mut uninitialized = Default::default();

    assert_is_unpin(&uninitialized);

    my_hook.h(std::pin::Pin::new(&mut uninitialized))
}

#[test]
fn into_hook_is_unpin() {
    let my_hook = use_my_hook();
    assert_into_hook_is_unpin(&my_hook);

    let mut hook = my_hook.into_hook();
    assert_is_unpin(&hook);
    hook.use_hook()
}
