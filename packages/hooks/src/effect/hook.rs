use super::{EffectCleanup, EffectFor};

struct EffectInner<Dep, E: EffectFor<Dep>> {
    changed: bool,
    dep: Dep,
    effect: Option<E>,
    cleanup: Option<E::Cleanup>,
}

impl<Dep, E: EffectFor<Dep>> Drop for EffectInner<Dep, E> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup.cleanup()
        }
    }
}

impl<Dep: std::fmt::Debug, E: EffectFor<Dep>> std::fmt::Debug for EffectInner<Dep, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EffectInner")
            .field("changed", &self.changed)
            .field("dep", &self.dep)
            .field("effect", &self.effect.as_ref().and(Some("effect")))
            .field("cleanup", &self.cleanup.as_ref().and(Some("cleanup")))
            .finish()
    }
}

pub struct Effect<Dep, E: EffectFor<Dep>>(Option<EffectInner<Dep, E>>);

impl<Dep, E: EffectFor<Dep>> Default for Effect<Dep, E> {
    #[inline]
    fn default() -> Self {
        Self(None)
    }
}

impl<Dep: std::fmt::Debug, E: EffectFor<Dep>> std::fmt::Debug for Effect<Dep, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EffectImpl").field(&self.0).finish()
    }
}

impl<Dep, E: EffectFor<Dep>> Unpin for Effect<Dep, E> {}

impl<Dep, E: EffectFor<Dep>> crate::HookBounds for Effect<Dep, E> {
    type Bounds = Self;
}

impl<Dep, E: EffectFor<Dep>> crate::HookPollNextUpdate for Effect<Dep, E> {
    fn poll_next_update(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        if let Some(this) = &mut self.get_mut().0 {
            if this.changed {
                this.changed = false;

                if let Some(cleanup) = this.cleanup.take() {
                    cleanup.cleanup()
                }

                let dep = &this.dep;
                if let Some(effect) = this.effect.take() {
                    let cleanup = effect.effect_for(dep);
                    this.cleanup = Some(cleanup);
                }
            }
        };

        std::task::Poll::Ready(false)
    }
}

impl<Dep, E: EffectFor<Dep>> Effect<Dep, E> {
    pub fn use_hook_eq(self: std::pin::Pin<&mut Self>, effect: E, dep: Dep)
    where
        Dep: PartialEq,
    {
        let this = self.get_mut();

        if let Some(this) = &mut this.0 {
            if this.dep != dep {
                this.dep = dep;
                this.changed = true;
                this.effect = Some(effect);
            }
        } else {
            this.0 = Some(EffectInner {
                changed: true,
                dep,
                effect: Some(effect),
                cleanup: None,
            })
        }
    }

    pub fn use_hook_with(
        self: std::pin::Pin<&mut Self>,
        get_new_dep_and_effect: impl FnOnce(Option<&Dep>) -> Option<(Dep, E)>,
    ) {
        let this = self.get_mut();
        if let Some(this) = &mut this.0 {
            if let Some((dep, effect)) = get_new_dep_and_effect(Some(&this.dep)) {
                this.changed = true;
                this.dep = dep;
                this.effect = Some(effect);
            }
        } else if let Some((dep, effect)) = get_new_dep_and_effect(None) {
            this.0 = Some(EffectInner {
                changed: true,
                dep,
                effect: Some(effect),
                cleanup: None,
            })
        }
    }
}

impl<'hook, Dep: PartialEq, E: EffectFor<Dep>> crate::HookLifetime<'hook, (E, Dep)>
    for Effect<Dep, E>
{
    type Value = ();
}

impl<Dep: PartialEq, E: EffectFor<Dep>> crate::Hook<(E, Dep)> for Effect<Dep, E> {
    #[inline]
    fn use_hook<'hook>(self: std::pin::Pin<&'hook mut Self>, (effect, dep): (E, Dep))
    where
        Self: 'hook,
    {
        self.use_hook_eq(effect, dep)
    }
}

impl<'hook, Dep, E: EffectFor<Dep>, F: FnOnce(Option<&Dep>) -> Option<(Dep, E)>>
    crate::HookLifetime<'hook, (F,)> for Effect<Dep, E>
{
    type Value = ();
}

impl<Dep, E: EffectFor<Dep>, F: FnOnce(Option<&Dep>) -> Option<(Dep, E)>> crate::Hook<(F,)>
    for Effect<Dep, E>
{
    #[inline]
    fn use_hook<'hook>(
        self: std::pin::Pin<&'hook mut Self>,
        (get_new_dep_and_effect,): (F,),
    ) -> <Self as hooks_core::HookLifetime<'hook, (F,)>>::Value
    where
        Self: 'hook,
    {
        self.use_hook_with(get_new_dep_and_effect)
    }
}

#[inline]
pub fn use_effect<Dep, E: EffectFor<Dep>>() -> Effect<Dep, E> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use futures_lite::future::block_on;
    use hooks_core::{HookExt, HookPollNextUpdateExt};

    use crate::{effect_with_fn, use_effect};

    #[test]
    fn test_use_effect_with() {
        block_on(async {
            let mut values = vec![];

            {
                let hook = use_effect();
                futures_lite::pin!(hook);

                assert!(!hook.next_update().await);

                let v = "123".to_string();

                hook.as_mut().use_hook((effect_with_fn(|old_v| {
                    if old_v == Some(&v) {
                        None
                    } else {
                        Some((v.clone(), |v: &String| values.push(v.clone())))
                    }
                }),));

                drop(v); // v is not moved before.

                assert!(!hook.next_update().await);
            }

            assert_eq!(values, ["123"]);
        });
    }
}

/// An identity method for defining `get_new_dep_and_effect` closure with proper lifetime generics.
///
/// Without this, you have to annotate each argument type in closure to make it compile.
///
/// ```
/// # use hooks::{hook, use_effect};
/// #[hook]
/// fn use_some_effect() {
///     use_effect(|old_dep: Option<&i32>| {
///         if old_dep == Some(&1) {
///             None
///         } else {
///             Some((1, |v: &_| println!("{}", *v)))
///         }
///     })
/// }
/// ```
///
/// With this, you can let the compiler infer the lifetimes.
///
/// ```
/// # use hooks::{hook, use_effect, effect_with_fn, effect_fn};
/// #[hook]
/// fn use_some_effect() {
///     use_effect(effect_with_fn(|old_dep| {
///         if old_dep == Some(&1) {
///             None
///         } else {
///             Some((1, effect_fn(|v| println!("{}", *v))))
///         }
///     }))
/// }
/// ```
#[inline]
pub fn effect_with_fn<Dep, E: EffectFor<Dep>, F: FnOnce(Option<&Dep>) -> Option<(Dep, E)>>(
    f: F,
) -> F {
    f
}

/// Please see [`effect_with_fn`] for how this works.
#[inline]
pub fn effect_fn<Dep, C: EffectCleanup, F: FnOnce(&Dep) -> C>(f: F) -> F {
    f
}
