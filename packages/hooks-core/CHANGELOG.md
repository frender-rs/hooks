# [1.0.0-alpha.4](https://github.com/frender-rs/hooks/compare/hooks-core-v1.0.0-alpha.3...hooks-core-v1.0.0-alpha.4) (2022-11-09)


### Features

* add `futures-core` feature which impl Stream for IterHook<H> ([94e52a1](https://github.com/frender-rs/hooks/commit/94e52a1725918643b7c521e752ca2c78748691d6))
* remove RunHook for simplicity, add NextValue and AsyncIterableHook ([aa7a7e5](https://github.com/frender-rs/hooks/commit/aa7a7e5076169dd7ac873545028ac591515bba40))


### BREAKING CHANGES

* RunHook and related methods are removed.

Please use `HookExt::next_value` and `AsyncIterableHook::into_iter` instead.

# [1.0.0-alpha.3](https://github.com/frender-rs/hooks/compare/hooks-core-v1.0.0-alpha.2...hooks-core-v1.0.0-alpha.3) (2022-11-08)


### Features

* `impl Hook for Pin<P> where P: DerefMut<impl Hook>` ([1e1733d](https://github.com/frender-rs/hooks/commit/1e1733dabe3b2aae330c0d15a119a589bd9618f6))

# [1.0.0-alpha.2](https://github.com/frender-rs/hooks/compare/hooks-core-v1.0.0-alpha.1...hooks-core-v1.0.0-alpha.2) (2022-11-08)


### Features

* impl Hook for Pin<&mut H> ([7a27dfa](https://github.com/frender-rs/hooks/commit/7a27dfaed56856f784a1774073e27a1ac3a2e448))

# 1.0.0-alpha.1 (2022-11-06)


### Features

* add package hooks-derive ([2adb69e](https://github.com/frender-rs/hooks/commit/2adb69e75ef3fa2bb135bed40ded7a235a32a422))
