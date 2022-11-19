pub mod debug_pointer;

mod rc_status;

pub use rc_status::*;

macro_rules! __impl_unexpected_token {
    () => {};
}

#[allow(unused_imports)]
pub(crate) use __impl_unexpected_token;

macro_rules! __impl_poll_next_update {
    (@ [$(#[$meta_poll:meta])*] $poll_next_update:ident [$this_0:ident $($this_1:ident)?] [$cx_pat:pat] $impl_poll:block) => {
        $(#[$meta_poll])*
        fn $poll_next_update(
            $this_0 $($this_1)? : ::core::pin::Pin<&mut Self>,
            $cx_pat: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> $impl_poll
    };
    ($(#[$meta_poll:meta])* $poll_next_update:ident($this_0:ident $($this_1:ident)? $(,)?             ) $impl_poll:block) => {
        $crate::utils::__impl_poll_next_update! {@ [$(#[$meta_poll])*] $poll_next_update [$this_0 $($this_1)?] [ _     ] $impl_poll}
    };
    ($(#[$meta_poll:meta])* $poll_next_update:ident($this_0:ident $($this_1:ident)?, $cx_pat:pat $(,)?) $impl_poll:block) => {
        $crate::utils::__impl_poll_next_update! {@ [$(#[$meta_poll])*] $poll_next_update [$this_0 $($this_1)?] [$cx_pat] $impl_poll}
    };
    ($(#[$meta_poll:meta])* $poll_next_update:ident($($pat_args_poll:tt)*) $impl_poll:block) => {
        $(#[$meta_poll])*
        fn $poll_next_update(
            self: ::core::pin::Pin<&mut Self>,
            _: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> $impl_poll
        $crate::utils::__impl_unexpected_token!($($pat_args_poll)*);
        compile_error!("Expect arguments for poll_next_update");
    };
    ($($tt:tt)*) => {
        fn poll_next_update(
            self: ::core::pin::Pin<&mut Self>,
            _: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> {
            ::core::todo!()
        }
        $crate::utils::__impl_unexpected_token!($($tt)*);
        compile_error!("Expect `poll_next_update(self, cx) {}`");
    };
}

pub(crate) use __impl_poll_next_update;

macro_rules! impl_hook {
    (
        impl $([ $($impl_generics:tt)* ])? for $ty:ty $(where [$($where:tt)*])? {
            $(#[$meta_poll:meta])*
            $poll_next_update:ident($($pat_args_poll:tt)*) $impl_poll:block

            $(#[$meta_use:meta])*
            $use_hook:ident $([$($args_generics:tt)*])? ( $self_0:ident $($self_1:ident)? $(, $arg_pat:ident : $arg_ty:ty)* $(,)? ) -> $value:ty $impl_use_hook:block
        }
    ) => {
        impl<$($($impl_generics)*)?> ::hooks_core::HookPollNextUpdate for $ty $(where $($where)*)? {
            $crate::utils::__impl_poll_next_update! {
                $(#[$meta_poll])*
                $poll_next_update($($pat_args_poll)*) $impl_poll
            }
        }

        impl<
            $($($impl_generics)*)?
            $(, $($args_generics)*)?
        > ::hooks_core::Hook<($($arg_ty ,)*)> for $ty $(where $($where)*)? {
            type Value<'hook> = $value where Self: 'hook;

            $(#[$meta_use])*
            fn $use_hook<'hook>(
                $self_0 $($self_1)?: ::core::pin::Pin<&'hook mut Self>,
                ($($arg_pat ,)*): ($($arg_ty ,)*),
            ) -> Self::Value<'hook>
            where
                Self: 'hook
            $impl_use_hook
        }
    };
}
pub(crate) use impl_hook;
