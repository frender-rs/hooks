use super::{EffectCleanup, EffectFor, EffectForNoneDependency};

pub struct Cleanup<C: EffectCleanup>(pub Option<C>);

impl<C: EffectCleanup> Drop for Cleanup<C> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.0.take() {
            cleanup.cleanup()
        }
    }
}

pub struct EffectInner<E, C: EffectCleanup> {
    pub cleanup: Cleanup<C>,
    pub effect: Option<E>,
}

impl<E, C: EffectCleanup> std::fmt::Debug for EffectInner<E, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PLACEHOLDER: Option<&str> = Some("..");
        f.debug_struct("EffectInner")
            .field("cleanup", &self.cleanup.0.as_ref().and(PLACEHOLDER))
            .field("effect", &self.effect.as_ref().and(PLACEHOLDER))
            .finish()
    }
}

impl<E, C: EffectCleanup> EffectInner<E, C> {
    #[inline]
    pub fn new_registered(effect: E) -> Self {
        Self {
            cleanup: Cleanup(None),
            effect: Some(effect),
        }
    }
    pub fn unmount(&mut self) {
        drop(std::mem::take(self).cleanup)
    }
}

impl<E, C: EffectCleanup> Default for EffectInner<E, C> {
    #[inline]
    fn default() -> Self {
        Self {
            cleanup: Cleanup(None),
            effect: None,
        }
    }
}

impl<E, C: EffectCleanup> EffectInner<E, C> {
    pub fn cleanup_and_effect_with(&mut self, run_effect: impl FnOnce(E) -> C) {
        let this = std::mem::take(self);
        drop(this.cleanup);
        if let Some(effect) = this.effect {
            *self = Self {
                cleanup: Cleanup(Some(run_effect(effect))),
                effect: None,
            };
        }
    }
}

impl<E, C: EffectCleanup> EffectInner<E, C> {
    #[inline]
    pub fn register_effect(&mut self, new_effect: E) {
        self.effect = Some(new_effect);
    }
}

impl<E: EffectForNoneDependency> EffectInner<E, E::Cleanup> {
    #[inline]
    pub fn cleanup_and_effect(&mut self) {
        self.cleanup_and_effect_with(EffectForNoneDependency::effect_for_none_dep)
    }
}

impl<E, C: EffectCleanup> EffectInner<E, C> {
    #[inline]
    pub fn cleanup_and_effect_for<Dep>(&mut self, dep: &Dep)
    where
        E: EffectFor<Dep, Cleanup = C>,
    {
        self.cleanup_and_effect_with(|effect| effect.effect_for(dep))
    }
}
