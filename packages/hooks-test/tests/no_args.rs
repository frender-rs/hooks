use hooks::{hook, HookExt, HookPollNextUpdateExt, IntoHook};

mod utils;

use utils::{assert_return_ty, hook_macro};

#[test]
fn no_return_ty_no_hooks() {
    hook_macro! {
        #[hook]
        fn use_tuple_0() {}
    }

    assert_return_ty! {
        use_tuple_0() => ::hooks::UpdateHookUninitialized![()]
    };

    assert_eq!(
        use_tuple_0::hook_args(),
        hooks_macro_core::HookArgs::default()
    );

    let mut hook = use_tuple_0().into_hook();

    assert_eq!(std::mem::size_of_val(&hook), std::mem::size_of::<bool>());

    futures_lite::future::block_on::<()>(async {
        assert!(hook.next_update().await);
        hook.use_hook()
    });

    futures_lite::future::block_on::<()>(async {
        let val = hook.next_value().await;
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
        use_return() => ::hooks::UpdateHookUninitialized![i32]
    };

    assert_eq!(
        use_return::hook_args(),
        hooks_macro_core::HookArgs::default()
    );

    {
        let mut hook = use_return().into_hook();
        assert_eq!(std::mem::size_of_val(&hook), std::mem::size_of::<bool>());

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(), 1);
        });

        futures_lite::future::block_on(async {
            let v = hook.next_value().await;
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
        use_type_param::<String>() => ::hooks::UpdateHookUninitialized![String]
    };

    assert_eq!(
        use_type_param::hook_args(),
        hooks_macro_core::HookArgs::default()
    );

    {
        let mut hook = use_type_param::<String>().into_hook();
        assert_eq!(std::mem::size_of_val(&hook), std::mem::size_of::<bool>());

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(), "");
        });

        futures_lite::future::block_on(async {
            let v = hook.next_value().await;
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
        use_hook_lt() => ::hooks::UpdateHookUninitialized![&'hook i32]
    };

    assert_eq!(
        use_hook_lt::hook_args(),
        hooks_macro_core::HookArgs::default()
    );

    {
        let mut hook = use_hook_lt().into_hook();
        assert_eq!(std::mem::size_of_val(&hook), std::mem::size_of::<bool>());

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(), &2);
        });

        futures_lite::future::block_on(async {
            let v = hook.next_value().await;
            assert!(v.is_none());
        });
    }
}

#[test]
fn one_hook() {
    hook_macro! {
        #[hook]
        fn use_one_hook() -> &'hook mut i32 {
            let v = ::hooks::use_lazy_pinned(0);
            v.get_mut()
        }
    }

    assert_return_ty! {
        use_one_hook() => ::hooks::core::UpdateHookUninitialized![&'hook mut i32]
    };

    assert_eq!(
        use_one_hook::hook_args(),
        hooks_macro_core::HookArgs::default()
    );

    {
        let mut hook = use_one_hook().into_hook();
        assert_eq!(
            std::mem::size_of_val(&hook),
            std::mem::size_of::<(
                bool,
                utils::HookUninitialized<::hooks::lazy_pinned::UseLazyPinned<i32>>
            )>()
        );

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(*hook.use_hook(), 0);
            assert!(!hook.next_update().await);
            *hook.use_hook() = -3;
            assert!(!hook.next_update().await);
            assert_eq!(*hook.use_hook(), -3);
        });

        futures_lite::future::block_on(async {
            let v = hook.next_value().await;
            assert!(v.is_none());
        });
    }
}

#[test]
fn one_state() {
    hook_macro! {
        #[hook]
        fn use_str_state() -> &'hook str {
            let (state, updater) = ::hooks::use_state_with(String::new);

            updater.replace_maybe_with_fn_pointer(|old| {
                if old.len() < 2 {
                    Some(format!("{old} "))
                } else {
                    None
                }
            });

            state
        }
    }

    assert_return_ty! {
        use_str_state() => ::hooks::core::UpdateHookUninitialized![&'hook str]
    };

    assert_eq!(
        use_str_state::hook_args(),
        hooks_macro_core::HookArgs::default()
    );

    {
        let mut hook = use_str_state().into_hook();
        assert_eq!(
            std::mem::size_of_val(&hook),
            std::mem::size_of_val(&(
                false,
                utils::hook_uninitialized_default(hooks::use_state(String::new())),
            ))
        );

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(), "");
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(), " ");
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(), "  ");
            assert!(!hook.next_update().await);
        });
    }

    futures_lite::future::block_on(async {
        let mut running_hook = use_str_state().into_hook();
        assert_eq!(running_hook.next_value().await, Some(""));
        assert_eq!(running_hook.next_value().await, Some(" "));
        assert_eq!(running_hook.next_value().await, Some("  "));
        assert_eq!(running_hook.next_value().await, None);
    });
}

#[test]
fn two_hooks() {
    hook_macro! {
        #[hook]
        fn use_state_effect() -> &'hook i32 {
            let (state, updater) = ::hooks::use_state_with(Default::default);
            let updater = updater.clone();

            ::hooks::use_effect(move |v: &_| {
                if *v < 2 {
                    updater.set(*v + 1)
                }
            }, *state);

            state
        }
    }

    assert_return_ty! {
        use_state_effect() => ::hooks::UpdateHookUninitialized![&'hook i32]
    };

    assert_eq!(
        use_state_effect::hook_args(),
        hooks_macro_core::HookArgs::default()
    );

    {
        let mut hook = use_state_effect().into_hook();

        assert_eq!(
            std::mem::size_of_val(&hook),
            std::mem::size_of_val(&(
                //
                false,
                utils::hook_uninitialized_default(hooks::use_state_with(i32::default),),
                utils::hook_uninitialized_default({
                    let updater = hooks::state::StateUpdater::<i32>::new();
                    let effect = move |_: &i32| drop(updater);
                    hooks::use_effect(effect, 0)
                }),
            )),
        );

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(), &0);
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(), &1);
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(), &2);
            assert!(!hook.next_update().await);
        });
    }

    futures_lite::future::block_on(async {
        let mut hook = use_state_effect().into_hook();
        assert_eq!(hook.next_value().await, Some(&0));
        assert_eq!(hook.next_value().await, Some(&1));
        assert_eq!(hook.next_value().await, Some(&2));
        assert_eq!(hook.next_value().await, None);
    });
}
