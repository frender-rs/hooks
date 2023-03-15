/// This macro should only be used in [`hook_fn`](crate::hook_fn) or `#[hook]`.
#[macro_export]
macro_rules! h {
    [] => {
        ::core::compile_error! {"h! must be used in hook_fn!"}
    };
}

/// Write hook fn without any proc-macro.
///
/// A *hook fn* is a `fn` which returns [`impl UpdateHookUninitialized`](trait@crate::UpdateHookUninitialized).
/// (Thus the output type is also [`UpdateHook`](crate::UpdateHook) + [`IntoHook`](crate::IntoHook)).
///
/// Tips: `cargo fmt` supports formatting code inside `hook_fn!(...);` and `hook_fn![...];`, but not `hook_fn!{...}`.
///
/// ## Usage
///
/// ```
/// # use hooks_core::prelude::*;
/// hook_fn!(
///     fn use_constant() -> i32 {
///         1
///     }
/// );
///
/// # use hooks_core::{UpdateHookUninitialized, Hook, HookValue};
/// /// `use_constant()` actually returns:
/// # fn assert_0() ->
/// UpdateHookUninitialized![i32]
/// # { use_constant() }
/// /// The above macro expands to
/// # fn assert_1() ->
/// impl UpdateHookUninitialized<
///     Hook = impl Hook + for<'hook> HookValue<'hook, Value = i32>
/// >
/// # { use_constant() }
/// ```
///
/// ## Use other hooks with [`h!(...)`](h)
///
/// Usually, you would want to use other hooks in hook_fn.
///
/// ```
/// # extern crate hooks_dev as hooks;
/// use hooks::prelude::*;
///
/// hook_fn!(
///     fn use_auto_increment(max: i32) -> i32 {
///         let state = h!(use_shared_state(0));
///         let v = state.get();
///
///         h!(use_effect_with::<i32, _>(|old_dependency| {
///             if *old_dependency != Some(v) {
///                 *old_dependency = Some(v);
///                 let state = state.clone();
///                 Some(move |v: &_| state.set(*v + 1))
///             } else {
///                 None
///             }
///         }));
///
///         v
///     }
/// );
/// ```
///
/// ## Borrow from arguments
///
/// You can borrow from arguments,
/// but you need to declare the lifetimes that this hook borrows from.
///
/// ```
/// # extern crate hooks_dev as hooks;
/// use std::rc::Rc;
/// use hooks::prelude::*;
///
/// hook_fn!(
///     type Bounds = impl 'a;
///     fn use_lazy_rc<'a, T: Clone + PartialEq>(value: &'a T) -> Rc<T> {
///         let (rc, _) = h!(use_memo(|value| Rc::new(T::clone(value)), value));
///         rc.clone()
///     }
/// );
/// ```
///
/// <details>
/// <summary>
///
/// There is a limitation that lifetimes must be used in arguments.
/// Phantom lifetimes will fail to compile.
///
/// </summary>
///
/// ```compile_fail
/// # use hooks_core::hook_fn;
/// hook_fn!(
///     fn use_non_used_lifetime<'a>() -> &'a str {
///        "hello"
///     }
/// );
/// ```
///
/// </details>
///
/// ## Use an explicit identifier for hook
///
/// `h!(use_hook())` actually expands to [`use_hook().h(hook_id)`](crate::UpdateHookUninitialized::h),
/// where `hook_id` is of type [`Pin<&mut UpdateHookUninitialized::Uninitialized>`](crate::UpdateHookUninitialized::Uninitialized).
///
/// You can use an explicit `hook_id` for debugging.
///
/// ```
/// # extern crate hooks_dev as hooks;
/// # use hooks::prelude::*;
/// hook_fn!(
///     fn use_my_state() -> i32 {
///         println!("{state_hook:?}"); // inspect this hook before it is used
///
///         let mut the_state_hook = state_hook;
///         let state_hook = std::pin::Pin::as_mut(&mut the_state_hook);
///
///         let (state, updater) = h![state_hook = use_state(1)];
///
///         let state = *state;
///
///         println!("{the_state_hook:?}"); // inspect this hook after it is used
///
///         state
///     }
/// );
/// ```
///
/// ## Limitations
///
/// <details><summary>
///
/// Only *top level token trees* are parsed.
///
/// </summary>
///
/// The following example doesn't work because
/// two macros are in a token tree `()`, which stops them to be parsed.
///
/// ```compile_fail
/// # extern crate hooks_dev as hooks;
/// # use hooks::prelude::*;
/// hook_fn!(
///     fn use_shared_state_2() -> (i32, i32) {
///         (
///             h![use_shared_state(0)].get(),
///             h![use_shared_state(1)].get(),
///         )
///     }
/// );
/// ```
///
/// You have to make sure `h!` to be *top level token trees* when using `hook_fn!`.
///
/// ```
/// # extern crate hooks_dev as hooks;
/// # use hooks::prelude::*;
/// hook_fn!(
///     fn use_shared_state_2() -> (i32, i32) {
///         let a = h![use_shared_state(0)].get();
///         let b = h![use_shared_state(1)].get();
///         (a, b)
///     }
/// );
/// ```
///
/// `#[hook]` parses any *top level expressions* (expressions that are not wrapped in `{}`).
/// Thus, the following example works.
///
/// ```
/// # extern crate hooks_dev as hooks;
/// # use hooks::prelude::*;
/// #[hook]
/// fn use_shared_state_2() -> (i32, i32) {
///     (
///         use_shared_state(0).get(),
///         use_shared_state(1).get(),
///     )
/// }
/// ```
///
/// </details>
///
/// <details><summary>
///
/// Supports at most 10 hooks.
///
/// </summary>
///
/// ```
/// # use hooks_core::hook_fn;
/// # hook_fn!( fn use_hook(_: usize) {} );
/// hook_fn!(
///     fn use_10_hooks() {
///         h!(use_hook(0)); h!(use_hook(1)); h!(use_hook(2)); h!(use_hook(3)); h!(use_hook(4));
///         h!(use_hook(5)); h!(use_hook(6)); h!(use_hook(7)); h!(use_hook(8)); h!(use_hook(9));
///     }
/// );
/// ```
///
/// ```compile_fail
/// # use hooks_core::hook_fn;
/// # hook_fn!( fn use_hook(_: usize) {} );
/// hook_fn!(
///     fn use_11_hooks() {
///         h!(use_hook(0)); h!(use_hook(1)); h!(use_hook(2)); h!(use_hook(3)); h!(use_hook(4));
///         h!(use_hook(5)); h!(use_hook(6)); h!(use_hook(7)); h!(use_hook(8)); h!(use_hook(9));
///         h!(use_hook(10));
///     }
/// );
/// ```
///
/// This limitation also applies to `#[hook]` because
/// traits are only implemented for [`HookTuple<(0, 1, ...)>`](crate::HookTuple)
/// with at most 10 elements.
///
/// ```compile_fail
/// # extern crate hooks_dev as hooks;
/// # use hooks::hook;
/// # hooks_core::hook_fn!( fn use_hook(_: usize) {} );
/// #[hook]
/// fn use_11_hooks() {
///     use_hook(0); use_hook(1); use_hook(2); use_hook(3); use_hook(4);
///     use_hook(5); use_hook(6); use_hook(7); use_hook(8); use_hook(9);
///     use_hook(10);
/// }
/// ```
///
/// </details>
///
/// <details><summary>
///
/// Output type must not be opaque type (impl Trait)
///
/// </summary>
///
/// ```compile_fail
/// # use hooks_core::hook_fn;
/// # use std::future::Future;
/// hook_fn!(
///     fn use_future() -> impl Future<Output = ()> {
///         async {}
///     }
/// );
/// ```
///
/// `#[hook]` allows to do so.
///
/// ```
/// # extern crate hooks_dev as hooks;
/// # use hooks::hook;
/// # use std::future::Future;
/// #[hook]
/// fn use_future() -> impl Future<Output = ()> {
///     async {}
/// }
/// ```
///
/// </details>
#[macro_export]
macro_rules! hook_fn {
    (
        type Bounds = impl $hook_bound:lifetime $(+ $hook_bounds:lifetime)* ;
        $($rest:tt)*
    ) => {
        $crate::__impl_hook_fn_bounds_resolved! {
            { $hook_bound $(+ $hook_bounds)* }
            $($rest)*
        }
    };
    ($($rest:tt)*) => {
        $crate::__impl_hook_fn_bounds_resolved! {
            {}
            $($rest)*
        }
    };
}

/// Easily impl traits.
///
/// For example:
///
/// ```
/// # use hooks_core::impl_hook;
/// struct OutputOnce<T>(pub Option<T>);
///
/// impl_hook!(
///     type For<T: Unpin> = OutputOnce<T>;
///
///     /// HookUnmount is implemented with default `fn unmount()`
///     fn unmount() {}
///
///     /// HookPollNextUpdate is implemented
///     fn poll_next_update(self, _cx: _) {
///         std::task::Poll::Ready(self.get_mut().0.is_some())
///     }
///
///     /// HookValue and Hook is implemented
///     /// HookValue::Value is `&'hook mut Option<T>`
///     #[inline]
///     fn use_hook(self) -> &'hook mut Option<T> {
///         &mut self.get_mut().0
///     }
/// );
/// ```
///
/// ## Declare the type to impl for with `type For = ...;`
///
/// ```
/// # use hooks_core::impl_hook;
/// # struct MyType;
/// impl_hook!(
///     type For = MyType;
/// #   fn poll_next_update(self) { false.into() }
/// );
/// # fn asserts() -> impl hooks_core::HookPollNextUpdate { MyType }
/// ```
///
/// You can declare generics and bounds.
///
/// ```
/// # use hooks_core::impl_hook;
/// # struct MyType<'a, T, F>(&'a T, F);
/// impl_hook!(
///     type For<'a, T: Clone + Default, F: FnMut(&T) -> T> = MyType<'a, T, F>;
/// #   fn poll_next_update(self) { false.into() }
/// );
/// # fn asserts() -> impl hooks_core::HookPollNextUpdate { MyType(&1, Clone::clone) }
/// ```
///
/// Parsing with `macro_rules` is limited.
/// For example, complex bounds are not supported.
///
/// ```compile_fail
/// # use hooks_core::impl_hook;
/// # struct MyType<F: for<'a> FnMut(&'a str) -> &'a str>(F);
/// impl_hook!(
///     type For<F: for<'a> FnMut(&'a str) -> &'a str> = MyType<F>;
/// #   fn poll_next_update(self) { false.into() }
/// );
/// ```
///
/// You can move complex bounds to a special where clause `where __![...]: __`
///
/// ```
/// # use hooks_core::impl_hook;
/// # struct MyType<F: for<'a> FnMut(&'a str) -> &'a str>(F);
/// impl_hook!(
///     type For<F> = MyType<F>
///         where __![F: for<'a> FnMut(&'a str) -> &'a str]: __;
/// #   fn poll_next_update(self) { false.into() }
/// );
/// # fn asserts() -> impl hooks_core::HookPollNextUpdate { MyType(|v: &str| v) }
/// ```
///
/// After declaring the type, you can easily impl traits for this type with methods.
///
/// ## Supported traits
///
/// <details>
/// <summary>
///
/// impl [`HookUnmount`] with [`unmount`](crate::HookUnmount::unmount)
///
/// </summary>
///
/// With default implementation:
///
/// ```
/// # use hooks_core::impl_hook; struct MyType;
/// impl_hook!(
///     type For = MyType;
///     fn unmount() {}
/// );
/// ```
///
/// With custom implementation:
///
/// ```
/// # use hooks_core::impl_hook; struct MyType;
/// # impl MyType { fn do_something(&self) {} }
/// impl_hook!(
///     type For = MyType;
///     fn unmount(self) {
///         self.do_something();
///     }
/// );
/// ```
///
/// </details>
///
/// <details><summary>
///
/// impl [`HookPollNextUpdate`] with [`poll_next_update`](crate::HookPollNextUpdate::poll_next_update)
///
/// </summary>
///
/// Argument `cx` is of type `&mut std::task::Context<'_>`.
/// The return type is `std::task::Poll<bool>`
///
/// ```
/// # use std::future::Future;
/// # use hooks_core::impl_hook;
/// # pin_project_lite::pin_project!( struct MyType {
/// #     #[pin]
/// #     inner: std::future::Ready<bool>,
/// # });
/// impl_hook!(
///     type For = MyType;
///     fn poll_next_update(self, cx: _) {
/// #       let cx: &mut std::task::Context<'_> = cx;
///         self.project().inner.poll(cx)
///     }
/// );
/// ```
///
/// </details>
///
/// <details><summary>
///
/// impl [`HookValue`] and [`Hook`] with [`use_hook`](crate::Hook::use_hook)
///
/// </summary>
///
/// Note that [`Hook`] requires [`HookUnmount`] + [`HookPollNextUpdate`]
///
/// ```
/// # use hooks_core::impl_hook;
/// # #[derive(Clone, Copy)] struct MyValueType; struct MyType(MyValueType);
/// impl_hook!(
///     type For = MyType;
///     fn unmount() {}
///     fn poll_next_update(self, cx: _) { todo!() }
///     fn use_hook(self) -> MyValueType {
///         self.0
///     }
/// );
/// ```
///
/// </details>
///
/// <details><summary>
///
/// impl [`IntoHook`] with [`into_hook`](crate::IntoHook::into_hook)
///
/// </summary>
///
/// ```
/// # use hooks_core::impl_hook; struct UseMyHook(i32); struct MyHook(i32);
/// # impl_hook!( type For = MyHook; fn unmount() {} fn poll_next_update(self, cx: _) { todo!() } fn use_hook(self) {} );
/// impl_hook!(
///     type For = UseMyHook;
///     fn into_hook(self) -> MyHook {
///         MyHook(self.0)
///     }
/// );
/// ```
///
/// </details>
///
/// <details><summary>
///
/// impl [`UpdateHook`] with [`update_hook`](crate::UpdateHook::update_hook)
///
/// </summary>
///
/// Note that [`UpdateHook`] requires [`IntoHook`].
///
/// ```
/// # use hooks_core::impl_hook; struct UseMyHook(i32); struct MyHook(i32);
/// # impl_hook!( type For = MyHook; fn unmount() {} fn poll_next_update(self, cx: _) { todo!() } fn use_hook(self) {} );
/// impl_hook!(
///     type For = UseMyHook;
///     fn into_hook(self) -> MyHook {
///         MyHook(self.0)
///     }
///     fn update_hook(self, mut hook: _) {
///         hook.0 = self.0
///     }
/// );
/// ```
///
/// </details>
///
/// <details><summary>
///
/// impl [`UpdateHookUninitialized`] with [`h`](crate::UpdateHookUninitialized::h)
///
/// </summary>
///
/// Note that [`UpdateHookUninitialized`] requires [`UpdateHook`] + [`IntoHook`].
///
/// ```
/// # use hooks_core::impl_hook; struct UseMyHook(i32); struct MyHook(i32);
/// # #[derive(Default)] struct MyHookUninitialized(Option<i32>);
/// # impl_hook!( type For = MyHook; fn unmount() {} fn poll_next_update(self, cx: _) { todo!() } fn use_hook(self) {} );
/// # impl_hook!( type For = MyHookUninitialized; fn unmount() {} fn poll_next_update(self, cx: _) { todo!() } );
/// impl_hook!(
///     type For = UseMyHook;
///     fn into_hook(self) -> MyHook {
///         MyHook(self.0)
///     }
///     fn update_hook(self, mut hook: _) {
///         hook.0 = self.0
///     }
///     fn h(self, mut hook: MyHookUninitialized) {
///         hook.0.get_or_insert(self.0);
///     }
/// );
/// ```
///
/// </details>
///
/// [`HookUnmount`]: crate::HookUnmount
/// [`HookPollNextUpdate`]: crate::HookPollNextUpdate
/// [`HookValue`]: crate::HookValue
/// [`Hook`]: trait@crate::Hook
/// [`IntoHook`]: crate::IntoHook
/// [`UpdateHook`]: crate::UpdateHook
/// [`UpdateHookUninitialized`]: trait@crate::UpdateHookUninitialized
#[macro_export]
macro_rules! impl_hook {
    (
        type For
            $(<$(
                $($lt:lifetime)?
                $($tp1:ident $($tp2:ident)?)?
                $(
                    :
                    $($bound_lt:lifetime)?
                    $(+ $bounds_lt:lifetime)*
                    $(
                        $( + $({$plus_ignore:tt })? )?
                        $( ? $([$relax_ignore:tt])? )?
                        $bounds:path
                    )*
                )?
            ),* >)?
            = $ty:ty
            $(
                where
                __![$($($where_clause:tt)+)?]: __
                $(,)?
            )?
        ;
        $(
            $(#$fn_attr:tt)*
            fn $fn_name:ident $args:tt $(-> $fn_ret_ty:ty)?
            {$($impl_hook:tt)*}
        )*
    ) => {
        $crate::__impl_hook_methods! {
            (
                [$($(
                    $($lt)?
                    $($tp1 $($tp2)?)?
                    $(
                        :
                        $($bound_lt)?
                        $(+ $bounds_lt)*
                        $(
                            $( + $({$plus_ignore})?  )?
                            $( ? $([$relax_ignore])? )?
                            $bounds
                        )*
                    )?
                ),*)?]
                [$ty]
                [$($($($where_clause)+)?)?]
            )
            $(
                $fn_name [
                    $(#$fn_attr)*
                    $fn_name $args $(-> $fn_ret_ty)?
                    {$($impl_hook)*}
                ]
            )*
        }
    };
}

/// Expands to an opaque type [`impl Hook`](trait@crate::Hook)
/// with type of [`Value`](crate::HookValue::Value).
#[macro_export]
macro_rules! Hook {
    ($value:ty $(, $($($bounds:tt)+)?)? ) => {
        impl $crate::Hook + for<'hook> $crate::HookValue<'hook, Value = $value>
            $($(+ $($bounds)+)?)?
    };
}

/// Expands to an opaque type [`impl UpdateHookUninitialized`](trait@crate::UpdateHookUninitialized)
/// with type of [`Hook`](crate::IntoHook::Hook)`::`[`Value`](crate::HookValue::Value).
#[macro_export]
macro_rules! UpdateHookUninitialized {
    ($value:ty $(, $($($bounds:tt)+)?)?) => {
        impl $crate::UpdateHookUninitialized<
            Hook = $crate::Hook![$value $($(, $($bounds)+)?)?]
        > $($(+ $($bounds)+)?)?
    };
}

/// Use this macro to get [`HookValue::Value`][crate::HookValue::Value].
///
/// In the future, when [`HookValue`](crate::HookValue) changed to GAT,
/// this macro would still works.
#[macro_export]
macro_rules! Value {
    ($hook_ty:ty $(,)?) => {
        <$hook_ty as $crate::HookValue<'_>>::Value
    };
    ($hook_ty:ty , $lt:lifetime $(,)?) => {
        <$hook_ty as $crate::HookValue<$lt>>::Value
    };
}
