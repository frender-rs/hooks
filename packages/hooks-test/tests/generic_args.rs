use hooks::{hook, HookExt, HookPollNextUpdateExt, IntoHook, UpdateHook};

mod utils;

use utils::{assert_return_ty, hook_macro};

#[test]
fn no_return_ty_no_hooks() {
    hook_macro!(
        #[hook]
        fn use_tuple_0<'a>(_: &'a ()) {}
    );

    assert_return_ty! {
        use_tuple_0(&()) => ::hooks::UpdateHookUninitialized![()]
    };

    assert_eq!(
        use_tuple_0::hook_args(),
        hooks_macro_core::HookArgs::default(),
    );

    let mut hook = use_tuple_0(&()).into_hook();

    assert_eq!(std::mem::size_of_val(&hook), std::mem::size_of::<bool>());

    futures_lite::future::block_on::<()>(async {
        assert!(hook.next_update().await);
        hook.use_hook();
    });

    futures_lite::future::block_on::<()>(async {
        let val = hook.next_value().await;
        assert!(val.is_none());
    });
}

#[test]
fn no_return_ty_no_hooks_elided() {
    hook_macro!(
        #[hook]
        fn use_tuple_0_elided(_: &()) {}
    );

    assert_return_ty! {
        use_tuple_0_elided(&()) => ::hooks::UpdateHookUninitialized![()]
    };

    assert_eq!(
        use_tuple_0_elided::hook_args(),
        hooks_macro_core::HookArgs::default(),
    );

    let mut hook = use_tuple_0_elided(&()).into_hook();

    assert_eq!(std::mem::size_of_val(&hook), std::mem::size_of::<bool>());

    futures_lite::future::block_on::<()>(async {
        assert!(hook.next_update().await);
        hook.use_hook();
    });

    futures_lite::future::block_on::<()>(async {
        let val = hook.next_value().await;
        assert!(val.is_none());
    });
}

#[test]
fn type_param() {
    hook_macro!(
        #[hook(bounds = "'a")]
        fn use_hooked<'a, T: ?Sized + ToOwned + PartialEq>(v: &'a T) -> &'hook T {
            use std::borrow::Borrow;

            let memo = hooks::use_mut_default::<Option<T::Owned>>();

            let memo = memo.get_or_insert_with(|| v.to_owned());
            (*memo).borrow()
        }
    );

    assert_return_ty! {
        use_hooked::<str>("") => ::hooks::UpdateHookUninitialized![&'hook str]
    };

    assert_eq!(use_hooked::hook_args(), {
        let mut args = hooks_macro_core::HookArgs::default();
        args.bounds = Some(hooks_macro_core::syn::parse_quote!('a));
        args
    });

    let mut hook = use_hooked::<str>("aa").into_hook();
    assert_eq!(
        std::mem::size_of_val(&hook),
        std::mem::size_of::<(
            // `FnHook.initialized`
            bool,
            // `FnHook.inner_hook` use_mut_default()
            utils::HookUninitialized<hooks::hook_mut::UseMutDefault<Option<String>>>,
            // `FnHook.use_hook` the argument is moved into this closure
            &str,
        )>()
    );

    futures_lite::future::block_on(async {
        assert!(hook.next_update().await);
        assert_eq!(hook.use_hook(), "aa");
        assert!(!hook.next_update().await);
    });

    futures_lite::future::block_on(async {
        use_hooked::<str>("a").update_hook(std::pin::Pin::new(&mut hook));
        let v = hook.next_value().await;
        assert!(v.is_none());
        assert_eq!(hook.use_hook(), "aa");
    });
}
