# [1.0.0-alpha.8](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.7...hooks-v1.0.0-alpha.8) (2022-11-09)


### Features

* add `futures-core` feature which impl Stream for IterHook<H> ([94e52a1](https://github.com/frender-rs/hooks/commit/94e52a1725918643b7c521e752ca2c78748691d6))
* remove RunHook for simplicity, add NextValue and AsyncIterableHook ([aa7a7e5](https://github.com/frender-rs/hooks/commit/aa7a7e5076169dd7ac873545028ac591515bba40))


### BREAKING CHANGES

* RunHook and related methods are removed.

Please use `HookExt::next_value` and `AsyncIterableHook::into_iter` instead.

# [1.0.0-alpha.7](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.6...hooks-v1.0.0-alpha.7) (2022-11-08)


### Features

* `impl Hook for Pin<P> where P: DerefMut<impl Hook>` ([1e1733d](https://github.com/frender-rs/hooks/commit/1e1733dabe3b2aae330c0d15a119a589bd9618f6))

# [1.0.0-alpha.6](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.5...hooks-v1.0.0-alpha.6) (2022-11-08)


### Features

* impl Default and Debug for Ref and RefWith ([cca89d3](https://github.com/frender-rs/hooks/commit/cca89d3b48b9c11307e20c3c41190245ba0b456e))
* impl Hook for Pin<&mut H> ([7a27dfa](https://github.com/frender-rs/hooks/commit/7a27dfaed56856f784a1774073e27a1ac3a2e448))

# [1.0.0-alpha.5](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.4...hooks-v1.0.0-alpha.5) (2022-11-07)


### Bug Fixes

* **hooks:** `State` is not correctly polled ([8e25460](https://github.com/frender-rs/hooks/commit/8e25460877085a7c233c786c3e8645daa692e8d7))


### Features

* rewrite use_effect ([8a80385](https://github.com/frender-rs/hooks/commit/8a803852e6f74ae66d9c94287922a4b774b2b409))

# [1.0.0-alpha.4](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.3...hooks-v1.0.0-alpha.4) (2022-11-07)


### Bug Fixes

* add metadata to hooks-derive-core ([c1e7e8e](https://github.com/frender-rs/hooks/commit/c1e7e8e6f093d35f6fa7c97b6f268da91233b46b))

# [1.0.0-alpha.3](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.2...hooks-v1.0.0-alpha.3) (2022-11-06)


### Features

* default impl methods for ShareValue ([526b3a5](https://github.com/frender-rs/hooks/commit/526b3a5386ff2c407157bfa492f620ec7429ff27))
* use_lazy_pinned use_lazy_pinned_with ([e2b78b9](https://github.com/frender-rs/hooks/commit/e2b78b9faf6bb41e26eaa7ed7a9b31d47bb2d5e3))

# [1.0.0-alpha.2](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.1...hooks-v1.0.0-alpha.2) (2022-09-30)


### Features

* add package hooks-derive ([2adb69e](https://github.com/frender-rs/hooks/commit/2adb69e75ef3fa2bb135bed40ded7a235a32a422))

# 1.0.0-alpha.1 (2022-09-30)


### Features

* **hooks:** add README.md ([071633a](https://github.com/frender-rs/hooks/commit/071633a194dbbd69c0f248a4a994b3391352f20d))
