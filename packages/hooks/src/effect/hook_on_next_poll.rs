use crate::EffectForNoneDependency;

use super::inner::EffectInner;

pub struct EffectOnNextPoll<E: EffectForNoneDependency>(EffectInner<E, E::Cleanup>);

impl<E: EffectForNoneDependency> std::fmt::Debug for EffectOnNextPoll<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EffectOnNextPoll").field(&self.0).finish()
    }
}

impl<E: EffectForNoneDependency> Default for EffectOnNextPoll<E> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<E: EffectForNoneDependency> Unpin for EffectOnNextPoll<E> {}

crate::utils::impl_hook! {
    impl[E: EffectForNoneDependency] for EffectOnNextPoll<E> {
        #[inline]
        poll_next_update(self) {
            let this = self.get_mut();
            this.0.cleanup_and_effect();
            std::task::Poll::Ready(false)
        }
        #[inline]
        use_hook(self, effect: E) -> () {
            let this = self.get_mut();
            this.0.register_effect(effect)
        }
    }
}

/// Register an effect which will be run on next poll.
///
/// Note that [`EffectOnNextPoll::poll_next_update`](HookPollNextUpdate::poll_next_update)
/// always returns `Poll::Ready(false)`, indicating its inner state never changes.
#[inline]
pub fn use_effect_on_next_poll<E: EffectForNoneDependency>() -> EffectOnNextPoll<E> {
    Default::default()
}
