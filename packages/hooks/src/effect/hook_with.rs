use hooks_core::{Hook, HookBounds, HookLifetime, HookPollNextUpdate};

use crate::{EffectCleanup, EffectFor};

pub struct EffectWith<Dep, E: EffectFor<Dep>> {
    changed: bool,
    dep: Option<Dep>,
    effect: Option<E>,
    cleanup: Option<E::Cleanup>,
}

impl<Dep, E: EffectFor<Dep>> EffectWith<Dep, E> {
    pub fn use_hook_generic(
        self: std::pin::Pin<&mut Self>,
        get_new_dep_and_effect: impl FnOnce(&Option<Dep>) -> Option<(Dep, E)>,
    ) {
        let this = self.get_mut();
        if let Some((dep, effect)) = get_new_dep_and_effect(&this.dep) {
            this.changed = true;
            this.dep = Some(dep);
            this.effect = Some(effect);
        }
    }
}

impl<Dep, E: EffectFor<Dep>> Unpin for EffectWith<Dep, E> {}

impl<Dep, E: EffectFor<Dep>> Drop for EffectWith<Dep, E> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup.cleanup()
        }
    }
}

impl<Dep, E: EffectFor<Dep>> HookBounds for EffectWith<Dep, E> {
    type Bounds = Self;
}

impl<'hook, Dep, E: EffectFor<Dep>, F: FnOnce(&Option<Dep>) -> Option<(Dep, E)>>
    HookLifetime<'hook, (F,)> for EffectWith<Dep, E>
{
    type Value = ();
}

impl<Dep, E: EffectFor<Dep>> HookPollNextUpdate for EffectWith<Dep, E> {
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
}

impl<Dep, E: EffectFor<Dep>, F: FnOnce(&Option<Dep>) -> Option<(Dep, E)>> Hook<(F,)>
    for EffectWith<Dep, E>
{
    #[inline]
    fn use_hook<'hook>(
        self: std::pin::Pin<&'hook mut Self>,
        (get_new_dep_and_effect,): (F,),
    ) -> <Self as HookLifetime<'hook, (F,)>>::Value
    where
        Self: 'hook,
    {
        self.use_hook_generic(get_new_dep_and_effect)
    }
}

#[inline]
pub fn use_effect_with<Dep, E: EffectFor<Dep>>() -> EffectWith<Dep, E> {
    EffectWith {
        changed: false,
        dep: None,
        effect: None,
        cleanup: None,
    }
}

#[cfg(test)]
mod tests {
    use futures_lite::future::block_on;
    use hooks_core::{HookExt, HookPollNextUpdateExt};

    use super::use_effect_with;

    #[test]
    fn test_use_effect_with() {
        block_on(async {
            let mut values = vec![];

            let mut hook = use_effect_with();

            assert!(!hook.next_update().await);

            let v = "123".to_string();

            hook.use_hook((|old_v| {
                if old_v.as_ref() == Some(&v) {
                    None
                } else {
                    Some((v.clone(), |v: &String| values.push(v.clone())))
                }
            },));

            drop(v); // v is not moved before.

            assert!(!hook.next_update().await);
            drop(hook);

            assert_eq!(values, ["123"]);
        });
    }
}
