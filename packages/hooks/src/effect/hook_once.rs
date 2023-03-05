use std::pin::Pin;

use super::{inner::Cleanup, EffectCleanup, EffectForNoneDependency};

pub struct EffectOnce<E: EffectForNoneDependency> {
    /// - `Ok(Cleanup(None))` means uninitialized
    /// - `Err(effect)` means ready to effect
    /// - `Ok(Cleanup(Some(cleanup)))` means effected and ready to cleanup
    inner: Result<Cleanup<E::Cleanup>, E>,
}

impl<E: EffectForNoneDependency> Unpin for EffectOnce<E> {}

impl<E: EffectForNoneDependency> Default for EffectOnce<E> {
    #[inline]
    fn default() -> Self {
        Self {
            inner: Ok(Cleanup(None)),
        }
    }
}

impl<E: EffectForNoneDependency> std::fmt::Debug for EffectOnce<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EffectOnce")
            .field(&match &self.inner {
                Err(_) => "effect",
                Ok(Cleanup(Some(_))) => "effected",
                Ok(Cleanup(None)) => "uninitialized",
            })
            .finish()
    }
}

impl<E: EffectForNoneDependency> EffectOnce<E> {
    pub fn register_effect_with(&mut self, get_effect: impl FnOnce() -> E) {
        let inner = &mut self.inner;
        if let Ok(Cleanup(None)) = &inner {
            *inner = Err(get_effect())
        }
    }

    #[inline]
    fn impl_poll(&mut self) -> std::task::Poll<bool> {
        let inner = &mut self.inner;
        if let Err(_) = &inner {
            let effect = match std::mem::replace(inner, Ok(Cleanup(None))) {
                Err(effect) => effect,
                _ => unreachable!(),
            };
            let cleanup = effect.effect_for_none_dep();
            *inner = Ok(Cleanup(Some(cleanup)));
        }
        std::task::Poll::Ready(false)
    }
}

crate::utils::impl_hook![
    type For<E: EffectForNoneDependency> = EffectOnce<E>;
    #[inline]
    fn unmount(self) {
        drop(std::mem::take(self.get_mut()))
    }
    #[inline]
    fn poll_next_update(self) {
        self.get_mut().impl_poll()
    }
    #[inline]
    fn use_hook(self) -> () {}
];

pub struct UseEffectOnce<E>(pub E);
pub use UseEffectOnce as use_effect_once;

crate::utils::impl_hook![
    type For<E: EffectForNoneDependency> = UseEffectOnce<E>;
    #[inline]
    fn into_hook(self) -> EffectOnce<E> {
        EffectOnce { inner: Err(self.0) }
    }
    #[inline]
    fn update_hook(self, hook: _) {
        hook.get_mut().register_effect_with(move || self.0)
    }
    #[inline]
    fn h(self, hook: EffectOnce<E>) {
        hooks_core::UpdateHook::update_hook(self, hook)
    }
];

pub struct UseEffectOnceWith<F>(pub F);

#[inline(always)]
pub fn use_effect_once_with<E: EffectForNoneDependency>(
    get_effect: impl FnOnce() -> E,
) -> UseEffectOnceWith<impl FnOnce() -> E> {
    UseEffectOnceWith(get_effect)
}

crate::utils::impl_hook![
    type For<E: EffectForNoneDependency, F> = UseEffectOnceWith<F>
        where __![F: FnOnce() -> E] : __;

    #[inline]
    fn into_hook(self) -> EffectOnce<E> {
        EffectOnce {
            inner: Err(self.0()),
        }
    }
    #[inline]
    fn update_hook(self, hook: _) {
        hook.get_mut().register_effect_with(self.0)
    }
    #[inline]
    fn h(self, hook: EffectOnce<E>) {
        hooks_core::UpdateHook::update_hook(self, hook)
    }
];

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use hooks_core::{HookExt, IntoHook};

    #[test]
    fn effect_once() {
        let effected = RefCell::new(Vec::with_capacity(2));

        {
            let effect = || {
                let mut e = effected.borrow_mut();
                assert!(e.is_empty());
                e.push("effected");

                || effected.borrow_mut().push("cleaned")
            };

            let hook = super::use_effect_once_with(effect).into_hook();

            futures_lite::pin!(hook);

            futures_lite::future::block_on(async {
                assert!(hook.next_value().await.is_some());
                assert_eq!(effected.borrow().len(), 0);
                hook.use_hook();
                assert!(hook.next_value().await.is_none());
                assert_eq!(*effected.borrow(), ["effected"]);
            });
        }

        assert_eq!(effected.into_inner(), ["effected", "cleaned"]);
    }
}
