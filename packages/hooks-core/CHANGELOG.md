# Changelog

## [3.0.0-alpha.3](https://github.com/frender-rs/hooks/compare/hooks-core-v3.0.0-alpha.2...hooks-core-v3.0.0-alpha.3) (2024-06-16)


### ⚠ BREAKING CHANGES

* remove macro `Value![]`

### Features

* export HookValueBounds and it's still sealed ([e6b0519](https://github.com/frender-rs/hooks/commit/e6b0519f7d43c159eec3123fb4196a8c7117016b))
* remove macro `Value![]` ([ba9b1ed](https://github.com/frender-rs/hooks/commit/ba9b1edac5eb4371dc66181d41a1e9d28a94d599))
* type alias Value ([eae87a6](https://github.com/frender-rs/hooks/commit/eae87a687c82ece9a909c40a141bc0c2845e7577))


### Miscellaneous Chores

* release ([69d178a](https://github.com/frender-rs/hooks/commit/69d178a454ba4da75581590a65da86527d149140))

## [3.0.0-alpha.2](https://github.com/frender-rs/hooks/compare/hooks-core-v3.0.0-alpha.1...hooks-core-v3.0.0-alpha.2) (2024-06-11)


### ⚠ BREAKING CHANGES

* remove use_state

### Features

* remove use_state ([a6039e2](https://github.com/frender-rs/hooks/commit/a6039e2b4dbee3dd278fb747758cb1e7bb204f33))


### Miscellaneous Chores

* release ([3cc30d6](https://github.com/frender-rs/hooks/commit/3cc30d61943aff45b5687827e58bd8547eda27e8))

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
