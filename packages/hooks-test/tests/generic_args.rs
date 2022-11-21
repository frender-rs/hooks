use hooks::{hook, HookExt, HookPollNextUpdateExt};

mod utils;

use utils::{assert_return_ty, hook_macro};

#[test]
fn no_return_ty_no_hooks() {
    hook_macro! {
        #[hook(args_generics = "'a")]
        fn use_tuple_0(_: &'a ()) {}
    }

    assert_return_ty! {
        use_tuple_0() =>
            impl for <'a> ::hooks::core::Hook<(&'a (),)>
                + ::hooks::core::HookPollNextUpdate
                + for<'hook, 'a> ::hooks::core::HookLifetime<'hook, (&'a (),), Value = ()>
                + ::hooks::core::HookBounds<Bounds = ()>
    };

    assert_eq!(
        use_tuple_0::hook_args(),
        hooks_derive_core::HookArgs::default()
            .with_args_generics(hooks_derive_core::syn::parse_quote!( 'a ))
    );

    let mut hook = use_tuple_0();

    assert_eq!(std::mem::size_of_val(&hook), 0);

    futures_lite::future::block_on::<()>(async {
        assert!(!hook.next_update().await);
        hook.use_hook((&(),));
    });

    futures_lite::future::block_on::<()>(async {
        let val = hook.next_value((&(),)).await;
        assert!(val.is_none());
    });
}

#[test]
fn no_return_ty_no_hooks_elided() {
    hook_macro! {
        #[hook]
        fn use_tuple_0_elided(_: &()) {}
    }

    assert_return_ty! {
        use_tuple_0_elided() =>
            impl for <'a> ::hooks::core::Hook<(&'a (),)>
                + ::hooks::core::HookPollNextUpdate
                + for<'hook, 'a> ::hooks::core::HookLifetime<'hook, (&'a (),), Value = ()>
                + ::hooks::core::HookBounds<Bounds = ()>
    };

    assert_eq!(
        use_tuple_0_elided::hook_args(),
        hooks_derive_core::HookArgs::default(),
    );

    let mut hook = use_tuple_0_elided();

    assert_eq!(std::mem::size_of_val(&hook), 0);

    futures_lite::future::block_on::<()>(async {
        assert!(!hook.next_update().await);
        hook.use_hook((&(),));
    });

    futures_lite::future::block_on::<()>(async {
        let val = hook.next_value((&(),)).await;
        assert!(val.is_none());
    });
}

#[test]
fn type_param() {
    hook_macro! {
        #[hook]
        fn use_hooked<T: ?Sized + ToOwned + PartialEq>(v: &T) -> &'hook T {
            use std::borrow::Borrow;

            let (_, v) = hooks::use_memo::<(), T::Owned>(hooks::memo_with(|old| match old {
                Some(old) => old,
                old => old.insert(hooks::DataAndDep {
                    data: (),
                    dep: v.to_owned(),
                }),
            }));

            v.borrow()
        }
    }

    assert_return_ty! {
        use_hooked::<str>() =>
            impl for<'a> ::hooks::core::Hook<(&'a str,)>
                + for<'hook, 'a> ::hooks::core::HookLifetime<'hook, (&'a str,), Value = &'hook str>
                + ::hooks::core::HookBounds<Bounds = (::core::marker::PhantomData<str>,)>
    };

    assert_eq!(
        use_hooked::hook_args(),
        hooks_derive_core::HookArgs::default()
    );

    let mut hook = use_hooked::<str>();
    assert_eq!(
        std::mem::size_of_val(&hook),
        std::mem::size_of::<hooks::Memo<(), String>>(),
    );

    futures_lite::future::block_on(async {
        assert!(hook.next_update().await);
        assert_eq!(hook.use_hook(("aa",)), "aa");
        assert!(!hook.next_update().await);
    });

    futures_lite::future::block_on(async {
        let v = hook.next_value(("a",)).await;
        assert!(v.is_none());
    });
}
