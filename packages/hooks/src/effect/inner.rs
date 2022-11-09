use crate::{EffectCleanup, EffectFor, EffectForNoneDependency};

enum EffectInnerImpl<E, C: EffectCleanup> {
    Unregistered,
    Registered(E),
    Effected { cleanup: C, effect: Option<E> },
}

impl<E, C: EffectCleanup> std::fmt::Debug for EffectInnerImpl<E, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unregistered => write!(f, "Unregistered"),
            Self::Registered(_) => f.debug_tuple("Registered").field(&"effect").finish(),
            Self::Effected { effect, .. } => f
                .debug_struct("Effected")
                .field("cleanup", &"..")
                .field("effect", &effect.as_ref().and(Some("..")))
                .finish(),
        }
    }
}

impl<E, C: EffectCleanup> EffectInnerImpl<E, C> {
    pub fn final_cleanup(self) {
        match self {
            EffectInnerImpl::Unregistered => {}
            EffectInnerImpl::Registered(_) => {}
            EffectInnerImpl::Effected { cleanup, .. } => cleanup.cleanup(),
        }
    }
}

impl<E, C: EffectCleanup> Default for EffectInnerImpl<E, C> {
    #[inline]
    fn default() -> Self {
        Self::Unregistered
    }
}

impl<E, C: EffectCleanup> EffectInnerImpl<E, C> {
    pub fn cleanup_and_effect_with(&mut self, run_effect: impl FnOnce(E) -> C) {
        let this = std::mem::take(self);
        match this {
            EffectInnerImpl::Unregistered => {}
            EffectInnerImpl::Registered(effect) => {
                *self = EffectInnerImpl::Effected {
                    cleanup: run_effect(effect),
                    effect: None,
                }
            }
            EffectInnerImpl::Effected { cleanup, effect } => {
                cleanup.cleanup();
                if let Some(effect) = effect {
                    *self = EffectInnerImpl::Effected {
                        cleanup: run_effect(effect),
                        effect: None,
                    }
                }
                //  else, self is already EffectInnerImpl::Unregistered
            }
        }
    }
}

pub struct EffectInner<E, C: EffectCleanup>(EffectInnerImpl<E, C>);

impl<E, C: EffectCleanup> std::fmt::Debug for EffectInner<E, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<E, C: EffectCleanup> Default for EffectInner<E, C> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<E, C: EffectCleanup> Drop for EffectInner<E, C> {
    #[inline]
    fn drop(&mut self) {
        let this = std::mem::take(&mut self.0);
        this.final_cleanup()
    }
}

impl<E, C: EffectCleanup> EffectInner<E, C> {
    #[inline]
    pub fn register_effect(&mut self, new_effect: E) {
        match &mut self.0 {
            this @ EffectInnerImpl::Unregistered => *this = EffectInnerImpl::Registered(new_effect),
            EffectInnerImpl::Registered(effect) => *effect = new_effect,
            EffectInnerImpl::Effected { effect, .. } => *effect = Some(new_effect),
        }
    }
}

impl<E: EffectForNoneDependency> EffectInner<E, E::Cleanup> {
    #[inline]
    pub fn cleanup_and_effect(&mut self) {
        self.0
            .cleanup_and_effect_with(EffectForNoneDependency::effect_for_none_dep)
    }
}

impl<E, C: EffectCleanup> EffectInner<E, C> {
    #[inline]
    pub fn cleanup_and_effect_for<Dep>(&mut self, dep: &Dep)
    where
        E: EffectFor<Dep, Cleanup = C>,
    {
        self.0
            .cleanup_and_effect_with(|effect| effect.effect_for(dep))
    }
}
