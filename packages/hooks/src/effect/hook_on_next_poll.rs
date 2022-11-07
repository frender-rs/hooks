use hooks_core::{Hook, HookBounds, HookLifetime, HookPollNextUpdate};

use crate::{EffectCleanup, EffectForNoneDependency};

pub struct EffectOnNextPoll<E: EffectForNoneDependency> {
    effect: Option<E>,
    cleanup: Option<E::Cleanup>,
}

impl<E: EffectForNoneDependency> std::fmt::Debug for EffectOnNextPoll<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EffectOnNextPoll")
            .field("effect", &self.effect.as_ref().and(Some("effect")))
            .field("cleanup", &self.cleanup.as_ref().and(Some("cleanup")))
            .finish()
    }
}

impl<E: EffectForNoneDependency> Default for EffectOnNextPoll<E> {
    #[inline]
    fn default() -> Self {
        Self {
            effect: None,
            cleanup: None,
        }
    }
}

impl<E: EffectForNoneDependency> Drop for EffectOnNextPoll<E> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup.cleanup()
        }
    }
}

impl<E: EffectForNoneDependency> Unpin for EffectOnNextPoll<E> {}

impl<E: EffectForNoneDependency> HookBounds for EffectOnNextPoll<E> {
    type Bounds = Self;
}

impl<'hook, E: EffectForNoneDependency> HookLifetime<'hook, (E,)> for EffectOnNextPoll<E> {
    type Value = ();
}

impl<E: EffectForNoneDependency> HookPollNextUpdate for EffectOnNextPoll<E> {
    fn poll_next_update(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let this = self.get_mut();

        if let Some(cleanup) = this.cleanup.take() {
            cleanup.cleanup()
        }

        if let Some(effect) = this.effect.take() {
            let cleanup = effect.effect_for_none_dep();
            this.cleanup = Some(cleanup);
        }

        std::task::Poll::Ready(false)
    }
}

impl<E: EffectForNoneDependency> Hook<(E,)> for EffectOnNextPoll<E> {
    fn use_hook<'hook>(
        self: std::pin::Pin<&'hook mut Self>,
        (effect,): (E,),
    ) -> <Self as HookLifetime<'hook, (E,)>>::Value
    where
        Self: 'hook,
    {
        let this = self.get_mut();
        this.effect = Some(effect);
    }
}

#[inline]
pub fn use_effect_on_next_poll<E: EffectForNoneDependency>() -> EffectOnNextPoll<E> {
    EffectOnNextPoll {
        effect: None,
        cleanup: None,
    }
}
