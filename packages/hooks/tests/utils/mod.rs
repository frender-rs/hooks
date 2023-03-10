macro_rules! test_many_async {
    ( $name:ident ( $($blocks:tt),+ $(,)? ) ) => {
        #[test]
        fn $name() {
            ::futures_lite::future::block_on(
                test_many_async!(
                    @ $($blocks)+
                )
            );
        }
    };
    (@ $block0:tt $($blocks:tt)+) => {
        ::futures_lite::future::zip(
            async $block0,
            test_many_async!(
                @ $($blocks)+
            )
        )
    };
    (@ $block0:tt) => {
        async $block0
    };
}

pub(crate) use test_many_async;
