use std::pin::Pin;

use crate::{Hook, HookBounds, HookLifetime, HookPollNextUpdate};

pub trait ValueGat<'hook> {
    type Value;
}

pub trait ErasedHook<Args>: HookPollNextUpdate {
    type ValueGat: ?Sized + for<'hook> ValueGat<'hook>;

    fn erased_use_hook<'hook>(
        self: Pin<&'hook mut Self>,
        args: Args,
    ) -> <Self::ValueGat as ValueGat<'hook>>::Value;
}

impl<Args, H: ?Sized + Hook<Args>> ErasedHook<Args> for H {
    type ValueGat = dyn for<'hook> ValueGat<'hook, Value = <H as HookLifetime<'hook, Args>>::Value>;

    #[inline]
    fn erased_use_hook<'hook>(
        self: Pin<&'hook mut Self>,
        args: Args,
    ) -> <H as HookLifetime<'hook, Args>>::Value {
        <H as Hook<Args>>::use_hook(self, args)
    }
}

/// See also [`dyn_hook`] macro.
pub type DynNonLendingHook<'a, Args, NonGenericValue> =
    dyn 'a + ErasedHook<Args, ValueGat = dyn for<'hook> ValueGat<'hook, Value = NonGenericValue>>;

impl<Args, V: ?Sized> HookBounds for dyn '_ + ErasedHook<Args, ValueGat = V> {
    type Bounds = ();
}

impl<'hook, Args, V: ?Sized + ValueGat<'hook>> HookLifetime<'hook, Args>
    for dyn '_ + ErasedHook<Args, ValueGat = V>
{
    type Value = <V as ValueGat<'hook>>::Value;
}

impl<Args, V: ?Sized + for<'hook> ValueGat<'hook>> Hook<Args>
    for dyn '_ + ErasedHook<Args, ValueGat = V>
{
    #[inline]
    fn use_hook<'hook>(
        self: Pin<&'hook mut Self>,
        args: Args,
    ) -> <Self as HookLifetime<'hook, Args>>::Value
    where
        Self: 'hook,
    {
        Self::erased_use_hook(self, args)
    }
}

impl<Args, V: ?Sized> HookBounds for dyn '_ + ErasedHook<Args, ValueGat = V> + Unpin {
    type Bounds = Self;
}

impl<'hook, Args, V: ?Sized + for<'h> ValueGat<'h>> HookLifetime<'hook, Args>
    for dyn '_ + ErasedHook<Args, ValueGat = V> + Unpin
{
    type Value = <V as ValueGat<'hook>>::Value;
}

impl<Args, V: ?Sized + for<'hook> ValueGat<'hook>> Hook<Args>
    for dyn '_ + ErasedHook<Args, ValueGat = V> + Unpin
{
    #[inline]
    fn use_hook<'hook>(
        self: Pin<&'hook mut Self>,
        args: Args,
    ) -> <Self as HookLifetime<'hook, Args>>::Value
    where
        Self: 'hook,
    {
        Self::erased_use_hook(self, args)
    }
}

/// Use this macro to write [`dyn ErasedHook`](ErasedHook) easily.
///
/// ```
/// # use hooks_core::{ dyn_hook, DynNonLendingHook };
/// # use hooks::{hook, SharedStateData, use_shared_state, use_effect, use_state};
/// let _: Box<dyn_hook![(i32) -> &'hook SharedStateData<i32>]> = Box::new(use_shared_state());
///
/// #[hook]
/// fn use_demo() -> i32 {
///     let (state, updater) = use_state(0);
///     let updater = updater.clone();
///     use_effect(move |v: &i32| {
///         if *v < 2 {
///             updater.set(*v + 1)
///         }
///     }, *state);
///     *state
/// }
/// let _: Box<dyn_hook![() -> i32]> = Box::new(use_demo());
///
/// let _: Box<DynNonLendingHook<(), i32>> = Box::new(use_demo());
/// ```
#[macro_export]
macro_rules! dyn_hook {
    (
        $($lt:lifetime +)?
        $(for < $($generic_lt:lifetime),* $(,)? >)?
        ($($arg:ty),* $(,)?) -> $value:ty
        $(: $($bounds:tt)+)?
    ) => {
        dyn
        $($lt +)?
        $(for < $($generic_lt),* >)?
        $crate::erased_hook::ErasedHook<
            ( $($arg ,)* ),
            ValueGat = dyn for<'hook> $crate::erased_hook::ValueGat<'hook, Value = $value>
        >
        $( + $($bounds)+ )?
    };
}

#[cfg(test)]
mod tests {
    use crate::Hook;

    use super::{ErasedHook, ValueGat};

    macro_rules! assert_impl {
        ($name:ident $([$($generics:tt)*])? ($ty:ty) $trait_ty:ty) => {
            fn $name $(<$($generics)*>)? () -> $trait_ty {
                #![allow(dead_code)]
                #![allow(unreachable_code)]
                #![allow(unused_variables)]
                #![allow(clippy::diverging_sub_expression)]
                #![allow(clippy::type_complexity)]
                let a: $ty = unreachable!();
                a
            }
        };
    }

    #[test]
    fn asserts() {
        assert_impl! {
            boxed_unpin['this](
                Box<dyn 'this + for<'a> ErasedHook<
                            (&'a (),), ValueGat = dyn for<'hook> ValueGat<'hook, Value = ()>
                        > + Unpin
                >
            ) impl for<'a> Hook<(&'a (),)> + 'this
        };

        assert_impl! {
            mut_ref_unpin[Props](
                &mut (dyn for<'a> ErasedHook<
                    (&'a Props,),
                    ValueGat = dyn for<'hook> ValueGat<'hook, Value = ()>,
                > + Unpin)
            ) impl for<'a> Hook<(&'a Props,)>
        };

        assert_impl! {
            mut_dyn_hook_unpin[Props](
                &mut dyn_hook![ for<'a> (&'a Props) -> () : Unpin ]
            ) impl for<'a> Hook<(&'a Props,)>
        };

        assert_impl! {
            pin_box[Props, V](
                std::pin::Pin<Box<dyn_hook![ for<'a> (&'a Props) -> V : Unpin ]>>
            ) impl for<'a> Hook<(&'a Props,), Value = V>
        };
    }
}
