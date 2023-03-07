use std::fmt::Debug;

use hooks_core::{IntoHook, NonLendingHook, UpdateHook, UpdateHookUninitialized};

pub use UseDebug as use_debug;

pub struct UseDebug<I: IntoHook>(pub I);

hooks_core::impl_hook![
    type For<I: IntoHook> = UseDebug<I>;

    #[inline]
    fn into_hook(self) -> I::Hook {
        self.0.into_hook()
    }
];

hooks_core::impl_hook![
    type For<I: UpdateHook> = UseDebug<I>
        where __![I::Hook: Debug]: __;

    fn update_hook(self, mut hook: _) {
        eprintln!("[update_hook][start] {:?}", hook.as_mut());
        self.0.update_hook(hook.as_mut());
        eprintln!("[update_hook][ end ] {hook:?}");
    }
];

hooks_core::impl_hook![
    type For<I: UpdateHookUninitialized> = UseDebug<I>
        where __![I::Hook: Debug + NonLendingHook, I::Uninitialized: Debug]: __;

    fn h(self, mut hook: I::Uninitialized) {
        eprintln!("[h][start] {:?}", hook.as_mut());
        let v = self.0.h(hook.as_mut());
        eprintln!("[h][ end ] {:?}", hook);
        v
    }
];
