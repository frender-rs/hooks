# Changelog
<!-- Touch this file so that the release-please would know this commit only affects this package -->

## [3.0.0-alpha.1](https://github.com/frender-rs/hooks/compare/hooks-core-v2.2.0-alpha...hooks-core-v3.0.0-alpha.1) (2024-02-20)


### ⚠ BREAKING CHANGES

* now `UpdateHookUninitialized::Uninitialized` is an abstract type in `UpdateHookUninitialized!` and return type of `hook_fn!` and `#[hook_fn]`
* reimplement lifetime bounds with the Captures trick

### Features

* now `UpdateHookUninitialized::Uninitialized` is an abstract type in `UpdateHookUninitialized!` and return type of `hook_fn!` and `#[hook_fn]` ([79f9be9](https://github.com/frender-rs/hooks/commit/79f9be991390f46e82dfa28984cd46bca7a58f9c))
* reimplement lifetime bounds with the Captures trick ([d8982be](https://github.com/frender-rs/hooks/commit/d8982be7cd2a0654e763930032952a70d53b8b5e))


### Bug Fixes

* docs and doc tests ([25f5e27](https://github.com/frender-rs/hooks/commit/25f5e27e896ce1c76aa1bec91c7329eea629b805))


### Miscellaneous Chores

* **hooks-core:** release ([ff815b3](https://github.com/frender-rs/hooks/commit/ff815b3f7878a42746546b7d7e064740180934ba))

## [2.2.0-alpha](https://github.com/frender-rs/hooks/compare/hooks-core-v2.1.0-alpha...hooks-core-v2.2.0-alpha) (2023-04-04)


### Features

* use declarative macro to pin project `HookTuple` so that `hook` macro doesn't expand unsafe code ([391794a](https://github.com/frender-rs/hooks/commit/391794adb84f9498fb076646ef26d759fa3a1e30))

## [2.1.0-alpha](https://github.com/frender-rs/hooks/compare/hooks-core-v2.0.1-alpha...hooks-core-v2.1.0-alpha) (2023-03-18)


### Features

* impl parsing in macros with `syn-lite`. ([0471573](https://github.com/frender-rs/hooks/commit/04715738fd4f7de69687ed1c723e42c5b00b4c0c))

## [2.0.1-alpha](https://github.com/frender-rs/hooks/compare/hooks-core-v2.0.0-alpha...hooks-core-v2.0.1-alpha) (2023-03-17)


### Bug Fixes

* remove fn_hook from hooks_core::prelude ([fa8df5c](https://github.com/frender-rs/hooks/commit/fa8df5ccda1c480a9616fe3b1790653ab1f5fcba))

## [2.0.0-alpha](https://github.com/frender-rs/hooks/compare/hooks-core-v1.0.0-alpha.10...hooks-core-v2.0.0-alpha) (2023-03-16)


### Features

* release v2.0.0-alpha ([b80d71e](https://github.com/frender-rs/hooks/commit/b80d71e8dd8aa80557a139b27094888b376f02a8))
