# Changelog

* The following workspace dependencies were updated
  * dependencies
    * hooks-core bumped from 2.0.0-alpha to 2.0.1-alpha

* The following workspace dependencies were updated
  * dependencies
    * hooks-core bumped from 2.0.1-alpha to 2.1.0-alpha

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
