use std::task::Poll;

use super::{inner::EffectInner, EffectCleanup, EffectFor};

#[derive(Debug)]
struct EffectDep<Dep> {
    changed: bool,
    value: Dep,
}

pub struct Effect<Dep, E: EffectFor<Dep>> {
    dep: Option<EffectDep<Dep>>,
    inner: EffectInner<E, E::Cleanup>,
}

impl<Dep: std::fmt::Debug, E: EffectFor<Dep>> std::fmt::Debug for Effect<Dep, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Effect")
            .field("dep", &self.dep)
            .field("inner", &self.inner)
            .finish()
    }
}

impl<Dep, E: EffectFor<Dep>> Default for Effect<Dep, E> {
    #[inline]
    fn default() -> Self {
        Self {
            dep: None,
            inner: Default::default(),
        }
    }
}

impl<Dep, E: EffectFor<Dep>> Unpin for Effect<Dep, E> {}

impl<Dep, E: EffectFor<Dep>> crate::HookBounds for Effect<Dep, E> {
    type Bounds = Self;
}

impl<Dep, E: EffectFor<Dep>> crate::HookPollNextUpdate for Effect<Dep, E> {
    #[inline]
    fn poll_next_update(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<bool> {
        let this = self.get_mut();

        match &mut this.dep {
            Some(dep) => {
                if dep.changed {
                    dep.changed = false;
                    this.inner.cleanup_and_effect_for(&dep.value)
                }
                Poll::Ready(false)
            }
            None => Poll::Ready(true),
        }
    }
}

impl<Dep, E: EffectFor<Dep>> Effect<Dep, E> {
    pub fn use_hook_eq(self: std::pin::Pin<&mut Self>, effect: E, dep: Dep)
    where
        Dep: PartialEq,
    {
        let this = self.get_mut();

        if let Some(old) = &mut this.dep {
            if old.value != dep {
                old.value = dep;
                old.changed = true;
                this.inner.register_effect(effect)
            }
        } else {
            this.dep = Some(EffectDep {
                changed: true,
                value: dep,
            });
            this.inner.register_effect(effect)
        }
    }

    pub fn use_hook_with(
        self: std::pin::Pin<&mut Self>,
        get_new_dep_and_effect: impl FnOnce(Option<&Dep>) -> Option<(Dep, E)>,
    ) {
        let this = self.get_mut();
        if let Some(old) = &mut this.dep {
            if let Some((dep, effect)) = get_new_dep_and_effect(Some(&old.value)) {
                old.changed = true;
                old.value = dep;
                this.inner.register_effect(effect)
            }
        } else if let Some((dep, effect)) = get_new_dep_and_effect(None) {
            this.dep = Some(EffectDep {
                changed: true,
                value: dep,
            });
            this.inner.register_effect(effect)
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

/// Register an effect for a dependency.
/// The effect will be run in the [`poll_next_update`]
/// after [`use_hook`] registers a new dependency.
///
/// [`use_hook`]: crate::Hook::use_hook
/// [`poll_next_update`]: crate::HookPollNextUpdate::poll_next_update
///
/// ## Usage
///
/// You can pass an effect with an dependency which impl [`PartialEq`].
/// If the dependency changes (`dependency != old_dependency`),
/// `effect` will be registered and run in the further [`poll_next_update`].
///
/// ```
/// # use hooks::{hook, use_effect}; fn do_some_effects() {} #[hook] fn use_demo() {
/// # let effect = |_: &_| {}; let dependency = ();
/// use_effect(effect, dependency);
/// # }
/// ```
///
/// ```
/// # use hooks::{hook, use_effect}; fn do_some_effects() {} #[hook] fn use_demo() {
/// use_effect(|dep: &i32| {
///     do_some_effects();
/// }, 0);
/// # }
/// ```
///
/// `effect` can return a cleanup (which impl [`EffectCleanup`]).
/// When the effect is run, the returned cleanup will be registered.
/// If further [`use_hook`] registers new dependency and new effect,
/// in the further [`poll_next_update`],
/// the cleanup will be run and then the new effect will be run.
/// When the hook is dropped, the last cleanup will be run.
///
/// ```
/// # use hooks::{hook, use_effect}; fn do_some_effects() {} fn do_some_cleanup() {} #[hook] fn use_demo() {
/// use_effect(|dep: &i32| {
///     do_some_effects();
///     || do_some_cleanup()
/// }, 0);
/// # }
/// ```
///
/// `use_effect(effect, dependency)` requires you to move the dependency
/// even it may be equal to the old dependency.
/// You can return new dependency and new effect conditionally with the following code:
///
/// ```
/// # use hooks::{hook, use_effect, effect_fn, get_new_dep_and_effect}; #[hook] fn use_demo() {
/// use_effect(get_new_dep_and_effect(|old_dep| {
///     if old_dep == Some(&1) {
///         None
///     } else {
///         let new_dep = 1;
///         let effect = effect_fn(|v| println!("{}", *v));
///         Some((new_dep, effect))
///     }
/// }))
/// # }
/// ```
///
/// In the above code, [`get_new_dep_and_effect`] and [`effect_fn`] are two fns
/// which just return the passed value. Without them, you will have to
/// annotate lifetimes in closure arguments.
/// For more details, see [`get_new_dep_and_effect`].
///
/// ## Examples
///
/// ```
/// # use hooks::{hook, use_effect, HookExt};
/// #[hook]
/// fn use_print_effect() {
///     use_effect(|_: &_| {
///         println!("do some effects");
///
///         // Return an optional cleanup function
///         move || println!("cleaning up")
///     }, ())
/// }
///
/// # futures_lite::future::block_on(async {
/// let mut hook = use_print_effect();
///
/// println!("hook created");
///
/// assert!(hook.next_value(()).await.is_some());
///
/// println!("first next_value returned");
///
/// assert!(hook.next_value(()).await.is_none());
///
/// println!("second next_value returned");
/// # });
///
/// println!("hook is dropped");
/// ```
///
/// The above code would print:
///
/// ```txt
/// hook created
/// first next_value returned
/// do some effects
/// second next_value returned
/// cleaning up
/// hook is dropped
/// ```
#[inline]
pub fn use_effect<Dep, E: EffectFor<Dep>>() -> Effect<Dep, E> {
    Default::default()
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use futures_lite::future::block_on;
    use hooks_core::{HookExt, HookPollNextUpdateExt};

    use crate::{effect_fn, get_new_dep_and_effect, hook, use_effect};

    #[test]
    fn custom_hook() {
        #[hook(hooks_core_path = "::hooks_core")]
        fn use_test_effect<'a>(history: &'a RefCell<Vec<&'static str>>) {
            use_effect(
                effect_fn(move |_| {
                    // Do some effects
                    history.borrow_mut().push("effecting");

                    // Return an optional cleanup function
                    move || history.borrow_mut().push("cleaning")
                }),
                (),
            )
        }

        let history = RefCell::new(Vec::with_capacity(2));

        futures_lite::future::block_on(async {
            let mut hook = use_test_effect();

            // poll_next_update would return true, use_hook would register effect,
            assert!(hook.next_value((&history,)).await.is_some());

            assert_eq!(history.borrow().len(), 0);

            // poll_next_update would return false and effect and register cleanup
            // use_hook would not register because dependencies does not change
            assert!(hook.next_value((&history,)).await.is_none());

            assert_eq!(*history.borrow(), ["effecting"]);

            // poll_next_update would return false and do nothing
            // use_hook would not register because dependencies does not change
            assert!(hook.next_value((&history,)).await.is_none());

            assert_eq!(*history.borrow(), ["effecting"]);

            drop(hook); // The last cleanup will be run when dropping.

            assert_eq!(history.into_inner(), ["effecting", "cleaning"]);
        })
    }

    #[test]
    fn test_use_effect_with() {
        block_on(async {
            let mut values = vec![];

            {
                let mut hook = use_effect();

                assert!(hook.next_update().await);

                let v = "123".to_string();

                hook.use_hook((get_new_dep_and_effect(|old_v| {
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
/// # use hooks::{hook, use_effect, get_new_dep_and_effect, effect_fn};
/// #[hook]
/// fn use_some_effect() {
///     use_effect(get_new_dep_and_effect(|old_dep| {
///         if old_dep == Some(&1) {
///             None
///         } else {
///             Some((1, effect_fn(|v| println!("{}", *v))))
///         }
///     }))
/// }
/// ```
#[inline]
pub fn get_new_dep_and_effect<
    Dep,
    E: EffectFor<Dep>,
    F: FnOnce(Option<&Dep>) -> Option<(Dep, E)>,
>(
    f: F,
) -> F {
    f
}

/// Please see [`get_new_dep_and_effect`] for how this works.
#[inline]
pub fn effect_fn<Dep, C: EffectCleanup, F: FnOnce(&Dep) -> C>(f: F) -> F {
    f
}

pub mod v2 {
    use super::*;
    use hooks_core::v2::{IntoHook, UpdateHook, UpdateHookUninitialized};

    pub struct UseEffect<Dep: PartialEq, E: EffectFor<Dep>>(pub E, pub Dep);

    impl<Dep: PartialEq, E: EffectFor<Dep>> UpdateHookUninitialized for UseEffect<Dep, E> {
        type Uninitialized = Effect<Dep, E>;

        fn hook(
            self,
            hook: std::pin::Pin<&mut Self::Uninitialized>,
        ) -> <Self::Hook as hooks_core::v2::Hook>::Value<'_> {
            self.update_hook(hook)
        }
    }

    hooks_core::v2_impl_hook!(
        const _: super::Effect<Dep, E> = Generics![Dep: PartialEq, E: EffectFor<Dep>];
        #[inline(always)]
        fn use_hook(self) {}
    );

    impl<Dep: PartialEq, E: EffectFor<Dep>> IntoHook for UseEffect<Dep, E> {
        type Hook = super::Effect<Dep, E>;

        fn into_hook(self) -> Self::Hook {
            let mut h = super::Effect::default();
            std::pin::Pin::new(&mut h).use_hook_eq(self.0, self.1);
            h
        }
    }

    impl<Dep: PartialEq, E: EffectFor<Dep>> UpdateHook for UseEffect<Dep, E> {
        fn update_hook(self, hook: std::pin::Pin<&mut Self::Hook>) {
            hook.use_hook_eq(self.0, self.1)
        }
    }

    pub use UseEffect as use_effect;
}
