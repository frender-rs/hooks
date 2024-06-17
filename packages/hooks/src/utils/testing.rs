pub(crate) fn assert_timeout<Fut: std::future::Future>(
    get_fut: impl FnOnce() -> Fut,
    timeout: u64,
) {
    let res = futures_lite::future::block_on(async {
        let duration = std::time::Duration::from_millis(timeout);
        futures_lite::future::or(
            //
            async { Ok(get_fut().await) },
            async {
                async_io::Timer::after(duration).await;
                Err(())
            },
        )
        .await
    });

    assert!(res.is_err());
}

pub(crate) fn assert_always_pending<Fut: std::future::Future>(get_fut: impl FnOnce() -> Fut) {
    const TIMEOUT: u64 = 100;
    assert_timeout(get_fut, TIMEOUT)
}
