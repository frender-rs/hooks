use std::{pin::Pin, task::Poll};

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
/// ### with [`hook`] macro
///
/// Usually, we just need a function which returns a hook,
/// without needing a type which implements `Hook`.
/// With `hook` macro, we can do this easily.
///
/// ```
/// /// Print debug on `value` change.
/// #[hook]
/// fn use_debug<T: std::fmt::Debug + Eq>(value: &'hook T) {
///     use_effect_by_ref(|v| {
///         println!("{v:?}");
///     }, value);
/// }
/// ```
///
/// ### implement `Hook` manually.
///
/// To implement `Hook` for a type, implement
/// [`HookBounds`] and [HookLifetime<'hook>]
/// first.
///
/// ```
/// # use hooks_core::{HookBounds, HookLifetime};
///
/// struct MyHook<T>(Option<T>);
///
/// impl<T> HookBounds for MyHook<T> {
///     type Bounds = Self;
/// }
///
/// impl<'hook, T> HookLifetime<'hook, &'hook Self> for MyHook<T> {
/// //                                 ^^^^^^^^^^^
/// //                                 This must be exactly
/// //                                 `&'hook <Self as HookBounds>::Bounds`
///
///     type Args = (T,);
///     type Value = &'hook T;
/// //               ^^^^^^^^  We can write `&'hook T` without
/// //                         implicitly specifying `T: 'hook`
/// //                         because `&'hook Self` implies this.
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
pub trait Hook<Args>: HookPollNextUpdate + for<'hook> HookLifetime<'hook, Args> {
    fn use_hook<'hook>(
        self: Pin<&'hook mut Self>,
        args: Args,
    ) -> <Self as HookLifetime<'hook, Args>>::Value
    where
        Self: 'hook;
}

impl<H: HookBounds + ?Sized> HookBounds for &mut H {
    type Bounds = H::Bounds;
}

impl<'hook, Args, H: HookLifetime<'hook, Args> + ?Sized> HookLifetime<'hook, Args> for &mut H {
    type Value = <H as HookLifetime<'hook, Args>>::Value;
}

impl<H: HookPollNextUpdate + Unpin + ?Sized> HookPollNextUpdate for &mut H {
    fn poll_next_update(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        H::poll_next_update(Pin::new(self.get_mut()), cx)
    }
}

impl<H: Hook<Args> + Unpin + ?Sized, Args> Hook<Args> for &mut H {
    fn use_hook<'hook>(
        self: Pin<&'hook mut Self>,
        args: Args,
    ) -> <Self as HookLifetime<'hook, Args>>::Value
    where
        Self: 'hook,
    {
        H::use_hook(Pin::new(self.get_mut()), args)
    }
}

/// `NonLendingHook` is a subset of [`Hook`].
/// `Value` of `NonLendingHook` is not generic,
/// thus not borrowing from the hook.
/// In other words, `NonLendingHook` doesn't lend to its `Value`.
///
/// [`Hook`] can be run by executor and become a `LendingAsyncIterator`,
/// `NonLendingHook` can be run by executor and become an `AsyncIterator`
/// (also known as [`Stream`](futures_lite::Stream)).
pub trait NonLendingHook<Args>:
    Hook<Args> + for<'hook> HookLifetime<'hook, Args, Value = Self::NonGenericValue>
{
    type NonGenericValue;
}

impl<H: ?Sized, Args, V> NonLendingHook<Args> for H
where
    H: Hook<Args> + for<'hook> HookLifetime<'hook, Args, Value = V>,
{
    type NonGenericValue = V;
}
