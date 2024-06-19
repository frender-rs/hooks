# Changelog

## [3.0.0-alpha.10](https://github.com/frender-rs/hooks/compare/hooks-v3.0.0-alpha.9...hooks-v3.0.0-alpha.10) (2024-06-19)


### ⚠ BREAKING CHANGES

* Signal::is_signal_of

### Features

* Signal::is_signal_of ([b72e0e7](https://github.com/frender-rs/hooks/commit/b72e0e79205d3d66c2cef88d059bd6a1b661775a))
* ToOwnedShareValue and ToOwnedSignal ([3c90b49](https://github.com/frender-rs/hooks/commit/3c90b49c85bbb5cd4c7841400ec0da3064c39e63))


### Bug Fixes

* better constraints for SignalHook ([4f5cee2](https://github.com/frender-rs/hooks/commit/4f5cee24c95093ea7c62540ced78d3d5d4b5b859))

## [3.0.0-alpha.9](https://github.com/frender-rs/hooks/compare/hooks-v3.0.0-alpha.8...hooks-v3.0.0-alpha.9) (2024-06-18)


### Features

* add some features ([33ef051](https://github.com/frender-rs/hooks/commit/33ef051d1bcea0713283e19a89cac12b397b322b))
* use_gen_update_state ([bf98b51](https://github.com/frender-rs/hooks/commit/bf98b5119dcec4ad62394ce469cbcfdc2a818194))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * hooks-gen bumped from 0.1.0 to 0.2.0

## [3.0.0-alpha.8](https://github.com/frender-rs/hooks/compare/hooks-v3.0.0-alpha.7...hooks-v3.0.0-alpha.8) (2024-06-17)


### ⚠ BREAKING CHANGES

* rename use_shared_state to use_shared_signal

### Features

* impl PartialEq for SharedSignal ([c02a223](https://github.com/frender-rs/hooks/commit/c02a223881abe760d1b3f6a8ae8430f44e505f9b))
* rename use_shared_state to use_shared_signal ([68cb5a2](https://github.com/frender-rs/hooks/commit/68cb5a29526be6fabc1048733c6f940fb5b1b14f))
* use_gen_signal and use_gen_ref ([9560467](https://github.com/frender-rs/hooks/commit/9560467b4966b512c1406265980d350a81ddedff))


### Miscellaneous Chores

* release ([eeba9f0](https://github.com/frender-rs/hooks/commit/eeba9f0b28b4469ae6919fb23c9907ed81629c7e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * hooks-core bumped from 3.0.0-alpha.3 to 3.0.0-alpha.4

## [3.0.0-alpha.7](https://github.com/frender-rs/hooks/compare/hooks-v3.0.0-alpha.6...hooks-v3.0.0-alpha.7) (2024-06-16)


### ⚠ BREAKING CHANGES

* reorganize exports
* now feature use_shared_ref and use_shared_state doesn't enable feature ShareValue
* remove use_reused
* remove macro `Value![]`
* IntoEq and remove use_shared_state_eq
* SignalHook::to_signal
* redesign Signal and SignalEq
* impl `ShareValue` for `&impl ShareValue`
* impl Signal for SharedState and SharedStateEq
* rename Signal::notify to notify_changed

### Features

* impl `ShareValue` for `&impl ShareValue` ([6f00ca0](https://github.com/frender-rs/hooks/commit/6f00ca0457463505fe6eeb4bd3ce237ace027b71))
* impl Signal for SharedState and SharedStateEq ([649024e](https://github.com/frender-rs/hooks/commit/649024ebca05f866565a74087c0083630595e334))
* IntoEq and remove use_shared_state_eq ([9c52bf3](https://github.com/frender-rs/hooks/commit/9c52bf3d7e96979cda19c1a6fec213bcb3ce8591))
* now feature use_shared_ref and use_shared_state doesn't enable feature ShareValue ([09841d6](https://github.com/frender-rs/hooks/commit/09841d69aed1feebd84fb5dfd1b1428b21946d8e))
* redesign Signal and SignalEq ([68e04d8](https://github.com/frender-rs/hooks/commit/68e04d80910af1d08226332d3b680710c572eea7))
* remove macro `Value![]` ([ba9b1ed](https://github.com/frender-rs/hooks/commit/ba9b1edac5eb4371dc66181d41a1e9d28a94d599))
* remove use_reused ([efc9de6](https://github.com/frender-rs/hooks/commit/efc9de6708ea503fc839b6638b6642d52b2ac9ee))
* rename Signal::notify to notify_changed ([02b5856](https://github.com/frender-rs/hooks/commit/02b5856e11f12ce1e4771b0368b46e88a9aae6c1))
* reorganize exports ([71b894f](https://github.com/frender-rs/hooks/commit/71b894f0e9bf4390f36f45d2fa66f190109aa1cc))
* Signal, SignalHook and use_signal ([893a380](https://github.com/frender-rs/hooks/commit/893a38083615a52369ef01751884d5d247cd0ae7))
* SignalHook::to_signal ([f1d3900](https://github.com/frender-rs/hooks/commit/f1d39000c603ff8b1ee581d293a18aed3e8a9147))
* type alias Value ([eae87a6](https://github.com/frender-rs/hooks/commit/eae87a687c82ece9a909c40a141bc0c2845e7577))


### Bug Fixes

* make SignalHookValue better ([72160bc](https://github.com/frender-rs/hooks/commit/72160bc39fa8e7ec5525ec58c7837d1f3e9aea08))


### Miscellaneous Chores

* release ([ebdfdaa](https://github.com/frender-rs/hooks/commit/ebdfdaa5d18b8a3cea514779fbba3c4e75ef5c1a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * hooks-core bumped from 3.0.0-alpha.2 to 3.0.0-alpha.3

## [3.0.0-alpha.6](https://github.com/frender-rs/hooks/compare/hooks-v3.0.0-alpha.5...hooks-v3.0.0-alpha.6) (2024-06-11)


### ⚠ BREAKING CHANGES

* remove use_state

### Features

* remove use_state ([a6039e2](https://github.com/frender-rs/hooks/commit/a6039e2b4dbee3dd278fb747758cb1e7bb204f33))
* use_state_with_updater ([e33d6b9](https://github.com/frender-rs/hooks/commit/e33d6b975879b3b646ffffe5422b83f13715fd90))


### Bug Fixes

* SharedUpdateState should wake when dropped ([36b18d1](https://github.com/frender-rs/hooks/commit/36b18d1b143e67994927c0cd15f43de309c38b69))


### Miscellaneous Chores

* release ([64aebe1](https://github.com/frender-rs/hooks/commit/64aebe11b6858df75fdbb73e94046a94236e691a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * hooks-core bumped from 3.0.0-alpha.1 to 3.0.0-alpha.2

## [3.0.0-alpha.5](https://github.com/frender-rs/hooks/compare/hooks-v3.0.0-alpha.4...hooks-v3.0.0-alpha.5) (2024-06-06)


### Features

* impl ReusableHook for SharedStateEq ([1d0ec20](https://github.com/frender-rs/hooks/commit/1d0ec20f1f7054a347673e6f9513bfefb53dd039))

## [3.0.0-alpha.4](https://github.com/frender-rs/hooks/compare/hooks-v3.0.0-alpha.3...hooks-v3.0.0-alpha.4) (2024-04-25)


### Miscellaneous Chores

* release ([f7aec7f](https://github.com/frender-rs/hooks/commit/f7aec7f96b8906a92e4ad7d52fec2ea920185185))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * hooks-macro bumped from 0.1.2 to 0.2.0

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
