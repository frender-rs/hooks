#[macro_export]
macro_rules! __impl_unexpected_token {
    () => {};
}

#[macro_export]
macro_rules! __impl_poll_next_update {
    (@ [$(#[$meta_poll:meta])*] $poll_next_update:ident [$this_0:ident $($this_1:ident)?] [$cx_pat:pat] $impl_poll:block) => {
        $(#[$meta_poll])*
        fn $poll_next_update(
            $this_0 $($this_1)? : ::core::pin::Pin<&mut Self>,
            $cx_pat: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> $impl_poll
    };
    ($(#[$meta_poll:meta])* $poll_next_update:ident($this_0:ident $($this_1:ident)? $(,)?             ) $impl_poll:block) => {
        $crate::__impl_poll_next_update! {@ [$(#[$meta_poll])*] $poll_next_update [$this_0 $($this_1)?] [ _     ] $impl_poll}
    };
    ($(#[$meta_poll:meta])* $poll_next_update:ident($this_0:ident $($this_1:ident)?, $cx_pat:pat $(,)?) $impl_poll:block) => {
        $crate::__impl_poll_next_update! {@ [$(#[$meta_poll])*] $poll_next_update [$this_0 $($this_1)?] [$cx_pat] $impl_poll}
    };
    ($(#[$meta_poll:meta])* $poll_next_update:ident($($pat_args_poll:tt)*) $impl_poll:block) => {
        $(#[$meta_poll])*
        fn $poll_next_update(
            self: ::core::pin::Pin<&mut Self>,
            _: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> $impl_poll
        $crate::__impl_unexpected_token!($($pat_args_poll)*);
        compile_error!("Expect arguments for poll_next_update");
    };
    ($($tt:tt)*) => {
        fn poll_next_update(
            self: ::core::pin::Pin<&mut Self>,
            _: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> {
            ::core::todo!()
        }
        $crate::__impl_unexpected_token!($($tt)*);
        compile_error!("Expect `poll_next_update(self, cx) {}`");
    };
}

#[macro_export]
macro_rules! __impl_use_hook {
    // actual implementation
    (
        @
        [ $($impl_generics:tt)* ]
        [ $ty:ty ]
        [ $($where:tt)* ]
        {
            $(#[$meta_use:meta])*
            $use_hook:ident
            $({$($args_lifetimes:tt)*})?
            $([$($args_generics:tt)*])?
            ( $self_0:ident $($self_1:ident)? $(, $arg_pat:ident : $arg_ty:ty)* $(,)? ) -> $value:ty $impl_use_hook:block
        }
    ) =>{
        impl<
            'hook,
            $($($args_lifetimes)* ,)?
            $($impl_generics)*
            $($($args_generics)*)?
        > $crate::HookLifetime<'hook, ($($arg_ty ,)*), &'hook Self> for $ty $($where)* {
            type Value = $value;
        }

        impl<
            $($($args_lifetimes)* ,)?
            $($impl_generics)*
            $($($args_generics)*)?
        > $crate::Hook<($($arg_ty ,)*)> for $ty $($where)* {
            $(#[$meta_use])*
            fn $use_hook<'hook>(
                $self_0 $($self_1)?: ::core::pin::Pin<&'hook mut Self>,
                ($($arg_pat ,)*): ($($arg_ty ,)*),
            ) -> <Self as $crate::HookLifetime<'hook, ($($arg_ty ,)*)>>::Value
            where
                Self: 'hook
            $impl_use_hook
        }
    };
    // do nothing if there is no `use_hook(self) -> Value {}`
    (
        *
        $impl_generics:tt
        $ty:tt
        $where:tt
    ) => {};
    // expand multiple use_hook
    (
        *
        $impl_generics:tt
        $ty:tt
        $where:tt
        $($use_hook_tt:tt)+
    ) => {
        $(
            $crate::__impl_use_hook! {
                @
                $impl_generics
                $ty
                $where
                $use_hook_tt
            }
        )+

    };
}

#[macro_export]
macro_rules! impl_hook {
    (
        impl $([ $($impl_generics:tt)* ])? for $ty:ty $(where [$($where:tt)*])? {
            $(#[$meta_poll:meta])*
            $poll_next_update:ident($($pat_args_poll:tt)*) $impl_poll:block

            $(

            $(#[$meta_use:meta])*
            $use_hook:ident
            $({$($args_lifetimes:tt)*})?
            $([$($args_generics:tt)*])?
            ( $self_0:ident $($self_1:ident)? $(, $arg_pat:ident : $arg_ty:ty)* $(,)? ) -> $value:ty $impl_use_hook:block

            )*
        }
    ) => {
        impl<$($($impl_generics)*)?> $crate::HookBounds for $ty $(where $($where)*)? {
            type Bounds = Self;
        }

        impl<$($($impl_generics)*)?> $crate::HookPollNextUpdate for $ty $(where $($where)*)? {
            $crate::__impl_poll_next_update! {
                $(#[$meta_poll])*
                $poll_next_update($($pat_args_poll)*) $impl_poll
            }
        }

        $crate::__impl_use_hook! {
            *
            [$($($impl_generics)* , )?]
            [$ty]
            [$(where $($where)*)?]
            $({
                $(#[$meta_use])*
                $use_hook
                $({$($args_lifetimes)*})?
                $([$($args_generics )*])?
                ( $self_0 $($self_1)? $(, $arg_pat : $arg_ty)* ) -> $value $impl_use_hook
            })*
        }
    };
}
