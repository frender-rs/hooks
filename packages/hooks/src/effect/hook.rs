use hooks_core::{Hook, HookBounds, HookLifetime};

use crate::EffectCleanup;

use super::EffectFor;

pub struct Effect<Dep: PartialEq, E: EffectFor<Dep>> {
    changed: bool,
    dep: Option<Dep>,
    effect: Option<E>,
    cleanup: Option<E::Cleanup>,
}

impl<Dep: PartialEq, E: EffectFor<Dep>> Drop for Effect<Dep, E> {
    fn drop(&mut self) {
        self.cleanup.take().map(EffectCleanup::cleanup);
    }
}

impl<Dep: PartialEq, E: EffectFor<Dep>> Unpin for Effect<Dep, E> {}

impl<Dep: PartialEq, E: EffectFor<Dep>> HookBounds for Effect<Dep, E> {
    type Bounds = Self;
}

impl<'hook, Dep: PartialEq, E: EffectFor<Dep>> HookLifetime<'hook> for Effect<Dep, E> {
    type Value = ();
    type Args = (E, Dep);
}

impl<Dep: PartialEq, E: EffectFor<Dep>> Hook for Effect<Dep, E> {
    fn poll_next_update(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let this = self.get_mut();

        if this.changed {
            this.changed = false;

            if let Some(cleanup) = this.cleanup.take() {
                cleanup.cleanup()
            }

            if let Some(dep) = &this.dep {
                if let Some(effect) = this.effect.take() {
                    let cleanup = effect.effect_for(dep);
                    this.cleanup = Some(cleanup);
                }
            }
        }

        std::task::Poll::Ready(false)
    }

    #[inline]
    fn use_hook<'hook>(
        self: std::pin::Pin<&'hook mut Self>,
        (effect, dep): <Self as HookLifetime<'hook>>::Args,
    ) -> <Self as HookLifetime<'hook>>::Value
    where
        Self: 'hook,
    {
        let this = self.get_mut();
        let dep = Some(dep);

        if this.dep != dep {
            this.dep = dep;
            this.changed = true;
            this.effect = Some(effect);
        }
    }
}

#[inline]
pub fn use_effect<Dep: PartialEq, E: EffectFor<Dep>>() -> Effect<Dep, E> {
    Effect {
        changed: false,
        dep: None,
        effect: None,
        cleanup: None,
    }
}
