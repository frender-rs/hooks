#[macro_export]
macro_rules! __expand_or {
    ([]$($defaults:tt)*) => {
        $($defaults)*
    };
    ([$($t:tt)+]$($defaults:tt)*) => {
        $($t)*
    };
}

#[macro_export]
macro_rules! h {
    [] => {
        ::core::compile_error! {"h! must be used in hook_fn!"}
    };
}

#[macro_export]
macro_rules! __impl_fn_hook_body_finish {
    (
        [
            [] // options
            $rest_ids:tt
        ]
        [] // used_ids
        [$($transformed_code:tt)*]
    ) => {
        move |_: ::core::pin::Pin<&mut $crate::HookTuple<()>>| {
            $($transformed_code)*
        }
    };
    (
        [
            [] // options
            $rest_ids:tt
        ]
        [$used_id:ident] // used_ids
        [$($transformed_code:tt)*]
    ) => {
        move |$used_id : ::core::pin::Pin<&mut _>| {
            $($transformed_code)*
        }
    };
    (
        [
            [] // options
            $rest_ids:tt
        ]
        [$($used_id:ident)+] // used_ids
        [$($transformed_code:tt)*]
    ) => {
        move |__hooks_hook_data: ::core::pin::Pin<&mut _>| {
            // SAFETY: pin projection
            let ($($used_id,)+) = unsafe {
                let $crate::HookTuple(($($used_id,)+)) = ::core::pin::Pin::get_unchecked_mut(__hooks_hook_data);
                ($(
                    ::core::pin::Pin::new_unchecked($used_id),
                )+)
            };

            $($transformed_code)*
        }
    };
}

#[macro_export]
macro_rules! __impl_fn_hook_body_hook_resolved {
    (
        [$id:ident = $($e:tt)+]
        $state:tt
        [$($used_ids:ident)*]
        $hook_ident:ident
        [$($transformed_code:tt)*]
        $code:tt
    ) => {
        $crate::__impl_fn_hook_body! {
            $state
            [$($used_ids)* $id]
            [
                $($transformed_code)*
                $crate::UpdateHookUninitialized::$hook_ident(
                    $($e)+,
                    $id
                )
            ]
            $code
            $code
        }
    };
    (
        [$($e:tt)+]
        [
            $options:tt
            [$id:ident $($rest_ids:ident)*]
        ]
        [$($used_ids:ident)*]
        $hook_ident:ident
        [$($transformed_code:tt)*]
        $code:tt
    ) => {
        $crate::__impl_fn_hook_body! {
            [
                $options
                [$($rest_ids)*]
            ]
            [$($used_ids)* $id]
            [
                $($transformed_code)*
                $crate::UpdateHookUninitialized::$hook_ident(
                    $($e)+,
                    $id
                )
            ]
            $code
            $code
        }
    };
    ( ($($e:tt)*) $($other:tt)* ) => {
        $crate::__impl_fn_hook_body_hook_resolved! { [$($e)*] $($other)* }
    };
    ( {$($e:tt)*} $($other:tt)* ) => {
        $crate::__impl_fn_hook_body_hook_resolved! { [$($e)*] $($other)* }
    };
}

#[macro_export]
macro_rules! __impl_fn_hook_body {
    (
        $state:tt
        $used_ids:tt
        $transformed_code:tt
        [][] // code is empty
    ) => {
        $crate::__impl_fn_hook_body_finish! {
            $state
            $used_ids
            $transformed_code
        }
    };
    (
        $state:tt $used_ids:tt $transformed_code:tt
        [h ! $e:tt $($code:tt)*] [ $hook_ident:ident ! $_e:tt $($_code:tt)* ] // code
    ) => {
        $crate::__impl_fn_hook_body_hook_resolved! {
            $e
            $state
            $used_ids
            $hook_ident
            $transformed_code
            [$($code)*] // code
        }
    };
    (
        $state:tt $used_ids:tt
        [$($transformed_code:tt)*]
        [$t0:tt $($code:tt)*] $_code:tt // code
    ) => {
        $crate::__impl_fn_hook_body! {
            $state $used_ids
            [$($transformed_code)* $t0]
            [$($code)*] [$($code)*]
        }
    };
}

#[macro_export]
macro_rules! __impl_fn_hook_body_start {
    ( $options:tt $code:tt ) => {
        $crate::__impl_fn_hook_body! {
            // state
            [
                $options
                [
                    __hooks_hook_0
                    __hooks_hook_1
                    __hooks_hook_2
                    __hooks_hook_3
                    __hooks_hook_4
                    __hooks_hook_5
                    __hooks_hook_6
                    __hooks_hook_7
                    __hooks_hook_8
                    __hooks_hook_9
                ]
            ]
            [] // used_ids
            [] // transformed_code
            $code
            $code
        }
    };
}

#[macro_export]
macro_rules! __impl_hook_fn_bounds_resolved {
    ($hook_bounds:tt #[hook $(($($options:tt)*))? ] $($rest:tt)*) => {
        $crate::__impl_hook_fn_bounds_and_options_resolved! {
            $hook_bounds ($($($options)*)?)
            $($rest)*
        }
    };
    ($hook_bounds:tt $($rest:tt)*) => {
        $crate::__impl_hook_fn_bounds_and_options_resolved! {
            $hook_bounds ()
            $($rest)*
        }
    };
}

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
macro_rules! __impl_hook_fn_bounds_and_options_resolved {
    (
        $hook_bounds:tt // { 'a + 'b }
        ($($method_path:ident),* $(,)?)

        $(#$attr:tt)*
        $vis:vis fn $name:ident
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
        ($($args:tt)*)
        $( -> $ret_ty:ty )?
        $( where
            __![$($where_clause:tt)*]: __
            $(,)?
        )?
        {
            $(#!$inner_attr:tt)*
            $($code:tt)*
        }
    ) => {
        $(#$attr)*
        $vis fn $name
        $(<$(
            $($lt)?
            $($tp1 $($tp2)?)?
            $(
                :
                $($bound_lt)?
                $(+ $bounds_lt)*
                $(
                    $( + $({$plus_ignore })? )?
                    $( ? $([$relax_ignore])? )?
                    $bounds
                )*
            )?
        ),* >)?
        ($($args)*)
        -> $crate::UpdateHookUninitialized![ $hook_bounds $($ret_ty)? ]
        $(
            where $($where_clause)*
        )?
        {
            $(#!$inner_attr)*

            #[allow(unused_imports)]
            use $crate::prelude_h::*;

            enum __HooksImplNever {}

            struct __HooksValueOfThisHook $(<$(
                $($lt)?
                $($tp1 $($tp2)?)?
                $(
                    :
                    $($bound_lt)?
                    $(+ $bounds_lt)*
                    $(
                        $( + $({$plus_ignore })? )?
                        $( ? $([$relax_ignore])? )?
                        $bounds
                    )*
                )?
            ),* >)?
            $( where $($where_clause)* )?
            {
                __: (
                    __HooksImplNever,
                    $($(
                        $crate::__impl_phantom![
                            $($lt)?
                            $($tp1 $($tp2)?)?
                        ],
                    )*)?
                )
            }

            impl<
                'hook,
                $($(
                    $($lt)?
                    $($tp1 $($tp2)?)?
                    $(
                        :
                        $($bound_lt)?
                        $(+ $bounds_lt)*
                        $(
                            $( + $({$plus_ignore })? )?
                            $( ? $([$relax_ignore])? )?
                            $bounds
                        )*
                    )?
                ),*)?
            > $crate::HookValue<'hook> for __HooksValueOfThisHook $(<$(
                $($lt)?
                $($tp1 $($tp2)?)?
            ),*>)? {
                type Value = $crate::__expand_or![[$($ret_ty)?]()];
            }

            $crate::fn_hook::use_fn_hook $(::$method_path)*
            ::<
                __HooksValueOfThisHook $(<$(
                    $($lt)?
                    $($tp1 $($tp2)?)?
                ),*>)?
                , _, _
            >
            (
                $crate::__impl_fn_hook_body_start!(
                    []
                    [$($code)*]
                )
            )
        }
    };
}

#[macro_export]
macro_rules! __impl_hook_method_poll_next_update {
    (
        $(#$fn_attr:tt)*
        $fn_name:ident () $fn_body:tt
    ) => {
        $crate::__impl_hook_method_poll_next_update! {
            $(#$fn_attr)*
            $fn_name (self, _: _) $fn_body
        }
    };
    (
        $(#$fn_attr:tt)*
        $fn_name:ident (
            $self0:ident
            $($self1:ident)?
            $(,)?
        ) $fn_body:tt
    ) => {
        $crate::__impl_hook_method_poll_next_update! {
            $(#$fn_attr)*
            $fn_name ($self0 $($self1)?, _: _) $fn_body
        }
    };
    (
        $(#$fn_attr:tt)*
        $fn_name:ident (
            $self0:ident
            $($self1:ident)?,
            $cx_pat:tt : _ $(,)?
        ) $fn_body:tt
    ) => {
        $(#$fn_attr)*
        fn $fn_name(
            $self0 $($self1)? : ::core::pin::Pin<&mut Self>,
            $cx_pat: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> $fn_body
    };
}

#[macro_export]
macro_rules! __impl_unmount_fn {
    ({}[$(#$attr:tt)*] $fn_name:ident ($($args:tt)*)) => {
        $crate::__impl_unexpected_token! { $($attr)* }
        $crate::__impl_unexpected_token! { $($args)* }
    };
    (
        $fn_body:tt [$($attr:tt)*] $fn_name:ident (
            $($self_arg:ident)+ $(,)?
        )
    ) => {
        $($attr)*
        fn $fn_name(
            $($self_arg)+ : ::core::pin::Pin<&mut Self>
        ) $fn_body
    };
}

#[macro_export]
macro_rules! __impl_hook_with_method {
    (
        poll_next_update([$($generics:tt)*][$ty:ty][$( $($where_clause:tt)+ )?]) $fn_data:tt
    ) => {
        impl<$($generics)*> $crate::HookPollNextUpdate for $ty $(where $($where_clause)*)? {
            $crate::__impl_hook_method_poll_next_update! $fn_data ;
        }
    };
    (
        unmount([$($generics:tt)*][$ty:ty][$( $($where_clause:tt)+ )?])[
            $(#$fn_attr:tt)*
            $fn_name:ident
            $args:tt
            $fn_body:tt
        ]
    ) => {
        impl<$($generics)*> $crate::HookUnmount for $ty $(where $($where_clause)*)? {
            $crate::__impl_unmount_fn! {
                $fn_body
                [$(#$fn_attr)*]
                $fn_name
                $args
            }
        }
    };
    (
        use_hook([$($generics:tt)*][$ty:ty][$( $($where_clause:tt)+ )?])[
            $(#$fn_attr:tt)*
            $fn_name:ident
            ($self0:ident $($self1:ident)? $(,)?)
            $(-> $ret_ty:ty)?
            {
                $($fn_body:tt)*
            }
        ]
    ) => {
        impl<'hook, $($generics)*> $crate::HookValue<'hook> for $ty $(where $($where_clause)*)? {
            type Value = $crate::__expand_or![
                [$($ret_ty)?]
                ()
            ];
        }

        impl<$($generics)*> $crate::Hook for $ty $(where $($where_clause)*)? {
            $(#$fn_attr)*
            fn $fn_name(
                $self0 $($self1)? : ::core::pin::Pin<&mut Self>
            ) -> <Self as $crate::HookValue<'_>>::Value
            {$($fn_body)*}
        }
    };
    (
        into_hook([$($generics:tt)*][$ty:ty][$( $($where_clause:tt)+ )?])[
            $(#$fn_attr:tt)*
            $fn_name:ident
            ($self0:ident $($self1:ident)?)
            -> $hook_ty:ty
            {$($fn_body:tt)*}
        ]
    ) => {
        impl<$($generics)*> $crate::IntoHook for $ty $(where $($where_clause)*)? {
            type Hook = $hook_ty;
            $(#$fn_attr)*
            fn $fn_name(
                $self0 $($self1)?
            ) -> Self::Hook
            {$($fn_body)*}
        }
    };
    (
        update_hook([$($generics:tt)*][$ty:ty][$( $($where_clause:tt)+ )?])[
            $(#$fn_attr:tt)*
            $fn_name:ident
            ($self0:ident $($self1:ident)?, $hook0:ident $($hook1:ident)? : _ $(,)?)
            $fn_body:tt
        ]
    ) => {
        impl<$($generics)*> $crate::UpdateHook for $ty $(where $($where_clause)*)? {
            $(#$fn_attr)*
            fn $fn_name(
                $self0 $($self1)?,
                $hook0 $($hook1)? : ::core::pin::Pin<&mut Self::Hook>,
            ) $fn_body
        }
    };
    (
        h([$($generics:tt)*][$ty:ty][$( $($where_clause:tt)+ )?])[
            $(#$fn_attr:tt)*
            $fn_name:ident
            ($self0:ident $($self1:ident)?, $hook0:ident $($hook1:ident)? : $ty_uninitialized:ty)
            $(-> $explicit_value:ty)?
            {$($fn_body:tt)*}
        ]
    ) => {
        impl<$($generics)*> $crate::UpdateHookUninitialized for $ty $(where $($where_clause)*)? {
            type Uninitialized = $ty_uninitialized;
            $(#$fn_attr)*
            fn $fn_name(
                $self0 $($self1)?,
                $hook0 $($hook1)? : ::core::pin::Pin<&mut Self::Uninitialized>,
            ) -> $crate::__expand_or![
                [$($explicit_value)?]
                <Self::Hook as $crate::HookValue<'_>>::Value
            ]
            {$($fn_body)*}
        }
    };
}

#[macro_export]
macro_rules! __impl_hook_methods {
    (
        $data:tt
        $(
            $fn_name:ident
            $fn_data:tt
        )*
    ) => {$(
        $crate::__impl_hook_with_method! {
            $fn_name
            $data
            $fn_data
        }
    )*};
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
macro_rules! UpdateHookUninitialized {
    ($bounds:tt) => {
        $crate::UpdateHookUninitialized![$bounds ()]
    };
    ({$($($bounds:tt)+)?} $ty:ty) => {
        impl $crate::UpdateHookUninitialized<
            Hook = impl $crate::Hook + for <'hook> $crate::HookValue<'hook, Value = $ty>
                    $(+ $($bounds)+)?
        > $(+ $($bounds)+)?
    };
}

#[macro_export]
macro_rules! __impl_phantom {
    ($lt:lifetime) => {
        &$lt()
    };
    (const $tp:ident) => {
        ()
    };
    ($tp:ident) => {
        ::core::marker::PhantomData::<$tp>
    };
}

#[macro_export]
macro_rules! Value {
    ($hook_ty:ty $(,)?) => {
        <$hook_ty as $crate::HookValue<'_>>::Value
    };
    ($hook_ty:ty , $lt:lifetime $(,)?) => {
        <$hook_ty as $crate::HookValue<$lt>>::Value
    };
}
