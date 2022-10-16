use hooks_core::{Hook, HookBounds, HookLifetime};

use crate::{EffectCleanup, EffectForNoneDependency};

pub struct EffectOnNextPoll<E: EffectForNoneDependency> {
    effect: Option<E>,
    cleanup: Option<E::Cleanup>,
}

impl<E: EffectForNoneDependency> Drop for EffectOnNextPoll<E> {
    fn drop(&mut self) {
        self.cleanup.take().map(EffectCleanup::cleanup);
    }
}

impl<E: EffectForNoneDependency> Unpin for EffectOnNextPoll<E> {}

impl<E: EffectForNoneDependency> HookBounds for EffectOnNextPoll<E> {
    type Bounds = Self;
}

impl<'hook, E: EffectForNoneDependency> HookLifetime<'hook> for EffectOnNextPoll<E> {
    type Value = ();
    type Args = (E,);
}

impl<E: EffectForNoneDependency> Hook for EffectOnNextPoll<E> {
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

    fn use_hook<'hook>(
        self: std::pin::Pin<&'hook mut Self>,
        (effect,): <Self as HookLifetime<'hook>>::Args,
    ) -> <Self as HookLifetime<'hook>>::Value
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
