use std::{marker::PhantomData, task::Poll};

use super::{inner::EffectInner, EffectCleanup, EffectFor};

pub struct Effect<Dep, E: EffectFor<Dep>> {
    dependency: Option<Dep>,
    inner: EffectInner<E, E::Cleanup>,
}

impl<Dep, E: EffectFor<Dep>> Unpin for Effect<Dep, E> {}

impl<Dep, E: EffectFor<Dep>> Default for Effect<Dep, E> {
    fn default() -> Self {
        Self {
            dependency: None,
            inner: Default::default(),
        }
    }
}

impl<Dep: std::fmt::Debug, E: EffectFor<Dep>> std::fmt::Debug for Effect<Dep, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Effect")
            .field("dependency", &self.dependency)
            .field("inner", &self.inner)
            .finish()
    }
}

hooks_core::impl_hook![
    type For<Dep, E: EffectFor<Dep>> = Effect<Dep, E>;
    #[inline]
    fn unmount(self) {
        self.get_mut().inner.unmount()
    }
    fn poll_next_update(self) {
        let this = self.get_mut();
        if this.inner.effect.is_some() {
            if let Some(dependency) = &this.dependency {
                this.inner.cleanup_and_effect_for(dependency);
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
        if this.dependency != dep {
            this.dependency = dep;
            this.inner.register_effect(effect)
        }
    }

    pub fn register_effect_if(
        self: std::pin::Pin<&mut Self>,
        get_new_dep_and_effect: impl FnOnce(&mut Option<Dep>) -> Option<E>,
    ) {
        let this = self.get_mut();
        if let Some(new_effect) = get_new_dep_and_effect(&mut this.dependency) {
            this.inner.register_effect(new_effect)
        }
    }
}

/// Register an effect for a dependency.
/// The effect will be run in the [`poll_next_update`]
/// after a new effect is registered.
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
/// # use hooks::{hook_fn, use_effect}; fn do_some_effects() {} hook_fn!( fn use_demo() {
/// # let effect = |_: &_| {}; let dependency = ();
/// use_effect(effect, dependency);
/// # });
/// ```
///
/// ```
/// # use hooks::{hook_fn, use_effect}; fn do_some_effects() {} hook_fn!( fn use_demo() {
/// use_effect(|dep: &i32| {
///     do_some_effects();
/// }, 0);
/// # });
/// ```
///
/// `effect` can return a cleanup (which impl [`EffectCleanup`]).
/// When the effect is run, the returned cleanup will be registered.
/// If further [`use_hook`] registers new dependency and new effect,
/// in the further [`poll_next_update`],
/// the cleanup will be run and then the new effect will be run.
/// When the hook is [`unmount`](crate::HookUnmount::unmount)ed or dropped, the last cleanup will be run.
///
/// ```
/// # use hooks::{hook_fn, use_effect}; fn do_some_effects() {} fn do_some_cleanup() {} hook_fn!( fn use_demo() {
/// use_effect(|dep: &i32| {
///     do_some_effects();
///     || do_some_cleanup()
/// }, 0);
/// # });
/// ```
///
/// `use_effect(effect, dependency)` requires you to move the dependency
/// even it may be equal to the old dependency.
/// You can conditionally register new dependency and new effect with [`use_effect_with`].
///
/// ## Examples
///
/// ```
/// # use hooks::prelude::*;
/// hook_fn!(
///     fn use_print_effect() {
///         h![use_effect(|_: &_| {
///             println!("do some effects");
///
///             // Return an optional cleanup function
///             move || println!("cleaning up")
///         }, ())]
///     }
/// );
///
/// # futures_lite::future::block_on(async {
/// let mut hook = use_print_effect().into_hook();
///
/// println!("hook created");
///
/// assert!(hook.next_value().await.is_some());
///
/// println!("first next_value returned");
///
/// assert!(hook.next_value().await.is_none());
///
/// println!("second next_value returned");
///
/// drop(hook);
///
/// println!("hook is dropped");
/// # });
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
            dependency: Some(self.1),
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

/// Conditionally register effects with a lazy initialized dependency.
/// [`use_effect_with`] doesn't require Dependency to be [`PartialEq`],
/// and also allows lazy initialization.
///
#[cfg_attr(
    feature = "proc-macro",
    doc = r###"
```
# use hooks::{hook, use_effect_with};
#[hook(bounds = "'a")]
fn use_effect_print<'a>(value: &'a str) {
    use_effect_with(|old_dep| {
        if old_dep.as_deref() == Some(value) {
            None
        } else {
            *old_dep = Some(value.to_owned()); // lazily calling to_owned()
            Some(|v: &_| println!("{}", *v))
        }
    })
}
```
"###
)]
#[inline(always)]
pub fn use_effect_with<Dep, E: EffectFor<Dep>>(
    get_effect: impl FnOnce(&mut Option<Dep>) -> Option<E>,
) -> UseEffectWith<Dep, E, impl FnOnce(&mut Option<Dep>) -> Option<E>> {
    UseEffectWith(get_effect, PhantomData)
}

hooks_core::impl_hook![
    type For<Dep, E: EffectFor<Dep>, F: FnOnce(&mut Option<Dep>) -> Option<E>> =
        UseEffectWith<Dep, E, F>;

    fn into_hook(self) -> Effect<Dep, E> {
        let mut dependency = None;
        let effect = self.0(&mut dependency);

        Effect {
            dependency,
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

/// An identity function which asserts argument is an [`effect fn`](EffectFor).
#[inline]
pub fn effect_fn<Dep, C: EffectCleanup, F: FnOnce(&Dep) -> C>(f: F) -> F {
    f
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use futures_lite::future::block_on;
    use hooks_core::{hook_fn, HookPollNextUpdateExt, IntoHook, UpdateHookUninitialized};

    use super::{effect_fn, use_effect, use_effect_with};

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

                assert!(hook.as_mut().next_value().await.is_some());

                assert!(history.borrow().is_empty());

                // poll_next_update would return false and run effect
                // use_hook would not register because dependencies does not change
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

                use_effect_with(|old_v| {
                    if old_v.as_ref() == Some(&v) {
                        None
                    } else {
                        *old_v = Some(v.clone());
                        Some(|v: &String| values.push(v.clone()))
                    }
                })
                .h(hook.as_mut());

                assert!(!hook.next_update().await);

                drop(v); // v is not moved before.

                assert!(!hook.next_update().await);
            }

            assert_eq!(values, ["123"]);
        });
    }
}
