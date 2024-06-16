use std::{cell::RefCell, rc::Rc};

use crate::{
    state_owner::{SharableRef, SharedStateInner, StateOwner},
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

pub struct SharedState<T> {
    imp: StateOwner<T, Rc<SharedStateInner<T>>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for SharedState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.imp.debug_fmt("SharedState", f)
    }
}

impl<T> Clone for SharedState<T> {
    fn clone(&self) -> Self {
        Self {
            imp: self.imp.clone(),
        }
    }
}

impl<T> SharedState<T> {
    #[inline]
    pub fn new(initial_value: T) -> Self {
        Self {
            imp: StateOwner::new(initial_value),
        }
    }

    pub(crate) fn imp_notify_changed(&self) {
        self.imp.notify_changed()
    }

    pub(crate) fn imp_map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut T) -> (R, bool)) -> R {
        self.imp.map_mut_and_notify_if(f)
    }

    fn inner(&self) -> &Rc<SharedStateInner<T>> {
        self.imp.inner()
    }
}

#[cfg(feature = "ShareValue")]
impl<T> crate::ShareValue for SharedState<T> {
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
        Rc::ptr_eq(self.inner(), other.inner())
    }
}

#[cfg(feature = "Signal")]
impl<T> crate::SignalHook for SharedState<T> {
    type SignalShareValue = T;

    fn to_signal(&self) -> &Self {
        self
    }
}

#[cfg(feature = "Signal")]
impl<T> crate::Signal for SharedState<T> {
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
    type For<T> = SharedState<T>;
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

pub struct UseSharedState<T>(pub T);
pub use UseSharedState as use_shared_state;

hooks_core::impl_hook![
    type For<T> = UseSharedState<T>;
    #[inline]
    fn into_hook(self) -> SharedState<T> {
        SharedState::new(self.0)
    }
    #[inline(always)]
    fn update_hook(self, _hook: _) {}
    fn h(self, hook: crate::utils::UninitializedHook<SharedState<T>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
];

pub struct UseSharedStateWith<T, F: FnOnce() -> T>(pub F);
pub use UseSharedStateWith as use_shared_state_with;

hooks_core::impl_hook![
    type For<T, F: FnOnce() -> T> = UseSharedStateWith<T, F>;

    #[inline]
    fn into_hook(self) -> SharedState<T> {
        SharedState::new(self.0())
    }

    #[inline(always)]
    fn update_hook(self, _hook: _) {}
    fn h(self, hook: crate::utils::UninitializedHook<SharedState<T>>) {
        hook.get_mut().use_into_or_update_hook(self)
    }
];

#[cfg(feature = "ShareValue")]
#[cfg(feature = "futures-core")]
#[cfg(test)]
mod tests {
    use futures_lite::StreamExt;
    use hooks_core::hook_fn;

    use crate::{use_shared_state, ShareValue};

    #[test]
    #[cfg(feature = "use_effect")]
    fn shared_state() {
        use hooks_core::IntoHook;

        use crate::use_effect;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h![use_shared_state(0)];

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
                let state = h!(use_shared_state(0));

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
                let state = h!(use_shared_state(0));

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

    fn assert_timeout<Fut: std::future::Future>(
        get_fut: impl 'static + Send + FnOnce() -> Fut,
        timeout: u64,
    ) {
        use std::sync::mpsc::RecvTimeoutError;

        let (tx, rx) = std::sync::mpsc::sync_channel(0);

        let hook_thread = std::thread::spawn(move || {
            let _ = futures_lite::future::block_on(get_fut());
            tx.send(()).unwrap();
        });

        assert!(matches!(
            rx.recv_timeout(std::time::Duration::from_millis(timeout)),
            Err(RecvTimeoutError::Timeout)
        ));

        assert!(!hook_thread.is_finished());
    }

    fn assert_always_pending<Fut: std::future::Future>(
        get_fut: impl 'static + Send + FnOnce() -> Fut,
    ) {
        const TIMEOUT: u64 = 100;
        assert_timeout(get_fut, TIMEOUT)
    }

    #[test]
    fn reference_cycle_should_always_pending() {
        use hooks_core::IntoHook;

        struct Data(#[allow(dead_code)] Option<super::SharedState<Self>>);

        hook_fn!(
            fn use_test() {
                let state = h!(use_shared_state(Data(None))).clone();
                state.set(Data(Some(state.clone())));
            }
        );

        assert_always_pending(|| use_test().into_hook_values().collect::<Vec<_>>());
    }

    #[test]
    fn unconditional_map_mut_should_always_pending() {
        use hooks_core::IntoHook;

        hook_fn!(
            fn use_test() -> i32 {
                let state = h!(use_shared_state(0));

                let value = state.get();

                let _: () = state.map_mut(|_| {});

                value
            }
        );

        assert_always_pending(|| use_test().into_hook_values().collect::<Vec<_>>());
    }
}
