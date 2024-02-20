# Changelog

## [3.0.0-alpha.3](https://github.com/frender-rs/hooks/compare/hooks-v3.0.0-alpha.2...hooks-v3.0.0-alpha.3) (2024-02-20)


### Miscellaneous Chores

* **hooks:** fix changelog ([22413fb](https://github.com/frender-rs/hooks/commit/22413fbc7a48a2e66f19ccca103b5268652284e8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * hooks-macro bumped from 0.1.1 to 0.1.2
    * hooks-core bumped from 2.2.0-alpha to 3.0.0-alpha.1

## [3.0.0-alpha.2](https://github.com/frender-rs/hooks/compare/hooks-v3.0.0-alpha.1...hooks-v3.0.0-alpha.2) (2023-12-28)


### ⚠ BREAKING CHANGES

* **hooks:** feature `use_shared_state` should not enable feature `use_shared_ref`

### Features

* **hooks:** use_reused ([2cc4265](https://github.com/frender-rs/hooks/commit/2cc42657b17f6f238a6a9df4eb165cc340b871b5))


### Bug Fixes

* **hooks:** feature `use_shared_state` should not enable feature `use_shared_ref` ([069ce37](https://github.com/frender-rs/hooks/commit/069ce376ff4835e52357e5c4b35d27f8576062a1))

## [3.0.0-alpha.1](https://github.com/frender-rs/hooks/compare/hooks-v3.0.0-alpha...hooks-v3.0.0-alpha.1) (2023-12-28)


### Bug Fixes

* **hooks:** cloned `SharedState` and `SharedStateEq` should be responsive in different tasks ([62dafeb](https://github.com/frender-rs/hooks/commit/62dafebbca40faf2a3154d8bd4606f114572b67c))

## [3.0.0-alpha](https://github.com/frender-rs/hooks/compare/hooks-v2.3.0-alpha...hooks-v3.0.0-alpha) (2023-12-20)


### ⚠ BREAKING CHANGES

* **hooks:** `ShareValue::try_unwrap` and `ShareValue::unwrap_or_get_cloned`
* **hooks:** remove `ShareValue::is_shared` because it should always return `true` as designed

### Features

* **hooks:** `ShareValue::try_unwrap` and `ShareValue::unwrap_or_get_cloned` ([7666013](https://github.com/frender-rs/hooks/commit/7666013f85875f8b6ccc315d78a915983029dcfb))
* **hooks:** remove `ShareValue::is_shared` because it should always return `true` as designed ([1812068](https://github.com/frender-rs/hooks/commit/1812068aa529f0667a81523a747444395840e6a3))

## [2.3.0-alpha](https://github.com/frender-rs/hooks/compare/hooks-v2.2.0-alpha...hooks-v2.3.0-alpha) (2023-05-15)


### ⚠ BREAKING CHANGES

* **hooks:** add required method `equivalent_to` to trait `ShareValue`
* **hooks:** trait `ShareValue` is changed from `ShareValue<T>` to `ShareValue<Value = T>`

### Features

* **hooks:** add required method `equivalent_to` to trait `ShareValue` ([330e3b0](https://github.com/frender-rs/hooks/commit/330e3b04e0e6bcbdf28e2075b69d924d5bc2bf4f))
* **hooks:** trait `ShareValue` is changed from `ShareValue&lt;T&gt;` to `ShareValue<Value = T>` ([8442b91](https://github.com/frender-rs/hooks/commit/8442b919193924c329eb9ed1fd84c71e6fe814e9))

## [2.2.0-alpha](https://github.com/frender-rs/hooks/compare/hooks-v2.1.1-alpha...hooks-v2.2.0-alpha) (2023-04-15)


### Features

* impl `PartialEq` for StateUpdater ([c77e08c](https://github.com/frender-rs/hooks/commit/c77e08c3dc58ec6d91ac6b995f86530b72943dc1))

## [2.1.1-alpha](https://github.com/frender-rs/hooks/compare/hooks-v2.1.0-alpha...hooks-v2.1.1-alpha) (2023-04-13)


### Bug Fixes

* **hooks:** SharedState should be able to be dropped while sharing value ([86b60c1](https://github.com/frender-rs/hooks/commit/86b60c1ac7eba823b409a4a77b8c72f5bd3017cb))

## [2.1.0-alpha](https://github.com/frender-rs/hooks/compare/hooks-v2.0.2-alpha...hooks-v2.1.0-alpha) (2023-04-04)


### ⚠ BREAKING CHANGES

* output type of `LazyPinnedHook::pin_project` and `LazyPinned::pin_project` changed

### Features

* output type of `LazyPinnedHook::pin_project` and `LazyPinned::pin_project` changed ([05939f9](https://github.com/frender-rs/hooks/commit/05939f9206eac2e7fc020c1851be041b8572336d))


### Miscellaneous Chores

* forbid unsafe code in hooks ([391aadd](https://github.com/frender-rs/hooks/commit/391aadd1dfcb10854179855147d26e58202c736c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * hooks-macro bumped from 0.1.0 to 0.1.1
    * hooks-core bumped from 2.1.0-alpha to 2.2.0-alpha

## [2.0.0-alpha](https://github.com/frender-rs/hooks/compare/hooks-v1.0.1-alpha.21...hooks-v2.0.0-alpha) (2023-03-16)


### Features

* release v2.0.0-alpha ([b80d71e](https://github.com/frender-rs/hooks/commit/b80d71e8dd8aa80557a139b27094888b376f02a8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * hooks-core bumped from 1.0.0-alpha.10 to 2.0.0-alpha
