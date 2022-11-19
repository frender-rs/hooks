use std::future::Future;

use futures_lite::StreamExt;
use hooks::{
    hook, use_effect, use_shared_state, use_shared_state_eq, AsyncIterableHook, ShareValue,
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
        let mut hook = __hooks_hook_0;
        let __hooks_hook_0 = hook.as_mut();
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
            #[cfg(not(real_gat_fail))]
            let values = use_test().into_iter().collect::<Vec<_>, _>().await;
            #[cfg(real_gat_fail)]
            let values = use_test().into_iter().collect::<Vec<_>>().await;
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
            #[cfg(not(real_gat_fail))]
            let values = use_test().into_iter().collect::<Vec<_>, _>().await;
            #[cfg(real_gat_fail)]
            let values = use_test().into_iter().collect::<Vec<_>>().await;
            assert_eq!(values, [0, 1, 2]);
        }))
    });
}
