use hooks::{core::AsyncIterableHook, hook, HookExt, HookPollNextUpdateExt};

mod utils;

use utils::{assert_return_ty, hook_macro};

mod ttt {
    use std::pin::Pin;

    #[allow(non_camel_case_types)]
    struct use_type_param {}
    fn use_type_param<T: Default>(
    ) -> impl ::hooks::core::Hook<()> + for<'hook> ::hooks::core::Hook<(), Value<'hook> = T> {
        ::hooks::core::__private::pin_project! {
            struct MyFnHook<Data, F, V> {
                #[pin]
                data: Data,
                use_hook: F,
                _phantom: ::core::marker::PhantomData<V>
            }
        }

        impl<Data, F, V> ::hooks::core::HookPollNextUpdate for MyFnHook<Data, F, V> {
            #[inline]
            fn poll_next_update(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> ::std::task::Poll<bool> {
                ::std::task::Poll::Ready(false)
            }
        }

        impl<T, Data, __HooksUseHook> ::hooks::core::Hook<()>
            for MyFnHook<Data, __HooksUseHook, (::core::marker::PhantomData<T>,)>
        where
            __HooksUseHook: FnMut(Pin<&mut Data>, ()) -> T,
        {
            type Value<'hook> = T
            where
                Self: 'hook;

            #[inline]
            fn use_hook<'hook>(self: std::pin::Pin<&'hook mut Self>, args: ()) -> Self::Value<'hook>
            where
                Self: 'hook,
            {
                let this = self.project();
                (this.use_hook)(this.data, args)
            }
        }

        MyFnHook {
            data: (),
            use_hook: |_: ::core::pin::Pin<&mut ()>, (): ()| T::default(),
            _phantom: ::core::marker::PhantomData,
        }
    }
}

#[test]
fn no_return_ty_no_hooks() {
    hook_macro! {
        #[hook]
        fn use_tuple_0() {}
    }

    assert_return_ty! {
        use_tuple_0() =>
            impl ::hooks::core::Hook<()>
                + for<'hook> ::hooks::core::Hook<(), Value<'hook> = ()>
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
        let val = hook.next_value(()).await;
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
                + for<'hook> ::hooks::core::Hook<(), Value<'hook> = i32>
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
            let v = hook.next_value(()).await;
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
                + for<'hook> ::hooks::core::Hook<(), Value<'hook> = String>
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
            let v = hook.next_value(()).await;
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

    fn assert_use_lt<'a>(
    ) -> impl ::hooks::core::Hook<()> + for<'hook> ::hooks::core::Hook<(), Value<'hook> = &'a i32>
    {
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
            let v = hook.next_value(()).await;
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
            + for<'hook> ::hooks::core::Hook<(), Value<'hook> = &'hook i32>

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
            let v = hook.next_value(()).await;
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
        use_one_hook() => impl ::hooks::core::Hook<()>
            + for<'hook> ::hooks::core::Hook<(), Value<'hook> = &'hook mut i32>

    };

    assert_eq!(
        use_one_hook::hook_args(),
        hooks_derive_core::HookArgs::default()
    );

    {
        let mut hook = use_one_hook();
        assert_eq!(
            std::mem::size_of_val(&hook),
            std::mem::size_of::<::hooks::LazyPinned<i32>>()
        );

        futures_lite::future::block_on(async {
            assert!(!hook.next_update().await);
            assert_eq!(*hook.use_hook(()), 0);
            assert!(!hook.next_update().await);
            *hook.use_hook(()) = -3;
            assert!(!hook.next_update().await);
            assert_eq!(*hook.use_hook(()), -3);
        });

        futures_lite::future::block_on(async {
            let v = hook.next_value(()).await;
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
                    Some(format!("{} ", old))
                } else {
                    None
                }
            });

            state
        }
    }

    assert_return_ty! {
        use_str_state() => impl ::hooks::core::Hook<()>
            + for<'hook> ::hooks::core::Hook<(), Value<'hook> = &'hook str>

    };

    assert_eq!(
        use_str_state::hook_args(),
        hooks_derive_core::HookArgs::default()
    );

    {
        let mut hook = use_str_state();
        assert_eq!(
            std::mem::size_of_val(&hook),
            std::mem::size_of::<::hooks::State<String>>()
        );

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(()), "");
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(()), " ");
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(()), "  ");
            assert!(!hook.next_update().await);
        });
    }

    futures_lite::future::block_on(async {
        let mut running_hook = use_str_state().into_iter();
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
        use_state_effect() => impl ::hooks::core::Hook<()>
            + for<'hook> ::hooks::core::Hook<(), Value<'hook> = &'hook i32>

    };

    assert_eq!(
        use_state_effect::hook_args(),
        hooks_derive_core::HookArgs::default()
    );

    {
        let mut hook = use_state_effect();

        let dummy_effect = {
            let updater = hooks::StateUpdater::<i32>::new();
            let effect = move |_: &i32| drop(updater);
            let mut hook = hooks::use_effect();
            hook.use_hook((effect, 0));
            hook
        };

        assert_eq!(
            std::mem::size_of_val(&hook),
            std::mem::size_of::<::hooks::State<i32>>() + std::mem::size_of_val(&dummy_effect),
        );

        futures_lite::future::block_on(async {
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(()), &0);
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(()), &1);
            assert!(hook.next_update().await);
            assert_eq!(hook.use_hook(()), &2);
            assert!(!hook.next_update().await);
        });
    }

    futures_lite::future::block_on(async {
        let mut hook = use_state_effect().into_iter();
        assert_eq!(hook.next_value().await, Some(&0));
        assert_eq!(hook.next_value().await, Some(&1));
        assert_eq!(hook.next_value().await, Some(&2));
        assert_eq!(hook.next_value().await, None);
    });
}
