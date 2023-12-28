# Hooks

[![Crates.io](https://img.shields.io/crates/v/hooks?style=for-the-badge)](https://crates.io/crates/hooks)
[![docs.rs](https://img.shields.io/docsrs/hooks/latest?style=for-the-badge)](https://docs.rs/hooks)
[![GitHub license](https://img.shields.io/github/license/frender-rs/hooks?style=for-the-badge)](https://github.com/frender-rs/hooks/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/frender-rs/hooks?style=for-the-badge)](https://github.com/frender-rs/hooks/stargazers)

Compile-time, async hooks in safe Rust.

Please see docs on [docs.rs](https://docs.rs/hooks).

This project is still in alpha and may have breaking changes.
You can see the [changelogs](./packages/hooks/CHANGELOG.md) before updating.

## Development

### Add a new hook

If `hooks::use_my_hook` is to be added,

- Add a feature named `use_my_hook` in [hooks/Cargo.toml](packages/hooks/Cargo.toml)
  and add the feature to `all`.

- Add mod `my_hook` in package [hooks](packages/hooks/src/lib.rs).

- Prelude it in mod `hooks::prelude` under the feature.

- Add the feature to [tests](scripts/test-hooks.sh).
