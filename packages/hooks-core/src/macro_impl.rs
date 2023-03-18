#[doc(hidden)]
#[macro_export]
macro_rules! __impl_unexpected_token {
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_or {
    ([]$($defaults:tt)*) => {
        $($defaults)*
    };
    ([$($t:tt)+]$($defaults:tt)*) => {
        $($t)*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_fn_hook_body_finish_as_closure {
    (
        // options
        [
            $(append_args_pat! { $($append_args_pat:tt)+ })?
        ]
        [$($args_pat:tt)*]
        $body:tt
    ) => {
        move |
            $($args_pat)*
            $(, $($append_args_pat)+ )?
        | $body
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_fn_hook_body_finish {
    (
        [
            $options:tt
            $rest_ids:tt
        ]
        [] // used_ids
        [$($transformed_code:tt)*]
    ) => {
        $crate::__impl_fn_hook_body_finish_as_closure! {
            $options
            [_: ::core::pin::Pin<&mut $crate::HookTuple<()>>]
            { $($transformed_code)* }
        }
    };
    (
        [
            $options:tt
            $rest_ids:tt
        ]
        [$used_id:ident] // used_ids
        [$($transformed_code:tt)*]
    ) => {
        $crate::__impl_fn_hook_body_finish_as_closure! {
            $options
            [$used_id : ::core::pin::Pin<&mut _>]
            { $($transformed_code)* }
        }
    };
    (
        [
            $options:tt
            $rest_ids:tt
        ]
        [$($used_id:ident)+] // used_ids
        [$($transformed_code:tt)*]
    ) => {
        $crate::__impl_fn_hook_body_finish_as_closure! {
            $options
            [__hooks_hook_data: ::core::pin::Pin<&mut _>]
            {
            // SAFETY: pin projection
            let ($($used_id,)+) = unsafe {
                let $crate::HookTuple(($($used_id,)+)) = ::core::pin::Pin::get_unchecked_mut(__hooks_hook_data);
                ($(
                    ::core::pin::Pin::new_unchecked($used_id),
                )+)
            };

            $($transformed_code)*
            }
        }
    };
}

#[doc(hidden)]
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

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_fn_hook_body {
    (
        $state:tt
        $used_ids:tt
        $transformed_code:tt
        {}{} // code is empty
    ) => {
        $crate::__impl_fn_hook_body_finish! {
            $state
            $used_ids
            $transformed_code
        }
    };
    (
        $state:tt $used_ids:tt $transformed_code:tt
        {h ! $e:tt $($code:tt)*} { $hook_ident:ident ! $_e:tt $($_code:tt)* } // code
    ) => {
        $crate::__impl_fn_hook_body_hook_resolved! {
            $e
            $state
            $used_ids
            $hook_ident
            $transformed_code
            {$($code)*} // code
        }
    };
    // + h![]
    (
        $state:tt $used_ids:tt [$($transformed_code:tt)*]
        {$t0:tt h ! $e:tt $($code:tt)*} {$_t0:tt $hook_ident:ident ! $_e:tt $($_code:tt)* } // code
    ) => {
        $crate::__impl_fn_hook_body_hook_resolved! {
            $e
            $state
            $used_ids
            $hook_ident
            [$($transformed_code)* $t0]
            {$($code)*} // code
        }
    };
    // 1 + h![]
    (
        $state:tt $used_ids:tt [$($transformed_code:tt)*]
        {$t0:tt $t1:tt h ! $e:tt $($code:tt)*} {$_t0:tt $_t1:tt $hook_ident:ident ! $_e:tt $($_code:tt)* } // code
    ) => {
        $crate::__impl_fn_hook_body_hook_resolved! {
            $e
            $state
            $used_ids
            $hook_ident
            [$($transformed_code)* $t0 $t1]
            {$($code)*} // code
        }
    };
    // + 1 + h![]
    (
        $state:tt $used_ids:tt [$($transformed_code:tt)*]
        {$t0:tt $t1:tt $t2:tt h ! $e:tt $($code:tt)*} {$_t0:tt $_t1:tt $_t2:tt $hook_ident:ident ! $_e:tt $($_code:tt)* } // code
    ) => {
        $crate::__impl_fn_hook_body_hook_resolved! {
            $e
            $state
            $used_ids
            $hook_ident
            [$($transformed_code)* $t0 $t1 $t2]
            {$($code)*} // code
        }
    };
    // 4
    (
        $state:tt $used_ids:tt
        [$($transformed_code:tt)*]
        {$t0:tt $t1:tt $t2:tt $t3:tt $($code:tt)*} $_code:tt // code
    ) => {
        $crate::__impl_fn_hook_body! {
            $state $used_ids
            [$($transformed_code)* $t0 $t1 $t2 $t3]
            {$($code)*} {$($code)*}
        }
    };
    // 3
    (
        $state:tt $used_ids:tt
        [$($transformed_code:tt)*]
        {$t0:tt $t1:tt $t2:tt $($code:tt)*} $_code:tt // code
    ) => {
        $crate::__impl_fn_hook_body! {
            $state $used_ids
            [$($transformed_code)* $t0 $t1 $t2]
            {$($code)*} {$($code)*}
        }
    };
    // 2
    (
        $state:tt $used_ids:tt
        [$($transformed_code:tt)*]
        {$t0:tt $t1:tt $($code:tt)*} $_code:tt // code
    ) => {
        $crate::__impl_fn_hook_body! {
            $state $used_ids
            [$($transformed_code)* $t0 $t1]
            {$($code)*} {$($code)*}
        }
    };
    (
        $state:tt $used_ids:tt
        [$($transformed_code:tt)*]
        {$t0:tt $($code:tt)*} $_code:tt // code
    ) => {
        $crate::__impl_fn_hook_body! {
            $state $used_ids
            [$($transformed_code)* $t0]
            {$($code)*} {$($code)*}
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_hook_fn_bounds_resolved {
    ($hook_bounds:tt #[hook $(($($options:tt)*))? ] $($rest:tt)*) => {
        $crate::__private::parse_item_fn! {
            [$hook_bounds ($($($options)*)?)]
            {$($rest)*} => $crate::__impl_hook_fn_item_fn_parsed!
        }
    };
    ($hook_bounds:tt $($rest:tt)*) => {
        $crate::__private::parse_item_fn! {
            [$hook_bounds ()]
            {$($rest)*} => $crate::__impl_hook_fn_item_fn_parsed!
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_hook_fn_item_fn_parsed {
    (
        { $($hook_bounds:tt)* } // { 'a + 'b }
        ($($method_path:ident),* $(,)?)
        item_fn! {
            outer_attrs! { $($outer_attrs:tt)* }
            vis! { $vis:vis }
            sig! {
                ident! { $name:ident }
                generics! {
                    params! { $($generic_params:tt)* }
                    impl_generics! $impl_generics:tt
                    type_generics! { $($type_generics:tt)* }
                    params_name! $params_name:tt
                }
                paren_inputs! { $paren_inputs:tt }
                output! { $(-> $ret_ty:ty)? }
                where_clause! { $($where_clause:tt)* }
            }
            inner_attrs! { $($inner_attrs:tt)* }
            stmts! { $($stmts:tt)* }
        }
        rest! {}
    ) => {
        $($outer_attrs)*
        $vis fn $name
        <$($generic_params)*>
        $paren_inputs
        -> $crate::UpdateHookUninitialized![ $crate::__private::expand_or![[$($ret_ty)?]()], $($hook_bounds)* ]
        $($where_clause)*
        {
            $($inner_attrs)*

            #[allow(unused_imports)]
            use $crate::prelude_h::*;

            enum __HooksImplNever {}

            struct __HooksValueOfThisHook <$($generic_params)*>
            $($where_clause)*
            {
                __: $crate::__impl_phantoms![
                    __HooksImplNever,
                    $params_name
                ]
            }

            impl<
                'hook,
                $($generic_params)*
            > $crate::HookValue<'hook> for __HooksValueOfThisHook <$($type_generics)*>
            $($where_clause)*
            {
                type Value = $crate::__private::expand_or![[$($ret_ty)?]()];
            }

            $crate::fn_hook::use_fn_hook $(::$method_path)*
            ::<
                __HooksValueOfThisHook <$($type_generics)*>
                , _, _
            >
            (
                $crate::transform_hook_fn_body_as_closure!(
                    []
                    {$($stmts)*}
                )
            )
        }
    };
}

#[doc(hidden)]
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

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_trait_hook_unmount {
    (
        ([$($generics:tt)*][$ty:ty][$( $($where_clause:tt)+ )?])[
            $(#$fn_attr:tt)*
            $fn_name:ident
            ($($args:tt)*)
            {}
        ]
    ) => {
        $crate::__impl_unexpected_token! { $($args)* }

        $(#$fn_attr)*
        impl<$($generics)*> $crate::HookUnmount for $ty $(where $($where_clause)*)? {}
    };
    (
        ([$($generics:tt)*][$ty:ty][$( $($where_clause:tt)+ )?])[
            $(#$fn_attr:tt)*
            $fn_name:ident
            ( $($self_arg:ident)+ $(,)? )
            $fn_body:tt
        ]
    ) => {
        impl<$($generics)*> $crate::HookUnmount for $ty $(where $($where_clause)*)? {
            $(#$fn_attr)*
            fn $fn_name(
                $($self_arg)+ : ::core::pin::Pin<&mut Self>
            ) $fn_body
        }
    };
}

#[doc(hidden)]
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
        unmount $types:tt $fn_data:tt
    ) => {
        $crate::__impl_trait_hook_unmount! {
            $types $fn_data
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

#[doc(hidden)]
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

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_phantoms {
    (
        $ty:ty,
        {$(
            $($lt:lifetime)?
            $($tp0:ident $($tp1:ident)?)?
        ),+}
    ) => {
        (
            $ty,
            $(
                $( $crate::__impl_phantom![$lt] )?
                $( $crate::__impl_phantom![$tp0 $($tp1)?] )?
            ),*
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_phantom {
    ($lt:lifetime) => {
        ::core::marker::PhantomData::<&$lt()>
    };
    (const $tp:ident) => {
        ()
    };
    ($tp:ident) => {
        ::core::marker::PhantomData::<$tp>
    };
}
