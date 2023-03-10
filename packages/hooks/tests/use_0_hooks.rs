use futures_lite::StreamExt;
use hooks_core::{prelude::*, HookExt};

mod utils;
use utils::test_many_async;

test_many_async!(use_zero_hooks(
    {
        hook_fn!(
            fn use_nothing() {}
        );

        let values = use_nothing().into_hook_values().collect::<Vec<_>>().await;
        assert_eq!(values, [()]);
    },
    {
        hook_fn!(
            fn use_nothing_but_returning() -> u32 {
                1
            }
        );

        let values = use_nothing_but_returning()
            .into_hook_values()
            .collect::<Vec<_>>()
            .await;
        assert_eq!(values, [1]);
    },
    {
        hook_fn!(
            fn use_nothing_but_lending() -> &'hook bool {
                &true
            }
        );

        let mut hook = use_nothing_but_lending().into_hook();

        assert_eq!(hook.next_value().await, Some(&true));
        assert!(hook.next_value().await.is_none());
    },
    {
        hook_fn!(
            fn use_nothing_with_arguments<'a>(v: &'a str) -> &'a str {
                v
            }
        );

        let mut hook = use_nothing_with_arguments("hi").into_hook();

        assert_eq!(hook.next_value().await, Some("hi"));
        assert!(hook.next_value().await.is_none());
    },
));
