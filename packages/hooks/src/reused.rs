use std::pin::Pin;

use hooks_core::{Hook, HookPollNextUpdate, HookUnmount};
pub use Reused as use_reused;

pub struct Reused<'a, H: ?Sized + ReusableHook>(pub &'a H);

hooks_core::impl_hook!(
    type For<'a, H: ?Sized + ReusableHook> = Reused<'a, H>;

    fn into_hook(self) -> H::ReusableHook {
        self.0.reuse_into_hook()
    }

    fn update_hook(self, mut hook: _) {
        self.0.reuse_update_hook(hook)
    }

    fn h(self, hook: H::ReusableUninitialized) {
        self.0.reuse_h(hook)
    }
);

pub trait ReusableHook {
    type ReusableHook: Hook;
    type ReusableUninitialized: HookPollNextUpdate + HookUnmount + Default;
    fn reuse_into_hook(&self) -> Self::ReusableHook;
    fn reuse_update_hook(&self, hook: Pin<&mut Self::ReusableHook>);
    fn reuse_h<'hook>(
        &self,
        hook: Pin<&'hook mut Self::ReusableUninitialized>,
    ) -> hooks_core::Value![Self::ReusableHook, 'hook];
}

pub trait ReusableHookExt: ReusableHook {
    fn use_reused(&self) -> Reused<'_, Self> {
        Reused(self)
    }
}

impl<H: ?Sized + ReusableHook> ReusableHookExt for H {}
