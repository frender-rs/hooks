set -e

test_single_feature() {
  RUSTFLAGS="$RUSTFLAGS -A dead_code -A unused_imports" cargo test \
    -p hooks \
    --quiet \
    --no-default-features --features "proc-macro,futures-core,$1"
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
test_single_feature use_shared_ref
test_single_feature use_shared_ref,ShareValue
test_single_feature use_shared_signal
test_single_feature use_shared_signal,ShareValue
test_single_feature use_shared_signal,Signal
test_single_feature use_uninitialized_hook

test_single_feature use_state_with_updater
test_single_feature use_shared_update_state
test_single_feature use_shared_call
test_single_feature use_shared_reducer
test_single_feature use_shared_set
test_single_feature use_shared_toggle

test_single_feature use_gen_ref
test_single_feature use_gen_ref,ShareValue
test_single_feature use_gen_signal
test_single_feature use_gen_signal,ShareValue
test_single_feature use_gen_signal,Signal

cargo test -p hooks --no-default-features
cargo test -p hooks --no-default-features --features all
cargo test -p hooks --no-default-features --features all,proc-macro
cargo test -p hooks # all,futures-core
cargo test -p hooks --all-features
