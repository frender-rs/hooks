use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
    task::Poll,
};

mod sealed {
    pub trait HookValueBounds<'hook, This: ?Sized> {}
    impl<'hook, T: ?Sized> HookValueBounds<'hook, T> for &'hook T {}
}

/// This is a helper trait to define
/// *lifetime generic associated types (lifetime-GAT)*
/// for [`Hook`].
///
/// This trait follows the [*better GAT*] pattern so that
/// *lifetime-GAT* can be used in earlier rust versions.
///
/// Please don't impl this trait manually because in the future this will be changed to GAT.
/// Instead, use [`impl_hook![...];`](crate::impl_hook).
///
/// [*better GAT*]: https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats#the-better-gats
/// [compiler bug]: https://github.com/rust-lang/rust/issues/61949#issuecomment-789664939
pub trait HookValue<'hook, ImplicitBounds: sealed::HookValueBounds<'hook, Self> = &'hook Self> {
    /// The output type of [`Hook::use_hook`].
    ///
    /// Please don't use this associated type directly.
    /// Instead, use [`Value![]`](crate::Value) for future compatibility.
    type Value;
}

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

pub trait HookUnmount {
    /// After `unmount`, the hook might still be used or updated.
    #[inline(always)]
    fn unmount(self: Pin<&mut Self>) {}
}

/// ## How to impl `Hook`
///
/// ### with [`fn_hook`](crate::fn_hook) macro
///
///
/// ```
/// # use hooks::use_state;
///
/// fn_hook![
///     pub fn use_my_hook() -> &'hook mut i32 {
///         let (state, updater) = h![use_state(1)];
///         state
///     }
/// ];
/// ```
///
/// ### with [`hook`](hooks::hook) macro
///
/// Usually, we just need a function which returns a hook,
/// without needing a type which implements `Hook`.
/// With `hook` macro, we can do this easily.
///
/// ```
/// # extern crate hooks_dev as hooks;
/// # use hooks::{use_effect, hook};
///
/// /// Print debug on `value` change.
/// #[hook]
/// fn use_debug<'a, T: std::fmt::Debug + Eq + 'a>(value: &'a T) {
///     use_effect(|v: &_| {
///         println!("{v:?}");
///     }, value);
/// }
/// ```
///
/// ### implement `Hook` manually.
///
/// To implement `Hook` for a type, implement
/// [`HookBounds`], [HookLifetime<'hook>] and [HookPollNextUpdate]
/// first.
///
/// ```
/// # use hooks_core::{HookBounds, HookLifetime, HookPollNextUpdate};
///
/// struct MyHook<T>(Option<T>);
///
/// impl<T> HookBounds for MyHook<T> {
///     type Bounds = Self;
/// }
///
/// impl<'hook, T> HookLifetime<'hook, (T,), &'hook Self> for MyHook<T> {
/// //                                       ^^^^^^^^^^^
/// //                                       This must be exactly
/// //                                       `&'hook <Self as HookBounds>::Bounds`
///
///     type Value = &'hook T;
/// //               ^^^^^^^^  We can write `&'hook T` without
/// //                         implicitly specifying `T: 'hook`
/// //                         because `&'hook Self` implies this.
/// }
///
/// impl<T> HookPollNextUpdate for MyHook<T> {
///     fn poll_next_update(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<bool> {
///         todo!()
///     }
/// }
/// ```
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
///   For example, `use_state_clone` clones the data in `use_hook`.
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
///   and this might make the hook dynamic again.
///
///   This behavior makes it possible to combine multiple hooks.
///   When some hooks are no longer dynamic
///   but other hooks depend on their returned values,
///   the executor can still get the values
///   from the no-longer-dynamic hooks,
///   and pass the values to the dynamic hooks.
pub trait Hook: HookPollNextUpdate + HookUnmount + for<'hook> HookValue<'hook> {
    fn use_hook(self: Pin<&mut Self>) -> <Self as HookValue<'_>>::Value;
}

/// `NonLendingHook` is a subset of [`Hook`].
/// `Value` of `NonLendingHook` is not generic,
/// thus not borrowing from the hook.
/// In other words, `NonLendingHook` doesn't lend to its `Value`.
///
/// [`Hook`] can be run by executor and become a `LendingAsyncIterator`,
/// `NonLendingHook` can be run by executor and become an [`AsyncIterator`](std::async_iter::AsyncIterator)
/// (also known as [`Stream`](futures_core::Stream)).
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
