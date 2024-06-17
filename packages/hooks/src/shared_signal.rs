use std::{cell::RefCell, rc::Rc};

use crate::{
    state_owner::{SharableRef, SignalInner, SignalOwner},
    utils::RcStatus,
};

impl<T> SharableRef for Rc<T> {
    type Value = T;

    fn create(value: Self::Value) -> Self {
        Rc::new(value)
    }

    fn map<R>(&self, f: impl FnOnce(&Self::Value) -> R) -> R {
        f(self)
    }

    fn shared_count(&self) -> usize {
        Rc::strong_count(self)
    }

    fn map_mut_or_borrow_mut<V, R>(
        &mut self,
        on_map_mut: impl FnOnce(&mut Self::Value) -> &mut V,
        on_borrow_mut: impl FnOnce(&Self::Value) -> &RefCell<V>,
        f: impl FnOnce(&mut V, RcStatus) -> R,
    ) -> R {
        if let Some(value) = Rc::get_mut(self) {
            f(on_map_mut(value), RcStatus::Owned)
        } else {
            let mut value = on_borrow_mut(self).borrow_mut();
            f(&mut value, RcStatus::Shared)
        }
    }
}

pub struct SharedSignal<T> {
    imp: SignalOwner<T, Rc<SignalInner<T>>>,
}

impl<T> PartialEq for SharedSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self.inner(), other.inner())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SharedSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.imp.debug_fmt("SharedState", f)
    }
}

impl<T> Clone for SharedSignal<T> {
    fn clone(&self) -> Self {
        Self {
            imp: self.imp.clone(),
        }
    }
}

impl<T> SharedSignal<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self {
            imp: SignalOwner::new(initial_value),
        }
    }

    pub(crate) fn imp_notify_changed(&self) {
        self.imp.notify_changed()
    }

    pub(crate) fn imp_map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut T) -> (R, bool)) -> R {
        self.imp.map_mut_and_notify_if(f)
    }

    fn inner(&self) -> &Rc<SignalInner<T>> {
        self.imp.inner()
    }
}

#[cfg(feature = "ShareValue")]
impl<T> crate::ShareValue for SharedSignal<T> {
    type Value = T;

    fn try_unwrap(self) -> Result<Self::Value, Self>
    where
        Self: Sized,
    {
        let inner = self.inner();
        if Rc::strong_count(inner) == 1 {
            let inner = inner.clone();
            drop(self);

            match Rc::try_unwrap(inner) {
                Ok(inner) => Ok(inner.into_value()),
                Err(_) => unreachable!(),
            }
        } else {
            Err(self)
        }
    }

    #[inline]
    fn get(&self) -> T
    where
        T: Copy,
    {
        *self.inner().value.borrow()
    }

    #[inline]
    fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        self.inner().value.borrow().clone()
    }

    #[inline]
    fn replace(&self, new_value: T) -> T {
        self.map_mut(|v| std::mem::replace(v, new_value))
    }

    #[inline]
    fn map<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&self.inner().value.borrow())
    }

    #[inline]
    fn map_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.imp_notify_changed();
        f(&mut self.inner().value.borrow_mut())
    }

    fn equivalent_to(&self, other: &Self) -> bool {
        *self == *other
    }
}

#[cfg(feature = "Signal")]
impl<T> crate::SignalHook for SharedSignal<T> {
    type SignalShareValue = T;

    fn to_signal(&self) -> &Self {
        self
    }
}

#[cfg(feature = "Signal")]
impl<T> crate::Signal for SharedSignal<T> {
    type SignalHook = Self;
    type SignalHookUninitialized = crate::utils::UninitializedHook<Self>;

    fn to_signal_hook(&self) -> Self::SignalHook {
        self.clone()
    }

    fn update_signal_hook(&self, mut hook: std::pin::Pin<&mut Self::SignalHook>) {
        use crate::ShareValue;

        if !hook.equivalent_to(self) {
            hook.set(self.clone())
        }
    }

    fn h_signal_hook<'hook>(
        &self,
        hook: std::pin::Pin<&'hook mut Self::SignalHookUninitialized>,
    ) -> crate::Value<'hook, Self::SignalHook> {
        hook.get_mut().use_with_signal(self)
    }

    fn notify_changed(&self) {
        self.imp_notify_changed()
    }

    fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut Self::Value) -> (R, bool)) -> R {
        self.imp_map_mut_and_notify_if(f)
    }
}

hooks_core::impl_hook![
    type For<T> = SharedSignal<T>;
    fn unmount() {}
    #[inline]
    fn poll_next_update(self, cx: _) {
        use hooks_core::HookPollNextUpdateExt;
        self.get_mut().imp.poll_next_update(cx)
    }
    #[inline]
    fn use_hook(self) -> &'hook Self {
        use hooks_core::HookExt;

        let this = self.get_mut();
        _ = this.imp.use_hook();
        this
    }
];

pub struct UseSharedSignal<T>(pub T);

hooks_core::impl_hook![
    type For<T> = UseSharedSignal<T>;
    #[inline]
    fn into_hook(self) -> SharedSignal<T> {
        SharedSignal::new(self.0)
    }
    #[inline(always)]
    fn update_hook(self, _hook: _) {}
    fn h(self, hook: crate::utils::UninitializedHook<SharedSignal<T>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
];

pub struct UseSharedSignalWith<T, F: FnOnce() -> T>(pub F);

hooks_core::impl_hook![
    type For<T, F: FnOnce() -> T> = UseSharedSignalWith<T, F>;

    #[inline]
    fn into_hook(self) -> SharedSignal<T> {
        SharedSignal::new(self.0())
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}
    fn h(self, hook: crate::utils::UninitializedHook<SharedSignal<T>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
];

#[cfg(feature = "ShareValue")]
#[cfg(feature = "futures-core")]
#[cfg(test)]
mod tests {
    use futures_lite::StreamExt;
    use hooks_core::hook_fn;

    use crate::{use_shared_signal, utils::testing::assert_always_pending, ShareValue};

    #[test]
    #[cfg(feature = "use_effect")]
    fn shared_signal() {
        use hooks_core::IntoHook;

        use crate::use_effect;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h![use_shared_signal(0)];

                let value = state.get();
                let s = state.clone();

                h![use_effect(
                    move |v: &_| {
                        if *v < 2 {
                            s.set(*v + 1);
                        }
                    },
                    value,
                )];

                value
            }
        );

        futures_lite::future::block_on(async {
            let values = use_test().into_hook_values();

            let values = values.collect::<Vec<_>>().await;
            assert_eq!(values, [0, 1, 2]);
        });
    }

    #[test]
    fn drop_in_map() {
        use hooks_core::IntoHook;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h!(use_shared_signal(0));

                let value = state.get();
                let s = state.clone();

                let _: () = state.map(|_| drop(s));

                value
            }
        );

        assert_eq!(
            futures_lite::future::block_on(use_test().into_hook_values().collect::<Vec<_>>(),),
            [0]
        )
    }

    #[test]
    fn drop_in_conditional_map_mut() {
        use hooks_core::IntoHook;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h!(use_shared_signal(0));

                let value = state.get();
                let s = state.clone();

                if value == 0 {
                    let _: () = state.map_mut(|v| {
                        drop(s);
                        *v = 1;
                    });
                }

                value
            }
        );

        assert_eq!(
            futures_lite::future::block_on(use_test().into_hook_values().collect::<Vec<_>>(),),
            [0, 1]
        )
    }

    #[test]
    fn reference_cycle_should_always_pending() {
        use hooks_core::IntoHook;

        use crate::IntoEq;

        #[derive(PartialEq)]
        struct Data(#[allow(dead_code)] Option<super::SharedSignal<Self>>);

        hook_fn!(
            fn use_test() {
                let state = h!(use_shared_signal(Data(None))).into_eq();

                state.set(Data(Some(state.0.clone())));
            }
        );

        assert_always_pending(|| use_test().into_hook_values().collect::<Vec<_>>());
    }

    #[test]
    fn unconditional_map_mut_should_emit_infinite_values() {
        use hooks_core::IntoHook;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h!(use_shared_signal(0));

                let value = state.get();

                let _: () = state.map_mut(|_| {});

                value
            }
        );

        futures_lite::future::block_on(async {
            let res = use_test()
                .into_hook_values()
                .take(100)
                .collect::<Vec<_>>()
                .await;

            assert_eq!(res, [0; 100])
        });
    }
}
