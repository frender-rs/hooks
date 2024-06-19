use std::pin::Pin;

use hooks_core::{Hook, HookPollNextUpdate, HookUnmount, HookValue};

/// This wrapper type also derives the following traits if `S: Unpin`.
///
/// - [`HookUnmount`]
/// - [`HookPollNextUpdate`]
/// - [`HookValue`] (the [`Value`](HookValue::Value) is wrapped with [`SignalEq`])
/// - [`Hook`] (the value returned by [`use_hook`](Hook::use_hook) is wrapped with [`SignalEq`])
/// - [`SignalHook`](crate::SignalHook)
/// - [`ShareValue`](crate::ShareValue)
/// - [`Signal`](crate::Signal)
/// - [`ToOwnedSignal`](crate::ToOwnedSignal) + [`ToOwnedShareValue`](crate::ToOwnedShareValue) (where `S: ToOwnedSignal`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalEq<S>(pub S);

impl<H: HookUnmount + Unpin> HookUnmount for SignalEq<H> {
    fn unmount(self: Pin<&mut Self>) {
        H::unmount(Pin::new(&mut self.get_mut().0))
    }
}

impl<H: HookPollNextUpdate + Unpin> HookPollNextUpdate for SignalEq<H> {
    fn poll_next_update(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        H::poll_next_update(Pin::new(&mut self.get_mut().0), cx)
    }
}

impl<'hook, H: HookValue<'hook> + Unpin> HookValue<'hook> for SignalEq<H> {
    type Value = SignalEq<H::Value>;
}

impl<H: Hook + Unpin> Hook for SignalEq<H> {
    fn use_hook(self: Pin<&mut Self>) -> <Self as HookValue<'_>>::Value {
        SignalEq(H::use_hook(Pin::new(&mut self.get_mut().0)))
    }
}

#[cfg(feature = "Signal")]
impl<H: crate::SignalHook + Unpin> crate::SignalHook for SignalEq<H>
where
    H::SignalShareValue: PartialEq,
{
    type SignalShareValue = H::SignalShareValue;

    fn to_signal(&self) -> SignalEq<crate::Value<'_, H>> {
        SignalEq(self.0.to_signal())
    }
}

#[cfg(feature = "Signal")]
impl<S, T> crate::ShareValue for SignalEq<S>
where
    S: crate::Signal<Value = T>,
    T: PartialEq,
{
    crate::share_value::proxy_share_value_non_eq!(|self| -> S { &self.0 }, |other| &other.0);

    fn try_unwrap(self) -> Result<Self::Value, Self>
    where
        Self: Sized,
    {
        self.0.try_unwrap().map_err(Self)
    }

    #[inline]
    fn set(&self, new_value: T) {
        self.0.map_mut_and_notify_if(move |old| {
            let changed = *old != new_value;
            *old = new_value;
            ((), changed)
        })
    }

    #[inline]
    fn replace(&self, new_value: T) -> T {
        self.0.map_mut_and_notify_if(move |old| {
            let changed = *old != new_value;
            let old = std::mem::replace(old, new_value);
            (old, changed)
        })
    }

    #[inline]
    fn replace_with<F: FnOnce(&T) -> T>(&self, f: F) -> T {
        self.0.map_mut_and_notify_if(move |v| {
            let new_value = f(v);
            let changed = new_value != *v;
            let old = std::mem::replace(v, new_value);
            (old, changed)
        })
    }
}

#[cfg(feature = "Signal")]
impl<S> crate::Signal for SignalEq<S>
where
    S: crate::Signal,
    S::Value: PartialEq,
    S::SignalHook: Unpin,
{
    type SignalHook = SignalEq<S::SignalHook>;
    type SignalHookUninitialized = S::SignalHookUninitialized;

    fn is_signal_of(&self, signal_hook: &Self::SignalHook) -> bool {
        self.0.is_signal_of(&signal_hook.0)
    }

    fn to_signal_hook(&self) -> Self::SignalHook {
        SignalEq(self.0.to_signal_hook())
    }

    fn update_signal_hook(&self, hook: std::pin::Pin<&mut Self::SignalHook>) {
        self.0.update_signal_hook(Pin::new(&mut hook.get_mut().0))
    }

    fn h_signal_hook<'hook>(
        &self,
        hook: std::pin::Pin<&'hook mut Self::SignalHookUninitialized>,
    ) -> crate::Value<'hook, Self::SignalHook> {
        SignalEq(self.0.h_signal_hook(hook))
    }

    fn notify_changed(&self) {
        self.0.notify_changed()
    }

    fn map_mut_and_notify_if<R>(&self, f: impl FnOnce(&mut Self::Value) -> (R, bool)) -> R {
        self.0.map_mut_and_notify_if(f)
    }
}

#[cfg(feature = "Signal")]
impl<S> crate::ToOwnedShareValue for SignalEq<S>
where
    S: crate::ToOwnedSignal,
    S::Value: PartialEq,
    S::SignalHook: Unpin,
{
    type OwnedShareValue = SignalEq<S::OwnedShareValue>;

    #[inline]
    fn to_owned_share_value(&self) -> Self::OwnedShareValue {
        SignalEq(self.0.to_owned_share_value())
    }
}
