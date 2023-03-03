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
macro_rules! hook {
    (must_be_used_in_fn_hook $method:ident $hook_id:ident $e:expr) => {
        $crate::v2::UpdateHookUninitialized::$method(
            $e,
            $hook_id
        )
    };
}

#[macro_export]
macro_rules! __impl_fn_hook_body_finish {
    (
        [
            [] // options
            $rest_ids:tt
        ]
        $used_ids:tt
        $transformed_code:tt
    ) => {
        $crate::__impl_fn_hook_body_finish! {
            [
                [(FnMut)]
                $rest_ids
            ]
            $used_ids
            $transformed_code
        }
    };
    (
        [
            [($method:ident)] // options
            $rest_ids:tt
        ]
        [] // used_ids
        [$($transformed_code:tt)*]
    ) => {
        $crate::v2::fn_hook::UpdateFnHook::$method(
            (),
            move |_| {
                $($transformed_code)*
            }
        )
    };
    (
        [
            [($method:ident)] // options
            $rest_ids:tt
        ]
        [$used_id:ident] // used_ids
        [$($transformed_code:tt)*]
    ) => {
        $crate::v2::fn_hook::UpdateFnHook::$method(
            move |$used_id| {
                $($transformed_code)*
            }
        )
    };
    (
        [
            [($method:ident)] // options
            $rest_ids:tt
        ]
        [$($used_id:ident)+] // used_ids
        [$($transformed_code:tt)*]
    ) => {
        // $crate::v2::HookTuple(($(
        //     $crate::__expand_or!(
        //         [::core::default::Default::default()]
        //         $used_id
        //     ),
        // )+)) // TODO: remove
        $crate::v2::fn_hook::UpdateFnHook::$method(
            move |__frender_hook_data: ::core::pin::Pin<&mut _>| {
                // SAFETY: pin projection
                let ($($used_id,)+) = unsafe {
                    let $crate::v2::HookTuple(($($used_id,)+)) = ::core::pin::Pin::get_unchecked_mut(__frender_hook_data);
                    ($(
                        ::core::pin::Pin::new_unchecked($used_id),
                    )+)
                };

                $($transformed_code)*
            }
        )
    };
}

#[macro_export]
macro_rules! __impl_fn_hook_body_hook_resolved {
    (
        [$id:ident = $e:expr]
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
                $hook_ident! { called_in_fn_hook $hook_ident $id $e }
            ]
            $code
            $code
        }
    };
    (
        [$e:expr]
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
                $hook_ident! { must_be_used_in_fn_hook $hook_ident $id $e }
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
        [hook ! $e:tt $($code:tt)*] [ $hook_ident:ident ! $_e:tt $($_code:tt)* ] // code
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
                    __frender_hook_0
                    __frender_hook_1
                    __frender_hook_2
                    __frender_hook_3
                    __frender_hook_4
                    __frender_hook_5
                    __frender_hook_6
                    __frender_hook_7
                    __frender_hook_8
                    __frender_hook_9
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
macro_rules! fn_hook {
    (
        $(#[hook $options:tt])?
        $(#$attr:tt)*
        $vis:vis fn $name:ident
        $(<$(
            $($lt:lifetime)?
            $($tp1:ident $($tp2:ident)?)?
            $(
                :
                $($bound_lt:lifetime)?
                $(+ $bounds_lt:lifetime)*
                $(? $([$relax_ignore:tt])?  )?
                $($bounds:path)?
            )?
        ),* $(,)? >)?
        ($($args:tt)*)
        $( -> $ret_ty:ty )?
        $( where
            __![$($where_clause:tt)*]: __,
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
                $(? $([$relax_ignore] +)?  )?
                $($bounds)?
            )?
        ),*>)?
        ($($args)*)
        -> $crate::Hook![$($ret_ty)?]
        $(
            where $($where_clause)*
        )?
        {
            $(#!$inner_attr)*

            $crate::__impl_fn_hook_body_start!(
                [$($options)?]
                [$($code)*]
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
macro_rules! __impl_hook_with_method {
    (
        poll_next_update([$($generics:tt)*][$ty:ty][$( $($where_clause:tt)+ )?]) $fn_data:tt
    ) => {
        impl<$($generics)*> $crate::HookPollNextUpdate for $ty $(where $($where_clause)*)? {
            $crate::__impl_hook_method_poll_next_update! $fn_data ;
        }
    };
    (
        use_value([$($generics:tt)*][$ty:ty][$( $($where_clause:tt)+ )?])[
            $(#$fn_attr:tt)*
            $fn_name:ident
            ($self0:ident $($self1:ident)?)
            $(-> $ret_ty:ty)?
            {$($fn_body:tt)*}
        ]
    ) => {
        impl<$($generics)*> $crate::v2::Hook for $ty $(where $($where_clause)*)? {
            type Value<'hook> = $crate::__expand_or![
                [$($ret_ty)?]
                ()
            ] where Self: 'hook;
            fn $fn_name(
                $self0 $($self1)? : ::core::pin::Pin<&mut Self>
            ) -> Self::Value<'_>
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
macro_rules! v2_impl_hook {
    (
        const _
            : $ty:ty
            $(
                =
                $(Generics![$($generics:tt)*] $(+)?)?
                $(Where![$($where_clause:tt)*] $(+)?)?
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
                [$($($($generics)*)?)?]
                [$ty]
                [$($($($where_clause)*)?)?]
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
    () => {
        $crate::Hook![()]
    };
    ($ty:ty) => {
        impl $crate::v2::UpdateHookUninitialized<
            Hook = impl for<'hook> $crate::v2::Hook<Value<'hook> = $ty>
        >
    };
}
