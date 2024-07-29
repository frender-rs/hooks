pub use UseSignal as use_signal;

use std::pin::Pin;

use hooks_core::{Hook, HookPollNextUpdate, HookUnmount};

use crate::{ShareValue, ToOwnedShareValue};

mod sealed {
    use hooks_core::{HookValue, HookValueBounds};

    use crate::ShareValue;

    use super::Signal;

    pub trait HookValueImplSignal<'hook, ImplicitBounds: HookValueBounds<'hook, Self> = &'hook Self>:
        HookValue<'hook, ImplicitBounds, Value = Self::HookValueImplSignal>
    {
        type HookValueImplShareValueValue;
        type HookValueImplSignal: Signal<
            SignalHook = Self,
            Value = Self::HookValueImplShareValueValue,
        >;
    }

    impl<'hook, H: ?Sized> HookValueImplSignal<'hook> for H
    where
        H: HookValue<'hook>,
        H::Value: Signal<SignalHook = Self>,
    {
        type HookValueImplShareValueValue = <H::Value as ShareValue>::Value;
        type HookValueImplSignal = H::Value;
    }
}

/// `for<'hook> sealed::HookValueImplSignal<'hook>` here acts like [`for<'hook> HookValue<'hook, Value: Signal<SignalHook = Self, Value = Self::SignalShareValue>>`](crate::HookValue::Value).
pub trait SignalHook:
    Hook
    + Signal<Value = Self::SignalShareValue, SignalHook = Self>
    + for<'hook> sealed::HookValueImplSignal<
        'hook,
        HookValueImplShareValueValue = Self::SignalShareValue,
    >
{
    type SignalShareValue;

    /// This is the opposite of [`Signal::to_signal_hook`].
    ///
    /// In contrast to [`Hook::use_hook`], this method just require a `&Self` instead of `Pin<&mut Self>`.
    /// In implementations, [`Hook::use_hook`] might mark the SignalHook's updated value as already read
    /// but this method should not.
    fn to_signal(&self) -> crate::Value<'_, Self>;
}

pub trait Signal: ShareValue {
    type SignalHook: SignalHook<SignalShareValue = Self::Value>;
    type SignalHookUninitialized: HookPollNextUpdate + HookUnmount + Default;

    fn is_signal_of(&self, signal_hook: &Self::SignalHook) -> bool;

    /// This is the opposite of [`SignalHook::to_signal`].
    fn to_signal_hook(&self) -> Self::SignalHook;

    fn update_signal_hook(&self, hook: Pin<&mut Self::SignalHook>);
    fn h_signal_hook<'hook>(
        &self,
        hook: Pin<&'hook mut Self::SignalHookUninitialized>,
    ) -> crate::Value<'hook, Self::SignalHook>;

    fn use_signal(&self) -> UseSignal<'_, Self> {
        UseSignal(self)
    }

    fn notify_changed(&self);

    fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut Self::Value) -> (R, bool)) -> R;
}

impl<S: Signal + ?Sized> Signal for &S {
    type SignalHook = S::SignalHook;
    type SignalHookUninitialized = S::SignalHookUninitialized;

    fn is_signal_of(&self, signal_hook: &Self::SignalHook) -> bool {
        S::is_signal_of(self, signal_hook)
    }
    fn to_signal_hook(&self) -> Self::SignalHook {
        S::to_signal_hook(self)
    }
    fn update_signal_hook(&self, hook: Pin<&mut Self::SignalHook>) {
        S::update_signal_hook(self, hook)
    }
    fn h_signal_hook<'hook>(
        &self,
        hook: ::core::pin::Pin<&'hook mut Self::SignalHookUninitialized>,
    ) -> crate::Value<'hook, Self::SignalHook> {
        S::h_signal_hook(self, hook)
    }
    fn notify_changed(&self) {
        S::notify_changed(self)
    }
    fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut Self::Value) -> (R, bool)) -> R {
        S::map_mut_and_notify_if(self, f)
    }
}

pub struct UseSignal<'a, S: Signal + ?Sized>(&'a S);

hooks_core::impl_hook!(
    type For<S: Signal> = UseSignal<'_, S>;

    fn into_hook(self) -> S::SignalHook {
        self.0.to_signal_hook()
    }

    fn update_hook(self, hook: _) {
        self.0.update_signal_hook(hook)
    }

    fn h(self, hook: S::SignalHookUninitialized) {
        self.0.h_signal_hook(hook)
    }
);

pub trait ToOwnedSignal: Signal + ToOwnedShareValue<OwnedShareValue = Self::OwnedSignal> {
    type OwnedSignal: Signal<
        SignalHook = Self::SignalHook,
        Value = Self::Value,
        SignalHookUninitialized = Self::SignalHookUninitialized,
    >;
}

/// A trait alias for `Signal + ToOwnedShareValue<OwnedShareValue: Signal<SignalHook = Self::SignalHook, SignalHookUninitialized = Self::SignalHookUninitialized>>`
impl<T: ?Sized + ToOwnedShareValue> ToOwnedSignal for T
where
    T: Signal,
    T::OwnedShareValue: Signal<
        SignalHook = Self::SignalHook,
        SignalHookUninitialized = Self::SignalHookUninitialized,
    >,
{
    type OwnedSignal = T::OwnedShareValue;
}
