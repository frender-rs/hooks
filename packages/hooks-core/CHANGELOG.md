# [1.0.0-alpha.9](https://github.com/frender-rs/hooks/compare/hooks-core-v1.0.0-alpha.8...hooks-core-v1.0.0-alpha.9) (2022-12-29)


### Features

* export impl_hook macro ([444477d](https://github.com/frender-rs/hooks/commit/444477deee1fdb23dabe169055fde6d4586a9aa6))

# [1.0.0-alpha.8](https://github.com/frender-rs/hooks/compare/hooks-core-v1.0.0-alpha.7...hooks-core-v1.0.0-alpha.8) (2022-11-23)


### Bug Fixes

* `HookPair` should be re-used if either hook of it should be re-used ([50682e9](https://github.com/frender-rs/hooks/commit/50682e973afd771ec799334fe1c256d5670fcf92))

# [1.0.0-alpha.7](https://github.com/frender-rs/hooks/compare/hooks-core-v1.0.0-alpha.6...hooks-core-v1.0.0-alpha.7) (2022-11-17)


### Features

* trait ErasedHook and dyn_hook macro ([7d41271](https://github.com/frender-rs/hooks/commit/7d4127130725e1eb347c05d6460a15dd59c89670))

# [1.0.0-alpha.6](https://github.com/frender-rs/hooks/compare/hooks-core-v1.0.0-alpha.5...hooks-core-v1.0.0-alpha.6) (2022-11-16)


### Features

* `::hooks_core::FnHook::new` ([5c3826e](https://github.com/frender-rs/hooks/commit/5c3826eee8d2bcc137952cc8a8a466b4c3d7d014))
* `impl Hook for Box<H: Unpin + Hook>` ([1d66ea1](https://github.com/frender-rs/hooks/commit/1d66ea166d89cd9ce7ca38fdc2ee5472fdd3f54d))

# [1.0.0-alpha.5](https://github.com/frender-rs/hooks/compare/hooks-core-v1.0.0-alpha.4...hooks-core-v1.0.0-alpha.5) (2022-11-09)


### Features

* impl HookPollNextUpdateExt for unsized types ([1c9b34a](https://github.com/frender-rs/hooks/commit/1c9b34a9614bbc3a185ad2aa41625dbc695bd998))


### BREAKING CHANGES

* Now unsized types that impl `HookPollNextUpdate` will auto impl `HookPollNextUpdateExt`.
(This was intended but omitted.)

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
