use std::{marker::PhantomData, pin::Pin, task::Poll};

use super::{inner::EffectInner, EffectCleanup, EffectFor};

#[derive(Debug)]
struct EffectDep<Dep> {
    changed: bool,
    value: Option<Dep>,
}

pub struct Effect<Dep, E: EffectFor<Dep>> {
    dep: EffectDep<Dep>,
    inner: EffectInner<E, E::Cleanup>,
}

impl<Dep, E: EffectFor<Dep>> Unpin for Effect<Dep, E> {}

impl<Dep, E: EffectFor<Dep>> Default for Effect<Dep, E> {
    fn default() -> Self {
        Self {
            dep: EffectDep {
                changed: false,
                value: None,
            },
            inner: Default::default(),
        }
    }
}

impl<Dep: std::fmt::Debug, E: EffectFor<Dep>> std::fmt::Debug for Effect<Dep, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Effect")
            .field("dep", &self.dep)
            .field("inner", &self.inner)
            .finish()
    }
}

crate::utils::impl_hook![
    type For<Dep, E: EffectFor<Dep>> = Effect<Dep, E>;
    #[inline]
    fn unmount(self) {
        self.get_mut().inner.unmount()
    }
    fn poll_next_update(self) {
        let this = self.get_mut();
        let EffectDep { changed, value } = &mut this.dep;
        if *changed {
            *changed = false;
            if let Some(value) = &value {
                this.inner.cleanup_and_effect_for(value);
            }
        }
        Poll::Ready(false)
    }
    #[inline(always)]
    fn use_hook(self) {}
];

impl<Dep, E: EffectFor<Dep>> Effect<Dep, E> {
    pub fn register_effect_if_dep_ne(self: std::pin::Pin<&mut Self>, effect: E, dep: Dep)
    where
        Dep: PartialEq,
    {
        let this = self.get_mut();

        let dep = Some(dep);
        if this.dep.value != dep {
            this.dep.changed = true;
            this.dep.value = dep;
            this.inner.register_effect(effect)
        }
    }

    pub fn register_effect_if(
        self: std::pin::Pin<&mut Self>,
        get_new_dep_and_effect: impl FnOnce(&mut Option<Dep>) -> Option<E>,
    ) {
        let this = self.get_mut();
        if let Some(new_effect) = get_new_dep_and_effect(&mut this.dep.value) {
            this.dep.changed = true;
            this.inner.register_effect(new_effect)
        }
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
pub struct UseEffect<Dep: PartialEq, E: EffectFor<Dep>>(pub E, pub Dep);
pub use UseEffect as use_effect;

hooks_core::impl_hook![
    type For<Dep: PartialEq, E: EffectFor<Dep>> = UseEffect<Dep, E>;
    #[inline]
    fn into_hook(self) -> Effect<Dep, E> {
        Effect {
            dep: EffectDep {
                changed: false,
                value: Some(self.1),
            },
            inner: EffectInner::new_registered(self.0),
        }
    }
    #[inline]
    fn update_hook(self, hook: _) {
        hook.register_effect_if_dep_ne(self.0, self.1)
    }
    #[inline]
    fn h(self, hook: Effect<Dep, E>) {
        hook.register_effect_if_dep_ne(self.0, self.1)
    }
];

pub struct UseEffectWith<Dep, E: EffectFor<Dep>, F: FnOnce(&mut Option<Dep>) -> Option<E>>(
    F,
    PhantomData<Dep>,
);

#[inline(always)]
pub fn use_effect_with<Dep, E: EffectFor<Dep>>(
    get_effect: impl FnOnce(&mut Option<Dep>) -> Option<E>,
) -> UseEffectWith<Dep, E, impl FnOnce(&mut Option<Dep>) -> Option<E>> {
    UseEffectWith(get_effect, PhantomData)
}

hooks_core::impl_hook![
    type For<Dep, E: EffectFor<Dep>, F> = UseEffectWith<Dep, E, F>
        where __![F: FnOnce(&mut Option<Dep>) -> Option<E>]: __;

    fn into_hook(self) -> Effect<Dep, E> {
        let mut dep = None;
        let effect = self.0(&mut dep);

        Effect {
            dep: EffectDep {
                changed: effect.is_some() && dep.is_some(),
                value: dep,
            },
            inner: effect.map(EffectInner::new_registered).unwrap_or_default(),
        }
    }

    #[inline]
    fn update_hook(self, hook: _) {
        hook.register_effect_if(self.0)
    }
    #[inline]
    fn h(self, hook: Effect<Dep, E>) {
        hook.register_effect_if(self.0)
    }
];

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use futures_lite::future::block_on;
    use hooks_core::{hook_fn, HookExt, HookPollNextUpdateExt, IntoHook, UpdateHookUninitialized};

    use super::{effect_fn, get_new_dep_and_effect, use_effect, use_effect_with};

    #[test]
    fn custom_hook() {
        hook_fn!(
            type Bounds = impl 'a;
            fn use_test_effect<'a>(history: &'a RefCell<Vec<&'static str>>) {
                h![use_effect(
                    effect_fn(move |_| {
                        // Do some effects
                        history.borrow_mut().push("effecting");

                        // Return an optional cleanup function
                        move || history.borrow_mut().push("cleaning")
                    }),
                    (),
                )]
            }
        );

        let history = RefCell::new(Vec::with_capacity(2));

        futures_lite::future::block_on(async {
            {
                let hook = use_test_effect(&history).into_hook_values();
                futures_lite::pin!(hook);

                // this FnHook is unpin, thus it is initialized in `into_hook`
                assert!(hook.as_mut().next_value().await.is_none());

                assert_eq!(*history.borrow(), ["effecting"]);

                // poll_next_update would return false and do nothing
                // use_hook would not register because dependencies does not change
                assert!(hook.as_mut().next_value().await.is_none());

                assert_eq!(*history.borrow(), ["effecting"]);
            }
            // The last cleanup will be run when `hook` is dropped.

            assert_eq!(history.into_inner(), ["effecting", "cleaning"]);
        })
    }

    #[test]
    fn test_use_effect_with() {
        block_on(async {
            let mut values = vec![];

            {
                let hook = super::Effect::default();

                futures_lite::pin!(hook);

                assert!(!hook.next_update().await);

                let v = "123".to_string();

                use_effect_with(get_new_dep_and_effect(|old_v| {
                    if old_v.as_ref() == Some(&v) {
                        None
                    } else {
                        *old_v = Some(v.clone());
                        Some(|v: &String| values.push(v.clone()))
                    }
                }))
                .h(hook.as_mut());

                assert!(!hook.next_update().await);

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
pub fn get_new_dep_and_effect<Dep, E: EffectFor<Dep>, F: FnOnce(&mut Option<Dep>) -> Option<E>>(
    f: F,
) -> F {
    f
}

/// Please see [`get_new_dep_and_effect`] for how this works.
#[inline]
pub fn effect_fn<Dep, C: EffectCleanup, F: FnOnce(&Dep) -> C>(f: F) -> F {
    f
}
