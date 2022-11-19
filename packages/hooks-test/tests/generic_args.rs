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
                + for<'hook, 'a> ::hooks::core::Hook<(&'a (),), Value<'hook> = ()>
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
                + for<'hook, 'a> ::hooks::core::Hook<(&'a (),), Value<'hook> = ()>
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
