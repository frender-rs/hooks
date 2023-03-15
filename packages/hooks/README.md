# `::hooks`

[![Crates.io](https://img.shields.io/crates/v/hooks?style=for-the-badge)](https://crates.io/crates/hooks)
[![docs.rs](https://img.shields.io/docsrs/hooks/latest?style=for-the-badge)](https://docs.rs/hooks)
[![GitHub license](https://img.shields.io/github/license/frender-rs/hooks?style=for-the-badge)](https://github.com/frender-rs/hooks/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/frender-rs/hooks?style=for-the-badge)](https://github.com/frender-rs/hooks/stargazers)

Compile-time, async hooks in safe Rust.

## Quick Start

Run `cargo add hooks` to add `hooks` to your project.

Note that this project is still in alpha,
and it may have BREAKING CHANGES.

Please see [changelogs](https://github.com/frender-rs/hooks/blob/alpha/packages/hooks/CHANGELOG.md) before upgrading.

You can compose hooks in a hook fn with [`hook_fn!(...)`](crate::hook_fn) or [`#[hook]`](crate::hook)

### With `hook_fn!(...)`

```rust
use hooks::prelude::*;

hook_fn!(
    fn use_demo() {
        let (state, updater) = h![hooks::use_state(0)];

        let updater = updater.clone();

        h![hooks::use_effect(move |v: &i32| {
            println!("state = {}", *v);

            if *v < 2 {
              updater.set(*v + 1);
            }
        }, *state)];
    }
);

fn main() {
    futures_lite::future::block_on(async {
        let mut hook = use_demo().into_hook();
        while let Some(()) = hook.next_value().await {}
    });
}
```

### With `#[hook]`

This attribute macro is only available under `proc-macro` feature.
Enable it with `cargo add -p hook --features proc-macro`.
`#[hook]` allows using hooks without `h!()`.
Any function call or method call with fn name starting with `use_`
is automatically detected as a hook.

```rust
use hooks::prelude::*;

#[hook]
fn use_demo() {
    let (state, updater) = hooks::use_state(0);

    let updater = updater.clone();

    hooks::use_effect(move |v: &i32| {
        println!("state = {}", *v);

        if *v < 2 {
          updater.set(*v + 1);
        }
    }, *state);
}

fn main() {
    futures_lite::future::block_on(async {
        let mut hook = use_demo().into_hook();
        while let Some(()) = hook.next_value().await {}
    });
}
```

You will see the following logs. Then the program exits gracefully because
it knows there won't be new values.

```txt
state = 0
state = 1
state = 2
```

## What is a compile-time `Hook`?

<details>
<summary>
To understand the concepts behind this crate,
you can expand this section and read the details.
</summary>

Hooks, introduced by [React 16.8](https://reactjs.org/docs/hooks-intro.html),
is a way to bring _state_ into functional components.
Hooks can make stateless functional components stateful, and reactive.

Conventional hook implementations use a global state to record hook calls and their states.
This way, a stateless function can maintain its state through runtime contexts.
Thus, the order of hook calls must not change; conditional hook calls are also forbidden.
Developers must follow [`Rules of Hooks`](https://reactjs.org/docs/hooks-rules.html)
to write a valid custom hook.
[`yew.rs`](https://yew.rs/docs/concepts/function-components/custom-hooks) also passes
[hook context](https://docs.rs/yew/0.20/yew/functional/struct.HookContext.html)s
to used hooks.
We can see the above implementation relies on runtime behavior of a hook fn.
The hook runner must run the function once to know what is initialized.
We call this _runtime hooks_.

Rust language has powerful static type systems.
In fact, the state of a hook function is statically typed.
The hard problem is to make the stateless function stateful,
which means its state should also be known by the executor.
We call this kind of hook implementation as _compile-time hooks_.

This crate defines and implements _compile-time hooks_ for you.

When a type implements [`Hook`], it defines three behaviors:

1. When using this hook, what does it output?

   [`Hook::use_hook`] returns [`HookValue::Value`].

   This crate uses _GAT (Generic Associated Types)_ to allow the output type borrowing from the hook itself.
   Due to some limitations of real GAT, this crate uses
   [_better GAT_ pattern](https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats) introduced by _Sabrina Jewson_. Thanks to her!

2. When should we re-use this hook?

   Hooks have states. When the state doesn't change, we don't need to re-call `use_hook` to get the new output.
   We can wait for the hook's state to change with [`HookPollNextUpdate::poll_next_update`],
   or by just [`hook.next_update().await`](HookPollNextUpdateExt::next_update).

   To wait for the next value when state changes,
   you can use [`hook.next_value().await`] method.

</details>

## How to write a custom hook?

Please see [`Hook`](trait@Hook#how-to-impl-hook) trait.

### How to use hooks in a hook function?

With [`hook_fn!`] macro, you can just use `h![use_another_hook(arg0, arg1)]` at
_top level token trees_ (not wrapped in token trees like `()`, `[]`, or `{}`).
The macro will transform the call.

With [`#[hook]`](crate::hook) macro, you can just call `use_another_hook(arg0, arg1)` at
_top level expressions_ (not in an inner block like `{}`).
The macro will transform the call.
You can see the [snapshots](https://github.com/frender-rs/hooks/blob/alpha/packages/hooks-test/tests/snapshots) for what this macro outputs.

### How to conditionally use hooks?

Please see [`use_lazy_pinned_hook`] and [`use_uninitialized_hook`].

## How to use the hook when not in a hook fn

A hook fn actually returns [`impl UpdateHookUninitialized`](trait@UpdateHookUninitialized).
To consume it, you can run [`use_my_hook().into_hook()`](IntoHook::into_hook) to turn it into a [`Hook`],
or run [`use_my_hook().into_hook_values()`](IntoHook::into_hook_values)
(which runs `use_my_hook().into_hook().into_values()`) to get async iterated values.

To consume a [`Hook`], you can use its next value with [`hook.next_value().await`](HookExt::next_value).
You can get async iterated values with [`hook.values()`](HookExt::values) or [`hook.into_values()`](HookExt::into_values),
which is a [`Stream`] if the hook is [`NonLendingHook`].

```rust
# use hooks::prelude::*;
hook_fn!(
    fn use_demo() -> i32 {
        let (state, updater) = h![use_state(0)];

        let updater = updater.clone();
        h![hooks::use_effect(move |_: &i32| {
            updater.replace_maybe_with_fn_pointer(
                |v| if *v < 2 { Some(*v + 1) } else { None }
            );
        }, *state)];

        *state
    }
);

// with hook.next_value().await
futures_lite::future::block_on(async {
    let mut hook = use_demo().into_hook();
    assert_eq!(hook.next_value().await, Some(0));
    assert_eq!(hook.next_value().await, Some(1));
    assert_eq!(hook.next_value().await, Some(2));
    assert_eq!(hook.next_value().await, None);
});

// with hook.into_hook_values() and stream.next().await
futures_lite::future::block_on(async {
    use futures_lite::StreamExt;

    let mut values = use_demo().into_hook_values();
    assert_eq!(values.next().await, Some(0));
    assert_eq!(values.next().await, Some(1));
    assert_eq!(values.next().await, Some(2));
    assert_eq!(values.next().await, None);
});

// with hook.into_hook_values() and stream.collect().await
futures_lite::future::block_on(async {
    use futures_lite::StreamExt;

    let values = use_demo().into_hook_values();
    let values = values.collect::<Vec<_>>().await;
    assert_eq!(values, [0, 1, 2]);
});
```

[`hook.next_value().await`]: HookExt::next_value
[`hook`]: trait@Hook
[`stream`]: https://docs.rs/futures-core/0.3/futures_core/stream/trait.Stream.html
