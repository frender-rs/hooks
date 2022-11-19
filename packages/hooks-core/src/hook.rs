use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
    task::Poll,
};

mod sealed {
    pub trait HookLifetimeBounds<'hook, This: ?Sized> {}
    impl<'hook, T: super::HookBounds<Bounds = B> + ?Sized, B: ?Sized> HookLifetimeBounds<'hook, T>
        for &'hook B
    {
    }
}

pub trait HookBounds {
    type Bounds: ?Sized;
}

/// This is a helper trait to define
/// *lifetime generic associated types (lifetime-GAT)*
/// for [`Hook`].
///
/// This trait follows the [*better GAT*] pattern so that
/// *lifetime-GAT* can be used in earlier rust versions.
///
/// This pattern also enables implicit bounds.
/// Comparing to the common *better GAT* pattern
/// which uses `&'a Self` as the implicit bound,
/// this trait uses [`HookBounds`] trait to allow
/// custom implicit bounds.
/// With just `&'a Self`, returning `impl for<'hook> HookLifetime<'hook, Value = ...>`
/// would emit *"return type cannot contain a projection or Self..."* error.
/// This might be a [compiler bug].
///
/// [*better GAT*]: https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats#the-better-gats
/// [compiler bug]: https://github.com/rust-lang/rust/issues/61949#issuecomment-789664939
pub trait HookLifetime<
    'hook,
    Args,
    ImplicitBounds: sealed::HookLifetimeBounds<'hook, Self> = &'hook <Self as HookBounds>::Bounds,
>: HookBounds
{
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
    ///   The executor **CAN** ignore this update, by polling next update
    ///   without calling `use_hook`.
    ///
    /// - `Poll::Ready(false)` means this hook's inner state will never be updated.
    ///   The executor **CAN** no longer call `use_hook` or even drop this hook.
    ///   The executor **CAN** also call `use_hook` to get the value and
    ///   the hook **MIGHT** become dynamic again during `use_hook` or when
    ///   some operations is done to the returned value.
    fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool>;
}

/// ## How to impl `Hook`
///
/// ### with [`hook`](hooks::hook) macro
///
/// Usually, we just need a function which returns a hook,
/// without needing a type which implements `Hook`.
/// With `hook` macro, we can do this easily.
///
/// ```
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
/// To implement `Hook` for a type, implement [HookPollNextUpdate] first.
///
/// You can use [`pin_project_lite::pin_project`] to do pin projections
/// unless you don't the fields to be pinned.
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
pub trait Hook<Args>: HookPollNextUpdate {
    type Value<'hook>
    where
        Self: 'hook;

    fn use_hook<'hook>(self: Pin<&'hook mut Self>, args: Args) -> Self::Value<'hook>
    where
        Self: 'hook;
}

impl<H: HookPollNextUpdate + Unpin + ?Sized> HookPollNextUpdate for &mut H {
    fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        H::poll_next_update(Pin::new(self.get_mut()), cx)
    }
}

impl<H: Hook<Args> + Unpin + ?Sized, Args> Hook<Args> for &mut H {
    type Value<'hook> = H::Value<'hook>
    where
        Self: 'hook;

    fn use_hook<'hook>(self: Pin<&'hook mut Self>, args: Args) -> Self::Value<'hook>
    where
        Self: 'hook,
    {
        H::use_hook(Pin::new(self.get_mut()), args)
    }
}

#[cfg(real_gat_fail)]
/// `NonLendingHook` is a subset of [`Hook`].
/// `Value` of `NonLendingHook` is not generic,
/// thus not borrowing from the hook.
/// In other words, `NonLendingHook` doesn't lend to its `Value`.
///
/// [`Hook`] can be run by executor and become a `LendingAsyncIterator`,
/// `NonLendingHook` can be run by executor and become an [`AsyncIterator`](std::async_iter::AsyncIterator)
/// (also known as [`Stream`](futures_core::Stream)).
pub trait NonLendingHook<Args>:
    for<'hook> Hook<Args, Value<'hook> = Self::NonGenericValue>
{
    type NonGenericValue;
}

#[cfg(real_gat_fail)]
impl<H: ?Sized, Args, V> NonLendingHook<Args> for H
where
    H: for<'hook> Hook<Args, Value<'hook> = Self::NonGenericValue>,
{
    type NonGenericValue = V;
}

impl<H: ?Sized + HookPollNextUpdate + Unpin> HookPollNextUpdate for Box<H> {
    #[inline]
    fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        H::poll_next_update(Pin::new(self.get_mut()), cx)
    }
}

impl<Args, H: ?Sized + Hook<Args> + Unpin> Hook<Args> for Box<H> {
    type Value<'hook> = H::Value<'hook>
    where
        Self: 'hook;

    #[inline]
    fn use_hook<'hook>(self: Pin<&'hook mut Self>, args: Args) -> Self::Value<'hook>
    where
        Self: 'hook,
    {
        <H as Hook<Args>>::use_hook(Pin::new(self.get_mut()), args)
    }
}

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

impl<P, Args> Hook<Args> for Pin<P>
where
    P: DerefMut,
    <P as Deref>::Target: Hook<Args>,
{
    type Value<'hook> = <<P as Deref>::Target as Hook<Args>>::Value<'hook>
    where
        Self: 'hook;

    #[inline]
    fn use_hook<'hook>(self: Pin<&'hook mut Self>, args: Args) -> Self::Value<'hook>
    where
        Self: 'hook,
    {
        <P::Target as Hook<Args>>::use_hook(crate::utils::pin_as_deref_mut(self), args)
    }
}
