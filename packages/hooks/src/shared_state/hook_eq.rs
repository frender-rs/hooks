use super::SharedStateEq;

pub struct UseSharedStateEq<T: PartialEq>(pub T);
pub use UseSharedStateEq as use_shared_state_eq;

hooks_core::impl_hook![
    type For<T: PartialEq> = UseSharedStateEq<T>;
    #[inline]
    fn into_hook(self) -> SharedStateEq<T> {
        SharedStateEq::new(self.0)
    }
    #[inline(always)]
    fn update_hook(self, hook: _) {}
    fn h(self, hook: crate::utils::UninitializedHook<SharedStateEq<T>>) {
        hook.get_mut().use_into_hook(self)
    }
];

pub struct UseSharedStateEqWith<T, F: FnOnce() -> T>(pub F);
pub use UseSharedStateEqWith as use_shared_state_eq_with;

hooks_core::impl_hook![
    type For<T: PartialEq, F> = UseSharedStateEqWith<T,F>
        where __![F: FnOnce() -> T]: __;

    #[inline]
    fn into_hook(self) -> SharedStateEq<T> {
        SharedStateEq::new(self.0())
    }

    #[inline(always)]
    fn update_hook(self, hook: _) {}
    fn h(self, hook: crate::utils::UninitializedHook<SharedStateEq<T>>) {
        hook.get_mut().use_into_hook(self)
    }
];
