use super::{inner::EffectInner, EffectForNoneDependency};

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

hooks_core::impl_hook![
    type For<E: EffectForNoneDependency> = EffectOnNextPoll<E>;
    #[inline]
    fn unmount(self) {
        self.get_mut().0.unmount()
    }
    #[inline]
    fn poll_next_update(self) {
        let this = self.get_mut();
        this.0.cleanup_and_effect();
        std::task::Poll::Ready(false)
    }
    #[inline(always)]
    fn use_hook(self) -> () {}
];

/// Register an effect which will be run on next poll.
///
/// Note that [`EffectOnNextPoll::poll_next_update`](crate::HookPollNextUpdate::poll_next_update)
/// always returns `Poll::Ready(false)`, indicating its inner state never changes.
pub struct UseEffectOnNextPoll<E: EffectForNoneDependency>(pub E);
pub use UseEffectOnNextPoll as use_effect_on_next_poll;

hooks_core::impl_hook![
    type For<E: EffectForNoneDependency> = UseEffectOnNextPoll<E>;
    #[inline]
    fn into_hook(self) -> EffectOnNextPoll<E> {
        EffectOnNextPoll(EffectInner::new_registered(self.0))
    }
    #[inline]
    fn update_hook(self, hook: _) {
        hook.get_mut().0.register_effect(self.0)
    }
    #[inline]
    fn h(self, hook: EffectOnNextPoll<E>) {
        hooks_core::UpdateHook::update_hook(self, hook)
    }
];
