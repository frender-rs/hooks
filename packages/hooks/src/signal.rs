pub use UseSignal as use_signal;

use std::pin::Pin;

use hooks_core::{Hook, HookPollNextUpdate, HookUnmount};

use crate::ShareValue;

mod sealed {
    use hooks_core::{HookValue, HookValueBounds};

    use super::Signal;

    pub trait HookValueImplSignal<
        'hook,
        V,
        ImplicitBounds: HookValueBounds<'hook, Self> = &'hook Self,
    >: HookValue<'hook, ImplicitBounds, Value = Self::HookValueImplSignal>
    {
        type HookValueImplSignal: Signal<SignalHook = Self, Value = V>;
    }

    impl<'hook, H: ?Sized, V> HookValueImplSignal<'hook, V> for H
    where
        H: HookValue<'hook>,
        H::Value: Signal<SignalHook = Self, Value = V>,
    {
        type HookValueImplSignal = H::Value;
    }
}

/// `for<'hook> sealed::HookValueImplSignal<'hook>` here acts like [`for<'hook> HookValue<'hook, Value: Signal<SignalHook = Self, Value = Self::SignalShareValue>>`](crate::HookValue::Value).
pub trait SignalHook:
    Hook + for<'hook> sealed::HookValueImplSignal<'hook, Self::SignalShareValue>
{
    type SignalShareValue;
}

pub trait Signal: ShareValue {
    type SignalHook: SignalHook<SignalShareValue = Self::Value>;
    type SignalHookUninitialized: HookPollNextUpdate + HookUnmount + Default;

    fn to_signal_hook(&self) -> Self::SignalHook;
    fn update_signal_hook(&self, hook: Pin<&mut Self::SignalHook>);
    fn h_signal_hook<'hook>(
        &self,
        hook: Pin<&'hook mut Self::SignalHookUninitialized>,
    ) -> hooks_core::Value![Self::SignalHook, 'hook];

    fn use_signal(&self) -> UseSignal<'_, Self> {
        UseSignal(self)
    }

    fn notify_changed(&self);

    fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut Self::Value) -> (R, bool)) -> R;
}

impl<S: Signal + ?Sized> Signal for &S {
    type SignalHook = S::SignalHook;
    type SignalHookUninitialized = S::SignalHookUninitialized;
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
