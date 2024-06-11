set -e

test_single_feature() {
  cargo test -p hooks --no-default-features --features "proc-macro,futures-core,$1"
}

test_single_feature ShareValue
test_single_feature use_debug
test_single_feature use_default_pinned
test_single_feature use_effect
test_single_feature use_lazy_pinned
test_single_feature use_lazy_pinned_hook
test_single_feature use_memo
test_single_feature use_mut
test_single_feature use_poll_next_update
test_single_feature use_reused
test_single_feature use_shared_ref
test_single_feature use_shared_state
test_single_feature use_uninitialized_hook

test_single_feature use_state_with_updater
test_single_feature use_shared_update_state
test_single_feature use_shared_call
test_single_feature use_shared_reducer
test_single_feature use_shared_set
test_single_feature use_shared_toggle

cargo test -p hooks --no-default-features
cargo test -p hooks --no-default-features --features all
cargo test -p hooks --no-default-features --features all,proc-macro
cargo test -p hooks # all,futures-core
cargo test -p hooks --all-features
