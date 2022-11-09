use std::pin::Pin;

use super::{EffectCleanup, EffectForNoneDependency};

pub struct EffectOnce<E: EffectForNoneDependency> {
    /// - `None` means uninitialized
    /// - `Some(Err(effect))` means ready to effect
    /// - `Some(Ok(cleanup))` means effected and ready to cleanup
    inner: Option<Result<E::Cleanup, E>>,
}

impl<E: EffectForNoneDependency> Unpin for EffectOnce<E> {}

impl<E: EffectForNoneDependency> Default for EffectOnce<E> {
    #[inline]
    fn default() -> Self {
        Self { inner: None }
    }
}

impl<E: EffectForNoneDependency> std::fmt::Debug for EffectOnce<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EffectOnce")
            .field(&match &self.inner {
                Some(Err(_)) => "effect",
                Some(Ok(_)) => "effected",
                None => "uninitialized",
            })
            .finish()
    }
}

impl<E: EffectForNoneDependency> Drop for EffectOnce<E> {
    fn drop(&mut self) {
        if let Some(Ok(cleanup)) = self.inner.take() {
            cleanup.cleanup()
        }
    }
}

impl<E: EffectForNoneDependency> EffectOnce<E> {
    #[inline]
    pub fn use_hook_with(self: Pin<&mut Self>, get_effect: impl FnOnce() -> E) {
        let this = self.get_mut();
        if this.inner.is_none() {
            this.inner = Some(Err(get_effect()))
        }
    }

    #[inline]
    fn impl_poll(&mut self) -> std::task::Poll<bool> {
        if self.inner.is_none() {
            return std::task::Poll::Ready(true);
        }

        if let Some(Err(_)) = &self.inner {
            let effect = match self.inner.take().unwrap() {
                Err(effect) => effect,
                _ => unreachable!(),
            };
            let cleanup = effect.effect_for_none_dep();
            self.inner = Some(Ok(cleanup));
        }
        std::task::Poll::Ready(false)
    }
}

crate::utils::impl_hook! {
    impl [E: EffectForNoneDependency] for EffectOnce<E> {
        #[inline]
        poll_next_update(self) {
            self.get_mut().impl_poll()
        }
        #[inline]
        use_hook(self, effect: E) -> () {
            self.use_hook_with(move || effect)
        }
    }
}

pub struct EffectOnceWith<E: EffectForNoneDependency>(EffectOnce<E>);

impl<E: EffectForNoneDependency> Unpin for EffectOnceWith<E> {}

impl<E: EffectForNoneDependency> Default for EffectOnceWith<E> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<E: EffectForNoneDependency> std::fmt::Debug for EffectOnceWith<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EffectOnceWith").field(&self.0).finish()
    }
}

crate::utils::impl_hook! {
    impl [E: EffectForNoneDependency] for EffectOnceWith<E> {
        #[inline]
        poll_next_update(self) {
            self.get_mut().0.impl_poll()
        }
        #[inline]
        use_hook[F: FnOnce() -> E](self, get_effect: F) -> () {
            Pin::new(&mut self.get_mut().0).use_hook_with(get_effect)
        }
    }
}

#[inline]
pub fn use_effect_once<E: EffectForNoneDependency>() -> EffectOnce<E> {
    Default::default()
}

#[inline]
pub fn use_effect_once_with<E: EffectForNoneDependency>() -> EffectOnceWith<E> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use hooks_core::HookExt;

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

            let mut hook = super::use_effect_once_with();

            futures_lite::future::block_on(async {
                assert!(hook.next_value((move || effect,)).await.is_some());
                assert_eq!(effected.borrow().len(), 0);
                hook.use_hook((|| unreachable!(),));
                assert!(hook.next_value((|| unreachable!(),)).await.is_none());
                assert_eq!(*effected.borrow(), ["effected"]);
            });
        }

        assert_eq!(effected.into_inner(), ["effected", "cleaned"]);
    }
}
