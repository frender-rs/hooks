# [1.0.0-alpha.18](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.17...hooks-v1.0.0-alpha.18) (2022-11-23)


### Bug Fixes

* `HookPair` should be re-used if either hook of it should be re-used ([50682e9](https://github.com/frender-rs/hooks/commit/50682e973afd771ec799334fe1c256d5670fcf92))


### Features

* partially support `impl Trait` in return type of hook fn using hook macro ([61efdab](https://github.com/frender-rs/hooks/commit/61efdab8d0b46b2173cd7840fdbfb2da40db5d9f))

# [1.0.0-alpha.17](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.16...hooks-v1.0.0-alpha.17) (2022-11-21)


### Bug Fixes

* hook macro missing comma when using generic arguments ([0a0ec81](https://github.com/frender-rs/hooks/commit/0a0ec813d3846ebeb012b2404d11b92e3b9e10b3))


### Features

* hooks:memo_with helper fn ([6d1f7a2](https://github.com/frender-rs/hooks/commit/6d1f7a2b83983de4883b6c312ffa20d11055d3ed))

# [1.0.0-alpha.16](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.15...hooks-v1.0.0-alpha.16) (2022-11-19)


### Bug Fixes

* `SharedRef::is_shared` has wrong behavior ([35091e1](https://github.com/frender-rs/hooks/commit/35091e10b32a3c194087492788affde11f25a306))
* use_shared_state* has wrong behavior ([3bda58c](https://github.com/frender-rs/hooks/commit/3bda58c9c0fd22e155982aa49daeed3d21b75157))


### Features

* impl Debug and Clone for SharedStateEqData ([47b2d8a](https://github.com/frender-rs/hooks/commit/47b2d8aa2e478fc7448881ce40b14d7f343948f6))
* impl Debug and Default for use_shared_state related types ([c747989](https://github.com/frender-rs/hooks/commit/c747989e96c44fdd1f4d05664c7b9652128ea00f))

# [1.0.0-alpha.15](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.14...hooks-v1.0.0-alpha.15) (2022-11-17)


### Features

* trait ErasedHook and dyn_hook macro ([7d41271](https://github.com/frender-rs/hooks/commit/7d4127130725e1eb347c05d6460a15dd59c89670))

# [1.0.0-alpha.14](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.13...hooks-v1.0.0-alpha.14) (2022-11-16)


### Features

* hook macro now supports lifetime generics in args ([320639f](https://github.com/frender-rs/hooks/commit/320639fb0733eb2cd18d07032c2a15955443307b))

# [1.0.0-alpha.13](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.12...hooks-v1.0.0-alpha.13) (2022-11-16)


### Features

* `::hooks_core::FnHook::new` ([5c3826e](https://github.com/frender-rs/hooks/commit/5c3826eee8d2bcc137952cc8a8a466b4c3d7d014))
* `impl Hook for Box<H: Unpin + Hook>` ([1d66ea1](https://github.com/frender-rs/hooks/commit/1d66ea166d89cd9ce7ca38fdc2ee5472fdd3f54d))
* impl Debug and Default for use_state related structs ([afd9305](https://github.com/frender-rs/hooks/commit/afd9305a0eac181450e4e3b8e5105c2179686a74))
* use_state related type param doesn't require T: 'a ([8f13d09](https://github.com/frender-rs/hooks/commit/8f13d09342309096f2a75d7ccd6442ee1dc7e14c))

# [1.0.0-alpha.12](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.11...hooks-v1.0.0-alpha.12) (2022-11-12)


### Features

* use_default_pinned_hook now also accepts one argument inferring hook type ([b2d14c3](https://github.com/frender-rs/hooks/commit/b2d14c30c90eaaa89b908426a37818fb493ce835))
* use_memo ([caa71dd](https://github.com/frender-rs/hooks/commit/caa71ddf61ebe8be2764fb6f5cab4abf29e8ebcb))

# [1.0.0-alpha.11](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.10...hooks-v1.0.0-alpha.11) (2022-11-10)


### Features

* use_default_pinned ([f4dc7ef](https://github.com/frender-rs/hooks/commit/f4dc7ef5a239b6e777d89ddcdddd2d085faf9f5b))
* use_lazy_pinned_hook and use_default_pinned_hook ([8d5cc75](https://github.com/frender-rs/hooks/commit/8d5cc755afeb6ebb517880475cf7c9cff5cf8f8d))

# [1.0.0-alpha.10](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.9...hooks-v1.0.0-alpha.10) (2022-11-10)


### Bug Fixes

* hook macro not working when using two hooks ([71a68b6](https://github.com/frender-rs/hooks/commit/71a68b6a39144e8ef31e1f2f0821d777177626de))

# [1.0.0-alpha.9](https://github.com/frender-rs/hooks/compare/hooks-v1.0.0-alpha.8...hooks-v1.0.0-alpha.9) (2022-11-09)


### Features

* `use_hook_once` and `use_hook_once_with` ([fb8c448](https://github.com/frender-rs/hooks/commit/fb8c448a44238a004dd4fa8ff76683d5f1260aa9))
* impl HookPollNextUpdateExt for unsized types ([1c9b34a](https://github.com/frender-rs/hooks/commit/1c9b34a9614bbc3a185ad2aa41625dbc695bd998))
* rewrite use_effect ([f5e9bec](https://github.com/frender-rs/hooks/commit/f5e9bec0ddb0395468c4c0987b7cc94cc08988f4))


### BREAKING CHANGES

* - `use_effect`: before dependency is registered, its `poll_next_update` now returns `true`,
indicating `use_hook` should be called.
This used to return `false`.

- `effect_with_fn` is renamed to `get_new_dep_and_effect`
* Now unsized types that impl `HookPollNextUpdate` will auto impl `HookPollNextUpdateExt`.
(This was intended but omitted.)

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
