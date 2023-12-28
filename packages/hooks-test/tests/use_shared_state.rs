use std::future::Future;

use futures_lite::StreamExt;
use hooks::{
    hook, use_effect, use_shared_state, use_shared_state_eq, HookExt, IntoHook, ShareValue,
};
use smol::Task;

#[test]
fn shared_state_delay() {
    thread_local! {
        static EXE: smol::LocalExecutor<'static> = smol::LocalExecutor::new();
    }

    fn spawn<T: 'static>(future: impl Future<Output = T> + 'static) -> Task<T> {
        EXE.with(|exe| exe.spawn(future))
    }

    #[hook]
    fn use_test() -> i32 {
        let state = use_shared_state(0);

        let value = state.get();
        let s = state.clone();

        use_effect(
            move |v: &_| {
                let v = *v;
                let task = spawn(async move {
                    smol::Timer::after(std::time::Duration::from_millis(300)).await;
                    if v < 2 {
                        s.set(v + 1);
                    }
                });

                // cancel the task when cleaning
                move || drop(task)
            },
            value,
        );

        value
    }

    EXE.with(|exe| {
        futures_lite::future::block_on(exe.run(async {
            let values = use_test().into_hook_values().collect::<Vec<_>>().await;
            assert_eq!(values, [0, 1, 2]);
        }))
    });
}

#[test]
fn shared_state_eq_delay() {
    thread_local! {
        static EXE: smol::LocalExecutor<'static> = smol::LocalExecutor::new();
    }

    fn spawn<T: 'static>(future: impl Future<Output = T> + 'static) -> Task<T> {
        EXE.with(|exe| exe.spawn(future))
    }

    #[hook]
    fn use_test() -> i32 {
        let state = use_shared_state_eq(0);

        let value = state.get();
        let s = state.clone();

        use_effect(
            move |v: &_| {
                let v = *v;
                let task = spawn(async move {
                    smol::Timer::after(std::time::Duration::from_millis(300)).await;
                    if v < 2 {
                        s.set(v + 1);
                    }
                });

                // cancel the task when cleaning
                move || drop(task)
            },
            value,
        );

        value
    }

    EXE.with(|exe| {
        futures_lite::future::block_on(exe.run(async {
            let values = use_test().into_hook_values().collect::<Vec<_>>().await;
            assert_eq!(values, [0, 1, 2]);
        }))
    });
}

async fn wait_millis(millis: u64) {
    smol::Timer::after(std::time::Duration::from_millis(millis)).await;
}

async fn assert_timeout(fut: impl Future, millis_timeout: u64) {
    let res = futures_lite::future::or(
        //
        async { Ok(fut.await) },
        async {
            wait_millis(millis_timeout).await;
            Err(())
        },
    )
    .await;
    assert!(res.is_err());
}

#[test]
fn different_tasks() {
    use hooks::shared_state::SharedState;

    let mut state = SharedState::new(0);

    let exe = smol::LocalExecutor::new();

    let task1 = exe.spawn({
        let mut state = state.clone();
        async move {
            let s = state.next_value().await.unwrap();
            assert_eq!(s.get(), 0);

            assert_timeout(state.next_value(), 100).await;

            let s = state.next_value().await.unwrap();
            assert_eq!(s.get(), 1);

            assert_timeout(state.next_value(), 100).await;
            let s = state.next_value().await;
            assert!(s.is_none());
        }
    });
    let task2 = exe.spawn({
        async move {
            let s = state.next_value().await.unwrap();
            assert_eq!(s.get(), 0);

            wait_millis(200).await;
            s.set(1);
            let s = state.next_value().await.unwrap();
            assert_eq!(s.get(), 1);

            wait_millis(200).await;
            drop(state)
        }
    });

    futures_lite::future::block_on(exe.run(futures_lite::future::or(task1, task2)))
}
