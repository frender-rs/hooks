pub use UseSignal as use_signal;

use std::pin::Pin;

use hooks_core::{Hook, HookPollNextUpdate, HookUnmount};

use crate::ShareValue;

mod sealed {
    use hooks_core::{HookValue, HookValueBounds};

    pub trait RefOrSelf<T: ?Sized> {}

    impl<T: ?Sized> RefOrSelf<T> for T {}
    impl<T: ?Sized> RefOrSelf<T> for &T {}

    pub trait SignalHookValue<
        'hook,
        S: ?Sized,
        ImplicitBounds: HookValueBounds<'hook, Self> = &'hook Self,
    >: HookValue<'hook, ImplicitBounds, Value = Self::SignalHookValue>
    {
        type SignalHookValue: RefOrSelf<S>;
    }

    impl<'hook, H, S> SignalHookValue<'hook, S> for H
    where
        H: HookValue<'hook>,
        H::Value: RefOrSelf<S>,
    {
        type SignalHookValue = H::Value;
    }
}

pub trait SignalHook: Hook + for<'hook> sealed::SignalHookValue<'hook, Self::Signal> {
    type Signal: Signal<SignalHook = Self> + ?Sized;
}

pub trait Signal: ShareValue {
    type SignalHook: SignalHook<Signal = Self>;
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
