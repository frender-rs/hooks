pub trait EffectCleanup {
    fn cleanup(self);
}

impl<C: EffectCleanup> EffectCleanup for Option<C> {
    #[inline]
    fn cleanup(self) {
        if let Some(c) = self {
            c.cleanup()
        }
    }
}

impl EffectCleanup for () {
    #[inline]
    fn cleanup(self) {}
}

impl<F: FnOnce()> EffectCleanup for F {
    #[inline]
    fn cleanup(self) {
        self()
    }
}

pub trait EffectFor<Dep> {
    type Cleanup: EffectCleanup;

    #[must_use = "effect cleanups must be run or scheduled to run"]
    fn effect_for(self, arg: &Dep) -> Self::Cleanup;
}

impl<F, C, Dep> EffectFor<Dep> for F
where
    F: FnOnce(&Dep) -> C,
    C: EffectCleanup,
{
    type Cleanup = C;

    #[inline]
    fn effect_for(self, arg: &Dep) -> Self::Cleanup {
        self(arg)
    }
}

pub trait EffectForNoneDependency {
    type Cleanup: EffectCleanup;

    #[must_use = "effect cleanups must be run or scheduled to run"]
    fn effect_for_none_dep(self) -> Self::Cleanup;
}
