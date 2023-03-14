use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
    task::Poll,
};

mod sealed {
    pub trait HookValueBounds<'hook, This: ?Sized> {}
    impl<'hook, T: ?Sized> HookValueBounds<'hook, T> for &'hook T {}
}

/// A helper trait to define
/// *lifetime generic associated types (lifetime-GAT)*
/// for [`Hook`].
///
/// This trait follows the [*better GAT*] pattern.
/// *better GAT* is a pattern which implements *lifetime-GAT* without real GAT,
/// and it also solves some problems relating to `for<'hook> HookValue<Value<'hook> = ...>`
/// that real GAT currently doesn't solve.
///
/// Please don't impl this trait manually because in the future this will be changed to GAT.
/// Instead, use [`impl_hook![...];`](crate::impl_hook).
///
/// [*better GAT*]: https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats#the-better-gats
pub trait HookValue<'hook, ImplicitBounds: sealed::HookValueBounds<'hook, Self> = &'hook Self> {
    /// The output type of [`Hook::use_hook`].
    ///
    /// Please don't use this associated type directly.
    /// Instead, use [`Value![]`](crate::Value) for future compatibility.
    type Value;
}

/// Defines reactivity of a hook.
///
/// See [`poll_next_update()`](HookPollNextUpdate::poll_next_update) for details.
pub trait HookPollNextUpdate {
    /// The meaning of the return value is:
    ///
    /// - `Poll::Pending` means this hook's inner state is not updated
    ///   after the last `use_hook`.
    ///   The executor **DO NOT NEED** to call `use_hook` again
    ///   because the returned value is expected to remain the same
    ///   as the value from the last call.
    ///   The executor **CAN** still call `use_hook`
    ///   to re-get the returned value.
    ///
    /// - `Poll::Ready(true)` means this hook's inner state has been updated
    ///   since the last `use_hook`.
    ///   The executor **SHOULD** call `use_hook` again to get the new value.
    ///   If the executor doesn't update or use this hook, instead, it polls the hook again,
    ///   the hook may still return `Poll::Ready(true)`.
    ///
    /// - `Poll::Ready(false)` means this hook's inner state will never be updated.
    ///   The executor **CAN** drop this hook.
    ///   The executor **CAN** call `use_hook` to get the value or update it, and
    ///   the hook **MIGHT** become dynamic again during `use_hook` or `update_hook`, or when
    ///   some operations is done to the returned value.
    fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool>;
}

/// Defines how to cleanup a hook.
///
/// Consider this as a re-entrant and pinned version of [`Drop`].
/// Cleanups can be run in [`unmount`](HookUnmount::unmount).
///
/// After `unmount`, the hook might still be used or updated.
pub trait HookUnmount {
    #[inline(always)]
    fn unmount(self: Pin<&mut Self>) {}
}

/// Defines how to use a hook (get value from the hook).
///
/// A hook is something that outputs values reactively.
///
/// ## How to impl `Hook`
///
/// Usually, you don't need to impl `Hook`.
/// You can easily compose hooks with [`hook_fn!(...)`] or
/// [`#[hook]`](https://docs.rs/hooks/latest/hooks/attr.hook.html).
///
/// The hook fn actually returns `impl UpdateHookUninitialized<Hook = impl Hook>`,
/// so that this hook fn can also be composed in other `hook_fn`.
/// For more information, see [`hook_fn!(...)`].
///
/// ### with `hook_fn!(...)` macro
///
/// ```
/// # extern crate hooks_dev as hooks;
/// # use hooks::{use_state, hook_fn};
/// hook_fn![
///     pub fn use_my_hook() -> &'hook mut i32 {
///         let (state, updater) = h![use_state(1)];
///         state
///     }
/// ];
/// ```
///
/// ### with `#[hook]` attribute macro
///
/// ```
/// # extern crate hooks_dev as hooks;
/// # use hooks::{use_effect, hook};
/// /// Print debug on `value` change.
/// #[hook]
/// fn use_debug<T: std::fmt::Debug + PartialEq + Copy>(value: T) {
///     use_effect(|v: &_| {
///         println!("{v:?}");
///     }, value);
/// }
/// ```
///
/// ### implement `Hook` manually.
///
/// See [`impl_hook!`](crate::impl_hook).
///
/// ## Comparison with `LendingAsyncIterator`
///
/// A `Hook` is like a `LendingAsyncIterator`.
/// They both produce items asynchronously,
/// but they have different meanings on pending and terminating:
///
/// For pending:
///
/// - If a `LendingAsyncIterator` is pending
///   (`poll_next` returns `Poll::Pending`),
///   it is producing the next item.
///
/// - If a `Hook` is pending,
///   (`poll_next_update` returns `Poll::Pending`),
///   it is waiting for its inner state to update.
///   When a `Hook` is pending, the executor can still *use* it
///   by calling [`use_hook`](Hook::use_hook) and
///   the returned value would remain the *same* as the last returned value.
///   *Using* a hook is like *inspecting* it.
///   Some hooks may do heavy work in `use_hook`.
///   It is advised to call `use_hook` only after
///   `poll_next_update` returns `Poll::Ready(true)`.
///
/// For terminating:
///
/// - If a `LendingAsyncIterator` is terminated
///   (`poll_next` returns `Poll::Ready(None)`),
///   the executor MUST NOT call `poll_next` again.
///
/// - There is no termination for a `Hook` until dropped.
///   When `poll_next_update` returns `Poll::Ready(false)`,
///   this means the hook is no longer dynamic
///   (its inner state will no longer update).
///   Thus, there is no need to call `use_hook` again because
///   the returned value is expected to remain the *same*.
///   But the executor can still call `use_hook` to re-get the value
///   or update it with [`update_hook`](crate::UpdateHook::update_hook) or [`h`](crate::UpdateHookUninitialized::h),
///   and this might make the hook dynamic again.
///
///   This behavior makes it possible to combine multiple hooks.
///   When some hooks are no longer dynamic
///   but other hooks depend on their returned values,
///   the executor can still get the values
///   from the no-longer-dynamic hooks,
///   and pass the values to the dynamic hooks.
///
/// Also see [`NonLendingHook`] for a subset of hooks that doesn't lend lifetimes to values,
/// which are like [`AsyncIterator`](std::async_iter::AsyncIterator) or [`Stream`](futures_core::Stream).
///
/// [`hook_fn!(...)`]: crate::hook_fn
pub trait Hook: HookPollNextUpdate + HookUnmount + for<'hook> HookValue<'hook> {
    fn use_hook(self: Pin<&mut Self>) -> <Self as HookValue<'_>>::Value;
}

/// `NonLendingHook` is a subset of [`Hook`].
/// `Value` of `NonLendingHook` is not generic,
/// thus not borrowing from the hook.
/// In other words, `NonLendingHook` doesn't lend to its `Value`.
///
/// [`Hook`] is like a `LendingAsyncIterator`,
/// `NonLendingHook` is like an [`AsyncIterator`](std::async_iter::AsyncIterator)
/// (also known as [`Stream`](futures_core::Stream)).
///
/// You can run
/// [`hook.into_values()`](crate::HookExt::into_values) and [`hook.values()`](crate::HookExt::values)
/// to get a stream of values from a hook.
pub trait NonLendingHook:
    Hook + for<'hook> HookValue<'hook, Value = Self::NonGenericValue>
{
    type NonGenericValue;
}

impl<H, V> NonLendingHook for H
where
    H: Hook + for<'hook> HookValue<'hook, Value = V>,
{
    type NonGenericValue = V;
}

macro_rules! impl_for_deref_hook {
    (
        impl<$h:ident> (
            $($ty:ty),*
            $(,)?
        ) {
            $poll_next_update:item
            $unmount:item
            $use_hook:item
        }
    ) => {$(
        impl<H: HookPollNextUpdate + Unpin + ?Sized> HookPollNextUpdate for $ty {
            $poll_next_update
        }

        impl<H: HookUnmount + Unpin + ?Sized> HookUnmount for $ty {
            $unmount
        }

        impl<'hook, H: Hook + Unpin + ?Sized> HookValue<'hook> for $ty {
            type Value = <H as HookValue<'hook>>::Value;
        }

        impl<H: Hook + Unpin + ?Sized> Hook for $ty {
            $use_hook
        }
    )*};
}

impl_for_deref_hook![
    impl<H> (&mut H, Box<H>) {
        #[inline]
        fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
            H::poll_next_update(Pin::new(self.get_mut()), cx)
        }

        #[inline]
        fn unmount(self: Pin<&mut Self>) {
            H::unmount(Pin::new(self.get_mut()))
        }

        #[inline]
        fn use_hook(self: Pin<&mut Self>) -> <Self as HookValue<'_>>::Value {
            H::use_hook(Pin::new(self.get_mut()))
        }
    }
];

impl<P> HookPollNextUpdate for Pin<P>
where
    P: DerefMut,
    <P as Deref>::Target: HookPollNextUpdate,
{
    #[inline]
    fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        <P::Target as HookPollNextUpdate>::poll_next_update(
            crate::utils::pin_as_deref_mut(self),
            cx,
        )
    }
}

impl<P> HookUnmount for Pin<P>
where
    P: DerefMut,
    <P as Deref>::Target: HookUnmount,
{
    #[inline]
    fn unmount(self: Pin<&mut Self>) {
        <<P as Deref>::Target as HookUnmount>::unmount(crate::utils::pin_as_deref_mut(self))
    }
}

impl<'hook, P> HookValue<'hook> for Pin<P>
where
    P: DerefMut,
    <P as Deref>::Target: Hook,
{
    type Value = <<P as Deref>::Target as HookValue<'hook>>::Value;
}

impl<P> Hook for Pin<P>
where
    P: DerefMut,
    <P as Deref>::Target: Hook,
{
    #[inline]
    fn use_hook(self: Pin<&mut Self>) -> <Self as HookValue<'_>>::Value {
        <P::Target as Hook>::use_hook(crate::utils::pin_as_deref_mut(self))
    }
}

pub trait IntoHook {
    type Hook: Hook;

    fn into_hook(self) -> Self::Hook;

    #[inline(always)]
    fn into_hook_values(self) -> crate::Values<Self::Hook>
    where
        Self: Sized,
    {
        crate::Values::new(self.into_hook())
    }
}

impl<H> IntoHook for H
where
    H: Hook,
{
    type Hook = H;

    #[inline(always)]
    fn into_hook(self) -> Self::Hook {
        self
    }
}

pub trait UpdateHook: IntoHook {
    fn update_hook(self, hook: Pin<&mut Self::Hook>);
}

pub trait UpdateHookUninitialized: UpdateHook {
    type Uninitialized: HookPollNextUpdate + HookUnmount + Default;

    fn h(self, hook: Pin<&mut Self::Uninitialized>) -> <Self::Hook as HookValue<'_>>::Value;
}
