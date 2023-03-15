#![cfg(feature = "all")]

use futures_lite::StreamExt;
use hooks::ShareValue;
use hooks_core::prelude::*;

mod utils;
use utils::test_many_async;

test_many_async!(use_2_hooks(
    {
        struct IncrementBoth<'a>([&'a hooks::shared_state::SharedState<u32>; 2]);

        impl IncrementBoth<'_> {
            pub fn increment_both(&self) {
                for state in self.0 {
                    state.replace_mut(|v| *v + 1);
                }
            }
        }

        hook_fn!(
            fn use_shared_state() -> (u32, IncrementBoth<'hook>) {
                let state_0 = h![hooks::use_shared_state(0)];
                let state_1 = h![hooks::use_shared_state(0)];

                (
                    state_0.get() + state_1.get(),
                    IncrementBoth([state_0, state_1]),
                )
            }
        );

        let mut hook = use_shared_state().into_hook();
        let (state, updater) = hook.next_value().await.unwrap();
        assert_eq!(state, 0);
        updater.increment_both();

        let (state, updater) = hook.next_value().await.unwrap();
        assert_eq!(state, 2);

        updater.increment_both();
        updater.increment_both();

        let (state, _) = hook.next_value().await.unwrap();
        assert_eq!(state, 6);

        // state is not updated, thus no more new values
        assert!(hook.next_value().await.is_none());
    },
    {
        use std::rc::Rc;

        hook_fn!(
            fn use_lazy_to_string<T: ToString + PartialEq + Copy>(value: T) -> Rc<String> {
                let (rc, _) = h!(hooks::use_memo(|v| Rc::new(v.to_string()), value));
                rc.clone()
            }
        );

        hook_fn!(
            fn use_strings() -> Rc<String> {
                let s = h!(hooks::use_mut_default::<Option<_>>());
                let values = s.get_or_insert_with(|| [0, 0, 1, 1].into_iter());

                let value = values.next().expect(
                    "use_strings cannot be used after poll_next_update returned Poll::Ready(false)",
                );

                h!({
                    let should_update = values.len() > 0;
                    hooks::use_poll_next_update(move |_| should_update.into())
                });

                h!(use_lazy_to_string(value))
            }
        );

        let strings = use_strings().into_hook_values().collect::<Vec<_>>().await;
        assert_eq!(strings.len(), 4);
        assert!(Rc::ptr_eq(&strings[0], &strings[1]));
        assert!(Rc::ptr_eq(&strings[2], &strings[3]));
        assert_eq!(
            strings.iter().map(|v| -> &str { v }).collect::<Vec<_>>(),
            ["0", "0", "1", "1"]
        );
    },
));
