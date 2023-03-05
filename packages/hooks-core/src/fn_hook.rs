use std::{marker::PhantomData, pin::Pin};

use crate::{Hook, HookPollNextUpdate, HookUnmount, UpdateHook};

mod sealed {
    pub trait Initialized: Default {
        fn is_initialized(this: &Self) -> bool;
        fn mark_as_initialized(this: &mut Self);
    }

    impl Initialized for () {
        #[inline(always)]
        fn is_initialized(_: &Self) -> bool {
            true
        }

        #[inline(always)]
        fn mark_as_initialized(_: &mut Self) {}
    }

    impl Initialized for bool {
        #[inline(always)]
        fn is_initialized(this: &Self) -> bool {
            *this
        }

        #[inline(always)]
        fn mark_as_initialized(this: &mut Self) {
            *this = true;
        }
    }
}

pin_project_lite::pin_project![
    #[derive(Default)]
    pub struct FnHook<InnerHook: Default, U, I: sealed::Initialized> {
        #[pin]
        pub inner_hook: InnerHook,
        pub use_hook: U,
        pub initialized: I,
    }
];

crate::impl_hook![
    type For<InnerHook, U, I: sealed::Initialized> = FnHook<InnerHook, U, I>
        where __![
            InnerHook: Default + HookPollNextUpdate + HookUnmount,
            U: for<'hook> FnMutOneArg<Pin<&'hook mut InnerHook>>,
        ]: __;
    fn poll_next_update(self, cx: _) {
        let mut this = self.project();
        if !I::is_initialized(this.initialized) {
            I::mark_as_initialized(this.initialized);
            this.use_hook
                .call_mut_with_one_arg(this.inner_hook.as_mut());
        }
        this.inner_hook.poll_next_update(cx)
    }
    fn unmount(self) {
        let this = self.project();
        if I::is_initialized(this.initialized) {
            this.inner_hook.unmount()
        }
    }
    #[inline]
    fn use_hook(self) -> <U as FnMutOneArg<std::pin::Pin<&'hook mut InnerHook>>>::FnOutput {
        let this = self.project();
        I::mark_as_initialized(this.initialized);
        this.use_hook.call_mut_with_one_arg(this.inner_hook)
    }
];

pin_project_lite::pin_project![
    pub struct FnHookUninitialized<InnerHook: Default, U> {
        #[pin]
        inner_hook: InnerHook,
        use_hook: Option<U>,
    }
];

impl<InnerHook: Default, U> Default for FnHookUninitialized<InnerHook, U> {
    fn default() -> Self {
        Self {
            inner_hook: Default::default(),
            use_hook: None,
        }
    }
}

crate::impl_hook![
    type For<InnerHook, U> = FnHookUninitialized<InnerHook, U>
        where __![
            InnerHook: Default + HookPollNextUpdate + HookUnmount,
            U: for<'hook> FnMutOneArg<Pin<&'hook mut InnerHook>>,
        ]: __;
    fn poll_next_update(self, cx: _) {
        let this = self.project();
        if this.use_hook.is_some() {
            this.inner_hook.poll_next_update(cx)
        } else {
            std::task::Poll::Ready(true)
        }
    }
    fn unmount(self) {
        let this = self.project();
        if this.use_hook.is_some() {
            this.inner_hook.unmount()
        }
    }
];

pub mod use_fn_hook {
    use super::*;
    pub mod pin {
        use super::super::*;
        pub struct UseFnHook<
            InnerHook: Default + HookPollNextUpdate + HookUnmount,
            U: for<'hook> FnMutOneArg<Pin<&'hook mut InnerHook>>,
        >(pub U, pub PhantomData<InnerHook>);

        crate::impl_hook![
            type For<InnerHook, U> = UseFnHook<InnerHook, U>
                where __![
                    InnerHook: Default + HookPollNextUpdate + HookUnmount,
                    U: for<'hook> FnMutOneArg<Pin<&'hook mut InnerHook>>,
                ]: __;

            fn into_hook(self) -> FnHook<InnerHook, U, bool> {
                FnHook {
                    inner_hook: Default::default(),
                    use_hook: self.0,
                    initialized: false,
                }
            }

            fn update_hook(self, hook: _) {
                let hook = hook.project();
                if !*hook.initialized {
                    *hook.initialized = true;
                    // value is dropped
                    let _ = hook.use_hook.call_mut_with_one_arg(hook.inner_hook);
                }
                *hook.use_hook = self.0;
            }

            #[inline]
            fn h(self, hook: FnHookUninitialized<InnerHook, U>) {
                let hook = hook.project();
                let use_hook = hook.use_hook.insert(self.0);
                use_hook.call_mut_with_one_arg(hook.inner_hook)
            }
        ];
    }

    pub fn pin<
        InnerHook: Default + HookPollNextUpdate + HookUnmount,
        U: for<'hook> FnMutOneArg<Pin<&'hook mut InnerHook>>,
    >(
        use_hook: U,
    ) -> pin::UseFnHook<InnerHook, U> {
        pin::UseFnHook(use_hook, PhantomData)
    }

    pub mod unpin {
        use super::super::*;
        pub struct UseFnHook<
            InnerHook: Default + HookPollNextUpdate + HookUnmount + Unpin,
            U: for<'hook> FnMutOneArg<Pin<&'hook mut InnerHook>>,
        >(pub U, pub PhantomData<InnerHook>);

        impl<
                InnerHook: Default + HookPollNextUpdate + HookUnmount + Unpin,
                U: for<'hook> FnMutOneArg<Pin<&'hook mut InnerHook>>,
            > UseFnHook<InnerHook, U>
        {
            pub fn new(use_hook: U) -> Self {
                Self(use_hook, PhantomData)
            }
        }

        crate::impl_hook![
            type For<InnerHook, U> = UseFnHook<InnerHook, U>
                where __![
                    InnerHook: Default + HookPollNextUpdate + HookUnmount + Unpin,
                    U: for<'hook> FnMutOneArg<Pin<&'hook mut InnerHook>>,
                ]: __;

            fn into_hook(mut self) -> FnHook<InnerHook, U, ()> {
                let mut inner_hook = Default::default();
                self.0.call_mut_with_one_arg(Pin::new(&mut inner_hook));
                FnHook {
                    inner_hook,
                    use_hook: self.0,
                    initialized: (),
                }
            }

            #[inline]
            fn update_hook(self, hook: _) {
                hook.get_mut().use_hook = self.0;
            }

            #[inline]
            fn h(self, hook: FnHookUninitialized<InnerHook, U>) {
                let hook = hook.project();
                let use_hook = hook.use_hook.insert(self.0);
                use_hook.call_mut_with_one_arg(hook.inner_hook)
            }
        ];
    }

    pub fn unpin<
        InnerHook: Default + HookPollNextUpdate + HookUnmount + Unpin,
        U: for<'hook> FnMutOneArg<Pin<&'hook mut InnerHook>>,
    >(
        use_hook: U,
    ) -> unpin::UseFnHook<InnerHook, U> {
        unpin::UseFnHook(use_hook, PhantomData)
    }

    pub(super) mod prelude_name {
        pub use super::unpin as use_fn_hook;
    }
}

pub use use_fn_hook::prelude_name::*;

pub trait FnMutOneArg<Arg> {
    type FnOutput;
    fn call_mut_with_one_arg(&mut self, arg: Arg) -> Self::FnOutput;
}

impl<F, Arg, R> FnMutOneArg<Arg> for F
where
    F: FnMut(Arg) -> R,
{
    type FnOutput = R;

    #[inline(always)]
    fn call_mut_with_one_arg(&mut self, arg: Arg) -> Self::FnOutput {
        self(arg)
    }
}
