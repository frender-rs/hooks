use std::future::Future;

use futures_lite::stream::StreamExt;
use hooks::{
    hook, hook_fn, reused::ReusableHookExt, shared_state::SharedState, HookExt, IntoHook,
    ShareValue,
};

hook_fn!(
    fn use_test(state: SharedState<i32>) -> i32 {
        let state = h![state.use_reused()];

        state.get()
    }
);

hook_fn!(
    type Bounds = impl '_;
    fn use_test_1(state: &SharedState<i32>) -> i32 {
        let state = h![state.use_reused()];

        state.get()
    }
);

#[hook(bounds = "'_")]
fn use_tests(state: &SharedState<i32>) -> i32 {
    let v = use_test_1(state);
    let state = state.use_reused();

    v + state.get()
}

const TIMEOUT: std::time::Duration = std::time::Duration::new(1, 0);

async fn assert_timeout(fut: impl Future) {
    let res = futures_lite::future::or(async { Ok(fut.await) }, async {
        smol::Timer::after(TIMEOUT).await;
        Err(())
    })
    .await;
    assert!(res.is_err());
}

#[test]
fn reuse_shared_state() {
    futures_lite::future::block_on(async {
        let state = SharedState::new(0);
        let mut values = use_test(state.clone()).into_hook_values();
        drop(state);
        assert_eq!(values.next().await, Some(0));

        let res = futures_lite::future::or(async { Ok(values.next().await) }, async {
            smol::Timer::after(TIMEOUT).await;
            Err(())
        })
        .await;
        assert!(res.is_err());
    })
}

#[test]
fn clone_shared_state() {
    futures_lite::future::block_on(async {
        let mut state_1 = SharedState::new(0);
        let mut state_2 = state_1.clone();

        if let Some(state) = state_1.next_value().await {
            assert!(state.equivalent_to(&state_2));
        } else {
            panic!()
        }

        if let Some(state) = state_2.next_value().await {
            assert!(state.equivalent_to(&state_1));
        } else {
            panic!()
        }

        assert_timeout(state_1.next_value()).await;

        {
            state_2.set(0);

            if let Some(state) = state_2.next_value().await {
                assert!(state.equivalent_to(&state_1));
            } else {
                panic!()
            }

            assert_timeout(state_2.next_value()).await;

            if let Some(state) = state_1.next_value().await {
                assert!(state.equivalent_to(&state_2));
            } else {
                panic!()
            }
        }

        {
            state_2.set(0);

            if let Some(state) = state_1.next_value().await {
                assert!(state.equivalent_to(&state_2));
            } else {
                panic!()
            }

            if let Some(state) = state_2.next_value().await {
                assert!(state.equivalent_to(&state_1));
            } else {
                panic!()
            }

            assert_timeout(state_2.next_value()).await;
        }
    })
}

#[test]
fn reuse_shared_state_2() {
    futures_lite::future::block_on(async {
        let state = SharedState::new(0);
        let mut values_1 = use_test(state.clone()).into_hook_values();
        let mut values_2 = use_test(state.clone()).into_hook_values();

        assert_eq!(values_1.next().await, Some(0));

        // This is because fn_hook will at least run once
        // and cloned SharedState is marked as not seen.
        assert_eq!(values_2.next().await, Some(0));

        assert_timeout(values_1.next()).await;
        assert_timeout(values_2.next()).await;

        // After update, only the first awaited state will be ready
        {
            state.set(0);
            assert_eq!(values_1.next().await, Some(0));
            assert_eq!(values_2.next().await, Some(0));

            assert_timeout(values_1.next()).await;
            assert_timeout(values_2.next()).await;
        }
    })
}

#[test]
fn tests() {
    futures_lite::future::block_on(async {
        let state = SharedState::new(0);
        let values = use_tests(&state).into_hook_values();

        futures_lite::pin!(values);

        assert_eq!(values.next().await, Some(0));

        state.set(1);
        assert_eq!(values.next().await, Some(2));

        {
            let res = futures_lite::future::or(async { Ok(values.next().await) }, async {
                smol::Timer::after(TIMEOUT).await;
                Err(())
            })
            .await;
            assert!(res.is_err());
        }
    })
}
