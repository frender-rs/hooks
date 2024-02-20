# Changelog

## [0.3.0](https://github.com/frender-rs/hooks/compare/hooks-macro-core-v0.2.0...hooks-macro-core-v0.3.0) (2024-02-20)


### ⚠ BREAKING CHANGES

* now `UpdateHookUninitialized::Uninitialized` is an abstract type in `UpdateHookUninitialized!` and return type of `hook_fn!` and `#[hook_fn]`
* reimplement lifetime bounds in `#[hook]` with the Captures trick

### Features

* now `UpdateHookUninitialized::Uninitialized` is an abstract type in `UpdateHookUninitialized!` and return type of `hook_fn!` and `#[hook_fn]` ([79f9be9](https://github.com/frender-rs/hooks/commit/79f9be991390f46e82dfa28984cd46bca7a58f9c))
* reimplement lifetime bounds in `#[hook]` with the Captures trick ([981adb2](https://github.com/frender-rs/hooks/commit/981adb25ae6196d917e102e35fdc02bcb86297a8))


### Bug Fixes

* docs and doc tests ([25f5e27](https://github.com/frender-rs/hooks/commit/25f5e27e896ce1c76aa1bec91c7329eea629b805))

## [0.2.0](https://github.com/frender-rs/hooks/compare/hooks-macro-core-v0.1.0...hooks-macro-core-v0.2.0) (2023-04-04)


### Features

* use declarative macro to pin project `HookTuple` so that `hook` macro doesn't expand unsafe code ([391794a](https://github.com/frender-rs/hooks/commit/391794adb84f9498fb076646ef26d759fa3a1e30))

## 0.1.0 (2023-03-16)


### Features

* release v2.0.0-alpha ([b80d71e](https://github.com/frender-rs/hooks/commit/b80d71e8dd8aa80557a139b27094888b376f02a8))
