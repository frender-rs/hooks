#![cfg(all(feature = "futures-core", feature = "all"))]

use futures_lite::StreamExt;
use hooks::ShareValue;
use hooks_core::{prelude::*, HookExt};

mod utils;
use utils::test_many_async;

test_many_async!(use_1_hook(
    {
        hook_fn!(
            fn use_shared_state() -> u32 {
                let state = h![hooks::use_shared_state(3)];
                state.get()
            }
        );

        let values = use_shared_state()
            .into_hook_values()
            .collect::<Vec<_>>()
            .await;
        assert_eq!(values, [3]);
    },
    {
        hook_fn!(
            fn use_shared_state_borrowing<'a>(
                initial_value: &'a str,
            ) -> &'hook hooks::shared_state::SharedState<&'a str> {
                let state = h!(hooks::use_shared_state(initial_value));
                state
            }
        );

        let hello = "hello".to_owned();
        let msg = hello + " world!";
        let initial_value: &str = &msg;
        let mut hook = use_shared_state_borrowing(initial_value).into_hook();

        assert_eq!(hook.next_value().await.unwrap().get(), "hello world!");
        assert!(hook.next_value().await.is_none());
    },
    {
        hook_fn!(
            type Bounds = impl 'a;
            fn use_borrow_mut<'a, T: Default>(values: &'a mut Vec<T>) {
                values.push(T::default())
            }
        );

        let mut values = vec![];

        let results = use_borrow_mut::<String>(&mut values)
            .into_hook_values()
            .collect::<Vec<_>>()
            .await;

        assert_eq!(results, [()]);
        assert_eq!(values, [""]);
    },
));
