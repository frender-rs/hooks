use std::{any::Any, fmt::Display};

use futures_lite::StreamExt;
use hooks::{hook, HookExt, HookPollNextUpdateExt, IntoHook};

mod utils;

use utils::{assert_return_ty, hook_macro};

#[test]
fn no_hooks() {
    hook_macro! {
        #[hook]
        fn use_return() -> impl Display {
            1
        }
    }

    assert_return_ty! {
        use_return() => ::hooks::core::UpdateHookUninitialized![impl Display]
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
            assert_eq!(hook.use_hook().to_string(), "1");
        });

        futures_lite::future::block_on(async {
            let v = hook.next_value().await;
            assert!(v.is_none());
        });
    }

    hook_macro! {
        #[hook]
        fn use_type_param<T: Default + Display>() -> impl Display {
            T::default()
        }
    }

    assert_return_ty! {
        use_type_param::<String>() => ::hooks::UpdateHookUninitialized![impl Display]
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
            assert_eq!(hook.use_hook().to_string(), "");
        });

        futures_lite::future::block_on(async {
            let v = hook.next_value().await;
            assert!(v.is_none());
        });
    }

    hook_macro!(
        #[hook(bounds = "'a")]
        fn use_lt<'a>(v: &'a impl Display) -> &'a (impl Display + 'a) {
            v
        }
    );

    assert_eq!(use_lt::hook_args(), {
        let mut args = hooks_macro_core::HookArgs::default();

        args.bounds = Some(hooks_macro_core::syn::parse_quote!('a));

        args
    });

    assert_return_ty! {
        use_lt(&1) => ::hooks::UpdateHookUninitialized![&'static (impl Display + 'static)]
    };

    {
        let mut hook = use_lt(&1).into_hook();
        assert_eq!(
            std::mem::size_of_val(&hook),
            std::mem::size_of_val(&(false, &1)),
        );

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook().to_string(), "1");
        });

        futures_lite::future::block_on(async {
            let v = hook.next_value().await;
            assert!(v.is_none());
        });
    }
}

/// TODO: hook fn returning (impl Trait + 'hook) is not working
///
/// - `higher kinded lifetime bounds on nested opaque types are not supported yet`
/// - expand `__HooksValueOfThisHook<HooksImplTrait0>`
///   instead of `struct __HooksValueOfThisHook<HooksImplTrait0: Trait + 'hook>`
#[cfg(hook_macro_fail)]
#[test]
fn no_hooks_borrow_hook() {
    hook_macro! {
        #[hook]
        fn use_hook_lt() -> &'hook (impl Display + 'hook) {
            static VALUE: i32 = 2;
            &VALUE
        }
    }

    assert_return_ty! {
        use_hook_lt() => impl ::hooks::core::Hook<()>
            + for<'hook> ::hooks::core::HookLifetime<'hook, (), Value = &'hook impl Display>
            + ::hooks::core::HookBounds<Bounds = ()>
    };

    assert_eq!(
        use_hook_lt::hook_args(),
        hooks_macro_core::HookArgs::default()
    );

    {
        let mut hook = use_hook_lt();
        assert_eq!(std::mem::size_of_val(&hook), std::mem::size_of::<bool>());

        futures_lite::future::block_on(async {
            assert!(!hook.next_update().await);
            assert_eq!(hook.use_hook(()), &2);
        });

        futures_lite::future::block_on(async {
            let v = hook.next_value(()).await;
            assert!(v.is_none());
        });
    }
}

#[test]
fn one_hook() {
    hook_macro!(
        #[hook]
        fn use_one_hook() -> &'hook mut (impl Display + Default) {
            let v = ::hooks::use_lazy_pinned(-3);
            v.get_mut()
        }
    );

    assert_return_ty! {
        use_one_hook() => ::hooks::UpdateHookUninitialized![&'hook mut (impl Display + Default)]
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
            assert_eq!(hook.use_hook().to_string(), "-3");
            assert!(!hook.next_update().await);
            *hook.use_hook() = Default::default();
            assert!(!hook.next_update().await);
            assert_eq!(hook.use_hook().to_string(), "0");
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
        fn use_str_state() -> impl Any {
            let (state, updater) = ::hooks::use_state_with(String::new);

            updater.replace_maybe_with_fn_pointer(|old| {
                if old.len() < 2 {
                    Some(format!("{old} "))
                } else {
                    None
                }
            });

            state.clone()
        }
    }

    assert_return_ty! {
        use_str_state() => ::hooks::UpdateHookUninitialized![impl Any]
    };

    assert_eq!(
        use_str_state::hook_args(),
        hooks_macro_core::HookArgs::default()
    );

    fn assert_string<T: Any>(t: &T) -> &String {
        <dyn Any>::downcast_ref::<String>(t).unwrap()
    }

    {
        let mut hook = use_str_state().into_hook();
        assert_eq!(
            std::mem::size_of_val(&hook),
            std::mem::size_of::<(
                bool,
                utils::HookUninitialized<
                    ::hooks::state::UseState<
                        String,
                        { ::hooks::state::STAGING_STATES_DEFAULT_STACK_COUNT },
                        false,
                    >,
                >
            )>()
        );

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(assert_string(&hook.use_hook()), "");
            assert!(hook.next_update().await);
            assert_eq!(assert_string(&hook.use_hook()), " ");
            assert!(hook.next_update().await);
            assert_eq!(assert_string(&hook.use_hook()), "  ");
            assert!(!hook.next_update().await);
        });
    }

    futures_lite::future::block_on(async {
        let values = use_str_state().into_hook_values().collect::<Vec<_>>().await;

        let values = values
            .iter()
            .map(assert_string)
            .map(String::as_str)
            .collect::<Vec<_>>();

        assert_eq!(values, ["", " ", "  "])
    });
}

#[test]
fn two_hooks() {
    hook_macro! {
        #[hook]
        fn use_state_effect() -> impl Display {
            let (state, updater) = ::hooks::use_state_with::<i32>(Default::default);
            let updater = updater.clone();

            ::hooks::use_effect(move |v: &_| {
                if *v < 2 {
                    updater.set(*v + 1)
                }
            }, *state);

            *state
        }
    }

    assert_return_ty! {
        use_state_effect() => ::hooks::UpdateHookUninitialized![impl Display]
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
                false,
                utils::hook_uninitialized_default(hooks::use_state(0)),
                utils::hook_uninitialized_default(hooks::use_effect(
                    {
                        let updater = hooks::state::StateUpdater::<i32>::new();
                        move |_: &i32| drop(updater)
                    },
                    0,
                )),
            )),
        );

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook().to_string(), "0");
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook().to_string(), "1");
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook().to_string(), "2");
            assert!(!hook.next_update().await);
        });
    }

    futures_lite::future::block_on(async {
        let values = use_state_effect()
            .into_hook_values()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .await;

        assert_eq!(values, ["0", "1", "2"]);
    });
}
