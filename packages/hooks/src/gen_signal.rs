use std::cell::RefCell;

use hooks_gen::{local, Key, Owner, Store};

use crate::{
    state_owner::{SharableRef, SignalInner, SignalOwner},
    utils::RcStatus,
};

impl<T> SharableRef for local::Owner<T> {
    type Value = T;

    fn create(value: Self::Value) -> Self {
        local::Store.insert(value)
    }

    fn map<R>(&self, f: impl FnOnce(&Self::Value) -> R) -> R {
        self.key().map(f)
    }

    fn shared_count(&self) -> usize {
        Owner::shared_count(self)
    }

    fn map_mut_or_borrow_mut<V, R>(
        &mut self,
        _: impl FnOnce(&mut Self::Value) -> &mut V,
        on_borrow_mut: impl FnOnce(&Self::Value) -> &RefCell<V>,
        f: impl FnOnce(&mut V, RcStatus) -> R,
    ) -> R {
        let status = if Owner::shared_count(self) <= 1 {
            RcStatus::Owned
        } else {
            RcStatus::Shared
        };

        self.map(|this| {
            let mut value = on_borrow_mut(this).borrow_mut();
            f(&mut value, status)
        })
    }
}

pub struct GenSignalHook<T: 'static> {
    imp: SignalOwner<T, local::Owner<SignalInner<T>>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for GenSignalHook<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.imp.debug_fmt("SharedState", f)
    }
}

impl<T> Clone for GenSignalHook<T> {
    fn clone(&self) -> Self {
        Self {
            imp: self.imp.clone(),
        }
    }
}

impl<T> GenSignalHook<T> {
    pub fn new(initial_value: T) -> Self {
        Self {
            imp: SignalOwner::new(initial_value),
        }
    }

    fn inner(&self) -> &local::Owner<SignalInner<T>> {
        self.imp.inner()
    }
}

/// GenSignal is !Send + !Sync
/// ```compile_fail
/// # fn test(v: hooks::GenSignal<()>) -> impl Send { v };
/// ```
/// ```compile_fail
/// # fn test(v: hooks::GenSignal<()>) -> impl Sync { v };
/// ```
pub struct GenSignal<T: 'static> {
    key: local::Key<SignalInner<T>>,
}

impl<T: 'static> Eq for GenSignal<T> {}
impl<T: 'static> PartialEq for GenSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T: 'static> Copy for GenSignal<T> {}
impl<T: 'static> Clone for GenSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

hooks_core::impl_hook![
    type For<T> = GenSignalHook<T>;
    fn unmount() {}
    #[inline]
    fn poll_next_update(self, cx: _) {
        self.get_mut().imp.impl_poll_next_update_never_false(cx)
    }
    #[inline]
    fn use_hook(self) -> GenSignal<T> {
        use hooks_core::HookExt;

        GenSignal {
            key: self.get_mut().imp.use_hook().inner().key(),
        }
    }
];

#[cfg(feature = "ShareValue")]
impl<T> crate::ShareValue for GenSignal<T> {
    type Value = T;

    fn try_unwrap(self) -> Result<Self::Value, Self>
    where
        Self: Sized,
    {
        Err(self)
    }

    #[inline]
    fn get(&self) -> T
    where
        T: Copy,
    {
        self.key.map(|inner| *inner.value.borrow())
    }

    #[inline]
    fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        self.key.map(|inner| inner.value.borrow().clone())
    }

    #[inline]
    fn replace(&self, new_value: T) -> T {
        self.map_mut(|v| std::mem::replace(v, new_value))
    }

    #[inline]
    fn map<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.key.map(|inner| f(&inner.value.borrow()))
    }

    #[inline]
    fn map_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.key.map(|inner| {
            inner.notify_changed();
            f(&mut inner.value.borrow_mut())
        })
    }

    fn equivalent_to(&self, other: &Self) -> bool {
        self == other
    }
}

#[cfg(feature = "Signal")]
impl<T> crate::SignalHook for GenSignalHook<T> {
    type SignalShareValue = T;

    fn to_signal(&self) -> GenSignal<T> {
        GenSignal {
            key: self.inner().key(),
        }
    }
}

#[cfg(feature = "Signal")]
impl<T> crate::Signal for GenSignal<T> {
    type SignalHook = GenSignalHook<T>;
    type SignalHookUninitialized = crate::utils::UninitializedHook<Self::SignalHook>;

    fn is_signal_of(&self, signal_hook: &Self::SignalHook) -> bool {
        use crate::SignalHook;
        *self == signal_hook.to_signal()
    }

    fn to_signal_hook(&self) -> Self::SignalHook {
        let owner = self.key.owner();
        GenSignalHook {
            imp: SignalOwner::from_sharable_ref(owner),
        }
    }

    fn update_signal_hook(&self, mut hook: std::pin::Pin<&mut Self::SignalHook>) {
        if self.key != hook.inner().key() {
            hook.set(self.to_signal_hook())
        }
    }

    fn h_signal_hook<'hook>(
        &self,
        hook: std::pin::Pin<&'hook mut Self::SignalHookUninitialized>,
    ) -> crate::Value<'hook, Self::SignalHook> {
        hook.get_mut().use_with_signal(self)
    }

    fn notify_changed(&self) {
        self.key.map(SignalInner::notify_changed)
    }

    fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut Self::Value) -> (R, bool)) -> R {
        self.key.map(|imp| imp.map_mut_and_notify_if(f))
    }
}

pub struct UseGenSignal<T: 'static>(pub T);

hooks_core::impl_hook!(
    type For<T> = UseGenSignal<T>;

    fn into_hook(self) -> GenSignalHook<T> {
        GenSignalHook::new(self.0)
    }

    fn update_hook(self, _hook: _) {}

    fn h(self, hook: crate::utils::UninitializedHook<GenSignalHook<T>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
);

pub struct UseGenSignalWith<F>(pub F);

hooks_core::impl_hook!(
    type For<T: 'static, F: FnOnce() -> T> = UseGenSignalWith<F>;

    fn into_hook(self) -> GenSignalHook<T> {
        GenSignalHook::new(self.0())
    }

    fn update_hook(self, _hook: _) {}

    fn h(self, hook: crate::utils::UninitializedHook<GenSignalHook<T>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
);

#[cfg(feature = "ShareValue")]
#[cfg(feature = "futures-core")]
#[cfg(test)]
mod tests {
    use futures_lite::StreamExt;
    use hooks_core::hook_fn;

    use crate::{use_gen_signal, utils::testing::assert_always_pending, ShareValue};

    #[test]
    #[cfg(feature = "use_effect")]
    fn signal() {
        use hooks_core::IntoHook;

        use crate::use_effect;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h![use_gen_signal(0)];

                let value = state.get();

                h![use_effect(
                    move |v: &_| {
                        if *v < 2 {
                            state.set(*v + 1);
                        }
                    },
                    value,
                )];

                value
            }
        );

        futures_lite::future::block_on(async {
            let mut values = use_test().into_hook_values();

            let res = (&mut values).take(3).collect::<Vec<_>>().await;
            assert_eq!(res, [0, 1, 2]);

            assert_always_pending(move || async move { values.next().await });
        });
    }

    #[cfg(feature = "Signal")]
    #[test]
    fn drop_in_map() {
        use hooks_core::IntoHook;

        use crate::Signal;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h!(use_gen_signal(0));

                let value = state.get();

                let s = state.to_signal_hook();

                let _: () = state.map(|_| drop(s));

                value
            }
        );

        let mut values = use_test().into_hook_values();
        assert_eq!(
            futures_lite::future::block_on((&mut values).take(1).collect::<Vec<_>>(),),
            [0]
        );
        assert_always_pending(|| values.next())
    }

    #[cfg(feature = "Signal")]
    #[test]
    fn drop_in_conditional_map_mut() {
        use hooks_core::IntoHook;

        use crate::Signal;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h!(use_gen_signal(0));

                let value = state.get();
                let s = state.to_signal_hook();

                if value == 0 {
                    let _: () = state.map_mut(|v| {
                        drop(s);
                        *v = 1;
                    });
                }

                value
            }
        );

        let mut values = use_test().into_hook_values();

        assert_eq!(
            futures_lite::future::block_on((&mut values).take(2).collect::<Vec<_>>(),),
            [0, 1]
        );

        assert_always_pending(|| values.next())
    }

    #[test]
    fn unconditional_map_mut_should_emit_infinite_values() {
        use hooks_core::IntoHook;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h!(use_gen_signal(0));

                let value = state.get();

                let _: () = state.map_mut(|_| {});

                value
            }
        );

        let res = futures_lite::future::block_on(
            use_test().into_hook_values().take(100).collect::<Vec<_>>(),
        );

        assert_eq!(res, [0; 100])
    }
}
