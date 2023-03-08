mod hook;
mod updater;
mod use_default;
mod use_hook;

pub use hook::*;
pub use updater::*;
pub use use_default::*;
pub use use_hook::*;

#[cfg(test)]
mod tests {
    use futures_lite::StreamExt;
    use hooks_core::{fn_hook, hook_fn, Hook, HookPollNextUpdateExt};
    use hooks_derive::hook;

    use super::{use_state, use_state_with, STAGING_STATES_DEFAULT_STACK_COUNT};

    #[test]
    fn state() {
        futures_lite::future::block_on(async {
            let hook = super::State::<_, 3, false>::new(1);
            futures_lite::pin!(hook);

            assert!(!std::future::poll_fn(|cx| hook.poll_next_update(cx)).await);

            let (state, updater) = hook.as_mut().use_hook();
            assert_eq!(*state, 1);
            updater.set(2);
            assert_eq!(*state, 1);

            assert!(std::future::poll_fn(|cx| hook.poll_next_update(cx)).await);
            let (state, _updater) = hook.as_mut().use_hook();
            assert_eq!(*state, 2);

            assert!(!std::future::poll_fn(|cx| hook.poll_next_update(cx)).await);
        });
    }

    #[cfg(feature = "use_effect")]
    #[test]
    fn hook_fn_state_2() {
        hook_fn!(
            fn use_state_2() -> (i32, i32) {
                let (state_1, updater_1) = h!(use_state(1));
                let (state_2, updater_2) = h!(use_state_with(|| *state_1 + 1));

                let ret = (*state_1, *state_2);

                let updater_1 = updater_1.clone();
                let updater_2 = updater_2.clone();
                h![crate::use_effect(
                    move |(v1, v2): &_| {
                        if *v2 > 10 {
                            return;
                        }
                        updater_1.set(*v2);
                        updater_2.set(*v1 + *v2);
                    },
                    ret,
                )];

                ret
            }
        );

        futures_lite::future::block_on(async {
            let values = use_state_2().into_iter().collect::<Vec<_>>().await;

            assert_eq!(values, [(1, 2), (2, 3), (3, 5), (5, 8), (8, 13)]);
        });
    }

    #[cfg(hook_macro)]
    #[test]
    fn state_2() {
        #[hook(hooks_core_path = "::hooks_core")]
        fn use_state_2() -> (i32, i32) {
            let (state_1, updater_1) = use_state(1);
            let (state_2, updater_2) = use_state_with(|| *state_1 + 1);

            let ret = (*state_1, *state_2);

            let updater_1 = updater_1.clone();
            let updater_2 = updater_2.clone();
            use_effect(
                move |(v1, v2): &_| {
                    if *v2 > 10 {
                        return;
                    }
                    updater_1.set(*v2);
                    updater_2.set(*v1 + *v2);
                },
                ret,
            );

            ret
        }

        futures_lite::future::block_on(async {
            let values = use_state_2().into_iter().collect::<Vec<_>>().await;

            assert_eq!(values, [(1, 2), (2, 3), (3, 5), (5, 8), (8, 13)]);
        });
    }
}
