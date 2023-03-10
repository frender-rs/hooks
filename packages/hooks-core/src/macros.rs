/// This macro should only be used in [`hook_fn`].
#[macro_export]
macro_rules! h {
    [] => {
        ::core::compile_error! {"h! must be used in hook_fn!"}
    };
}

/// This macro enables writing hook fn without any proc-macro.
///
/// A *hook fn* is a `fn` which returns [`impl UpdateHookUninitialized`](crate::UpdateHookUninitialized).
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
/// but you need to declare the lifetimes this hook borrows from.
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
/// There is a limitation that lifetimes must be used.
/// Otherwise, you should use [`#[hook]`](hooks::hook) with `#[phantom]`.
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

#[macro_export]
macro_rules! Hook {
    ($value:ty $(, $($($bounds:tt)+)?)? ) => {
        impl $crate::Hook + for<'hook> $crate::HookValue<'hook, Value = $value>
            $($(+ $($bounds)+)?)?
    };
}

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
