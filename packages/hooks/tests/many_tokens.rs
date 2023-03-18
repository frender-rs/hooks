#![cfg(feature = "use_state")]

use hooks::prelude::*;

hook_fn!(
    fn use_max_tokens_rust_analyzer() -> i32 {
        let (state, _) = h!(use_state(0));
        let value = *state
            +1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1
            +1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1
            // + 1 // TEST: rust-analyzer should works fine unless uncomment this
            + 1;
        *h!(use_state(value)).0
    }
);

hook_fn!(
    fn use_max_tokens_recursion_limit() -> i32 {
        let (state, _) = h!(use_state(0));
        let value = *state
            +1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1
            +1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1
            +1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1
            +1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1
            +1+1+1+1+1+1+1+1+1+1+1+1+1+1
            // + 1 // TEST: if uncomment this, recursion limit is reached
            + 1;
        *h!(use_state(value)).0
    }
);

#[test]
fn tests() {
    futures_lite::future::block_on(async {
        let mut hook = use_max_tokens_rust_analyzer().into_hook();
        assert_eq!(hook.next_value().await, Some(85));
        assert_eq!(hook.next_value().await, None);
    });

    futures_lite::future::block_on(async {
        let mut hook = use_max_tokens_recursion_limit().into_hook();
        assert_eq!(hook.next_value().await, Some(215));
        assert_eq!(hook.next_value().await, None);
    });
}
