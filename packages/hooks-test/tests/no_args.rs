use hooks::{hook, HookExt, HookPollNextUpdateExt};

mod utils;

use utils::{assert_return_ty, hook_macro};

#[test]
fn no_return_ty_no_hooks() {
    hook_macro! {
        #[hook]
        fn use_tuple_0() {}
    }

    assert_return_ty! {
        use_tuple_0() =>
            impl ::hooks::core::Hook<()>
                + ::hooks::core::HookPollNextUpdate
                + for<'hook> ::hooks::core::HookLifetime<'hook, (), Value = ()>
                + ::hooks::core::HookBounds<Bounds = ()>
    };

    assert_eq!(
        use_tuple_0::hook_args(),
        hooks_derive_core::HookArgs::default()
    );

    let mut hook = use_tuple_0();

    assert_eq!(std::mem::size_of_val(&hook), 0);

    futures_lite::future::block_on::<()>(async {
        assert!(!hook.next_update().await);
        hook.use_hook(())
    });

    futures_lite::future::block_on::<()>(async {
        let val = hook.into_run_with_clone_args(()).next_value().await;
        assert!(val.is_none());
    });
}

#[test]
fn no_hooks() {
    hook_macro! {
        #[hook]
        fn use_return() -> i32 {
            1
        }
    }

    assert_return_ty! {
        use_return() =>
            impl ::hooks::core::Hook<()>
                + for<'hook> ::hooks::core::HookLifetime<'hook, (), Value = i32>
                + ::hooks::core::HookBounds<Bounds = ()>
    };

    assert_eq!(
        use_return::hook_args(),
        hooks_derive_core::HookArgs::default()
    );

    {
        let mut hook = use_return();
        assert_eq!(std::mem::size_of_val(&hook), 0);

        futures_lite::future::block_on(async {
            assert!(!hook.next_update().await);
            assert_eq!(hook.use_hook(()), 1);
        });

        futures_lite::future::block_on(async {
            let v = hook.into_run_with_default_args().next_value().await;
            assert!(v.is_none());
        });
    }

    hook_macro! {
        #[hook]
        fn use_type_param<T: Default>() -> T {
            T::default()
        }
    }

    assert_return_ty! {
        use_type_param::<String>() =>
            impl ::hooks::core::Hook<()>
                + for<'hook> ::hooks::core::HookLifetime<'hook, (), Value = String>
                + ::hooks::core::HookBounds<Bounds = (::core::marker::PhantomData<String>,)>
    };

    assert_eq!(
        use_type_param::hook_args(),
        hooks_derive_core::HookArgs::default()
    );

    {
        let mut hook = use_type_param::<String>();
        assert_eq!(std::mem::size_of_val(&hook), 0);

        futures_lite::future::block_on(async {
            assert!(!hook.next_update().await);
            assert_eq!(hook.use_hook(()), "");
        });

        futures_lite::future::block_on(async {
            let v = hook.into_run_with_default_args().next_value().await;
            assert!(v.is_none());
        });
    }

    hook_macro! {
        #[hook]
        fn use_lt<'a>() -> &'a i32 {
            static VALUE: i32 = 1;
            &VALUE
        }
    }

    assert_eq!(use_lt::hook_args(), hooks_derive_core::HookArgs::default());

    fn assert_use_lt<'a>() -> impl ::hooks::core::Hook<()>
           + for<'hook> ::hooks::core::HookLifetime<'hook, (), &'hook (&'a (),), Value = &'a i32>
           + ::hooks::core::HookBounds<Bounds = (&'a (),)> {
        use_lt::<'a>()
    }

    assert_use_lt();

    {
        let mut hook = use_lt();
        assert_eq!(std::mem::size_of_val(&hook), 0);

        futures_lite::future::block_on(async {
            assert!(!hook.next_update().await);
            assert_eq!(hook.use_hook(()), &1);
        });

        futures_lite::future::block_on(async {
            let v = hook.into_run_with_default_args().next_value().await;
            assert!(v.is_none());
        });
    }
}

#[test]
fn no_hooks_borrow_hook() {
    hook_macro! {
        #[hook]
        fn use_hook_lt() -> &'hook i32 {
            static VALUE: i32 = 2;
            &VALUE
        }
    }

    assert_return_ty! {
        use_hook_lt() => impl ::hooks::core::Hook<()>
            + for<'hook> ::hooks::core::HookLifetime<'hook, (), Value = &'hook i32>
            + ::hooks::core::HookBounds<Bounds = ()>
    };

    assert_eq!(
        use_hook_lt::hook_args(),
        hooks_derive_core::HookArgs::default()
    );

    {
        let mut hook = use_hook_lt();
        assert_eq!(std::mem::size_of_val(&hook), 0);

        futures_lite::future::block_on(async {
            assert!(!hook.next_update().await);
            assert_eq!(hook.use_hook(()), &2);
        });

        futures_lite::future::block_on(async {
            let mut running_hook = hook.into_run_with_default_args();
            let v = running_hook.next_value().await;
            assert!(v.is_none());
        });
    }
}
