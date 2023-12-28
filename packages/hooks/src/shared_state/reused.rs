use std::pin::Pin;

use crate::{reused::ReusableHook, ShareValue};

use super::SharedState;

impl<T> ReusableHook for SharedState<T> {
    type ReusableHook = Self;

    type ReusableUninitialized = crate::utils::UninitializedHook<Self>;

    fn reuse_into_hook(&self) -> Self::ReusableHook {
        self.clone()
    }

    fn reuse_update_hook(&self, mut hook: Pin<&mut Self::ReusableHook>) {
        if !hook.equivalent_to(self) {
            hook.set(self.clone())
        }
    }

    fn reuse_h<'hook>(
        &self,
        hook: Pin<&'hook mut Self::ReusableUninitialized>,
    ) -> hooks_core::Value![Self::ReusableHook, 'hook] {
        hook.get_mut().use_with(
            || self.reuse_into_hook(),
            |hook| self.reuse_update_hook(hook),
        )
    }
}
