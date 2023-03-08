use super::{SharedRef, SharedRefUninitialized};
use hooks_core::IntoHook;

/// use [`SharedRef`] with initial value.
pub struct UseSharedRef<T>(pub T);
pub use UseSharedRef as use_shared_ref;

hooks_core::impl_hook![
    type For<T> = UseSharedRef<T>;

    #[inline]
    fn into_hook(self) -> SharedRef<T> {
        SharedRef::new(self.0)
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}

    #[inline]
    fn h(self, hook: SharedRefUninitialized<T>) {
        hook.get_mut()
            .0
            .get_or_insert_with(move || self.into_hook())
    }
];

/// use [`SharedRef`] with an `FnOnce` which returns the initial value.
pub struct UseSharedRefWith<T, F: FnOnce() -> T>(pub F);
pub use UseSharedRefWith as use_shared_ref_with;

hooks_core::impl_hook![
    type For<T, F: FnOnce() -> T> = UseSharedRefWith<T, F>;

    #[inline]
    fn into_hook(self) -> SharedRef<T> {
        SharedRef::new(self.0())
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}

    #[inline]
    fn h(self, hook: SharedRefUninitialized<T>) {
        hook.get_mut()
            .0
            .get_or_insert_with(move || self.into_hook())
    }
];
