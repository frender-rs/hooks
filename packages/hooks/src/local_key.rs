#[cfg(feature = "Signal")]
pub use self::signal::{LocalKeySignalHook, LocalKeySignalHookUninitialized};

#[cfg(feature = "ShareValue")]
mod share_value {
    use std::thread::LocalKey;

    #[cfg(feature = "ShareValue")]
    pub(super) fn same_local_key<T>(a: &'static LocalKey<T>, b: &'static LocalKey<T>) -> bool {
        std::ptr::eq(a, b)
    }

    fn same_local_key_value<T>(
        a: &'static LocalKey<T>,
        b: &'static LocalKey<T>,
        f: impl FnOnce(&T, &T) -> bool,
    ) -> bool {
        a.with(|a| b.with(|b| f(a, b)))
    }

    fn same_local_key_or_value<T>(
        a: &'static LocalKey<T>,
        b: &'static LocalKey<T>,
        f: impl FnOnce(&T, &T) -> bool,
    ) -> bool {
        same_local_key(a, b) || same_local_key_value(a, b, f)
    }

    impl<S: crate::ShareValue> crate::ShareValue for &'static LocalKey<S> {
        type Value = S::Value;

        fn try_unwrap(self) -> Result<Self::Value, Self> {
            Err(self)
        }

        #[inline]
        fn get(&self) -> Self::Value
        where
            Self::Value: Copy,
        {
            self.with(S::get)
        }

        #[inline]
        fn get_cloned(&self) -> Self::Value
        where
            Self::Value: Clone,
        {
            self.with(S::get_cloned)
        }

        fn map<R>(&self, f: impl FnOnce(&Self::Value) -> R) -> R {
            self.with(|s| s.map(f))
        }

        #[inline]
        fn set(&self, new_value: Self::Value) {
            self.with(|s| s.set(new_value))
        }

        /// The old value is returned.
        #[inline]
        fn replace(&self, new_value: Self::Value) -> Self::Value {
            self.with(|s| s.replace(new_value))
        }

        /// The old value is returned.
        #[inline]
        fn replace_mut<F: FnOnce(&mut Self::Value) -> Self::Value>(&self, f: F) -> Self::Value {
            self.with(|s| s.replace_mut(f))
        }

        /// The old value is returned.
        #[inline]
        fn replace_with<F: FnOnce(&Self::Value) -> Self::Value>(&self, f: F) -> Self::Value {
            self.with(|s| s.replace_with(f))
        }

        fn map_mut<R>(&self, f: impl FnOnce(&mut Self::Value) -> R) -> R {
            self.with(|s| s.map_mut(f))
        }

        /// Returns `true` if `self` and `other` are sharing values from the same allocation.
        /// In that case, `self` and `other` are equivalent to each other
        /// because calling the same method on either of them leads to the same result.
        fn equivalent_to(&self, other: &Self) -> bool {
            same_local_key_or_value(self, other, S::equivalent_to)
        }
    }

    impl<T: crate::ShareValue> crate::ToOwnedShareValue for &'static LocalKey<T> {
        type OwnedShareValue = Self;

        fn to_owned_share_value(&self) -> Self::OwnedShareValue {
            *self
        }
    }
}

#[cfg(feature = "Signal")]
mod signal {
    use std::thread::LocalKey;

    use crate::{ShareValue, Signal, SignalHook, ToOwnedShareValue};

    use super::share_value::same_local_key;

    pin_project_lite::pin_project!(
        // self.local_key should contains a signal of self.signal_hook,
        // which is: self.local_key.is_signal_of(&self) should always be true
        pub struct LocalKeySignalHook<S>
        where
            S: 'static,
            S: Signal,
        {
            local_key: &'static LocalKey<S>,
            #[pin]
            signal_hook: S::SignalHook,
        }
    );

    pin_project_lite::pin_project!(
        // self.local_key should contains a signal of self.signal_hook,
        // which is: self.local_key.is_signal_of(&self) should always be true
        pub struct LocalKeySignalHookUninitialized<S>
        where
            S: 'static,
            S: Signal,
        {
            local_key: Option<&'static LocalKey<S>>,
            #[pin]
            signal_hook: <S::SignalHook as SignalHook>::SignalHookUninitialized,
        }
    );

    impl<S: 'static + Signal> LocalKeySignalHook<S> {
        fn _new(local_key: &'static LocalKey<S>) -> Self {
            Self {
                local_key,
                signal_hook: local_key.with(S::to_signal_hook),
            }
        }

        fn _clone_signal_hook(&self) -> Self {
            Self {
                local_key: self.local_key,
                signal_hook: self.signal_hook.to_signal_hook(),
            }
        }
    }

    hooks_core::impl_hook!(
        type For<S: 'static + Signal> = LocalKeySignalHook<S>;

        fn unmount(self) {
            self.project().signal_hook.unmount()
        }

        fn poll_next_update(self, cx: _) {
            self.project().signal_hook.poll_next_update(cx)
        }

        fn use_hook(self) -> &'static LocalKey<S> {
            let this = self.project();
            let _ = this.signal_hook.use_hook();
            *this.local_key
        }
    );

    impl<S: 'static + Signal> Default for LocalKeySignalHookUninitialized<S> {
        fn default() -> Self {
            Self {
                local_key: None,
                signal_hook: Default::default(),
            }
        }
    }

    hooks_core::impl_hook!(
        type For<S: 'static + Signal> = LocalKeySignalHookUninitialized<S>;

        fn unmount(self) {
            self.project().signal_hook.unmount()
        }

        fn poll_next_update(self, cx: _) {
            self.project().signal_hook.poll_next_update(cx)
        }
    );

    impl<S: 'static + Signal> ShareValue for LocalKeySignalHook<S> {
        crate::share_value::proxy_share_value!(
            |self| -> &'static LocalKey<S> { &self.local_key },
            |other| &other.local_key
        );

        fn try_unwrap(self) -> Result<Self::Value, Self> {
            self.signal_hook.try_unwrap().map_err(|signal_hook| Self {
                local_key: self.local_key,
                signal_hook,
            })
        }
    }

    impl<S: Signal> Signal for LocalKeySignalHook<S> {
        type SignalHook = LocalKeySignalHook<S>;

        fn is_signal_of(&self, signal_hook: &Self::SignalHook) -> bool {
            same_local_key(self.local_key, signal_hook.local_key)
                || self.signal_hook.is_signal_of(&signal_hook.signal_hook)
        }

        fn to_signal_hook(&self) -> Self::SignalHook {
            self._clone_signal_hook()
        }

        fn update_signal_hook(&self, hook: std::pin::Pin<&mut Self::SignalHook>) {
            if same_local_key(self.local_key, hook.local_key) {
                return;
            }

            let hook = hook.project();

            self.signal_hook.update_signal_hook(hook.signal_hook);
            *hook.local_key = self.local_key;
        }

        fn h_signal_hook<'hook>(
            &self,
            hook: std::pin::Pin<
                &'hook mut <Self::SignalHook as SignalHook>::SignalHookUninitialized,
            >,
        ) -> crate::Value<'hook, Self::SignalHook> {
            let hook = hook.project();

            *hook.local_key = Some(self.local_key);
            self.signal_hook.h_signal_hook(hook.signal_hook);

            self.local_key
        }

        fn notify_changed(&self) {
            self.signal_hook.notify_changed()
        }

        fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut Self::Value) -> (R, bool)) -> R {
            self.signal_hook.map_mut_and_notify_if(f)
        }
    }

    impl<S: 'static + Signal> ToOwnedShareValue for LocalKeySignalHook<S> {
        type OwnedShareValue = &'static LocalKey<S>;

        fn to_owned_share_value(&self) -> Self::OwnedShareValue {
            self.local_key
        }
    }

    impl<S: 'static + Signal> SignalHook for LocalKeySignalHook<S> {
        type SignalShareValue = S::Value;
        type SignalHookUninitialized = LocalKeySignalHookUninitialized<S>;

        fn to_signal(&self) -> crate::Value<'_, Self> {
            self.local_key
        }
    }

    impl<S: Signal> Signal for &'static LocalKey<S> {
        type SignalHook = LocalKeySignalHook<S>;

        fn is_signal_of(&self, signal_hook: &Self::SignalHook) -> bool {
            same_local_key(self, signal_hook.local_key)
                || self.with(|this| this.is_signal_of(&signal_hook.signal_hook))
        }

        fn to_signal_hook(&self) -> Self::SignalHook {
            LocalKeySignalHook::_new(self)
        }

        fn update_signal_hook(&self, hook: std::pin::Pin<&mut Self::SignalHook>) {
            if same_local_key(self, hook.local_key) {
                return;
            }
            let hook = hook.project();
            *hook.local_key = self;
            self.with(|s| s.update_signal_hook(hook.signal_hook));
        }

        fn h_signal_hook<'hook>(
            &self,
            hook: std::pin::Pin<
                &'hook mut <Self::SignalHook as SignalHook>::SignalHookUninitialized,
            >,
        ) -> crate::Value<'hook, Self::SignalHook> {
            let hook = hook.project();
            *hook.local_key = Some(self);
            self.with(|s| s.h_signal_hook(hook.signal_hook));
            self
        }

        fn notify_changed(&self) {
            self.with(S::notify_changed)
        }

        fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut Self::Value) -> (R, bool)) -> R {
            self.with(|s| s.map_mut_and_notify_if(f))
        }
    }
}
